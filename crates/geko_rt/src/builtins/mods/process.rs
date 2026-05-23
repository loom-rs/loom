/// Imports
use crate::{
    arg, arg_ref, callable, class, error, expect, expect_as, native_class, native_fun,
    native_method, realm,
    refs::{MutRef, RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Class, Method, Native, Value},
    },
};
use geko_common::bug;
use geko_lex::token::Span;
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    process::{self, Child, Command},
    thread,
    time::Duration,
};

/// Thread sleep
fn sleep() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let time = expect!(span, arg!(values, 0), Value::Int);

            if time >= 0 {
                thread::sleep(Duration::from_millis(time as u64));
                Value::Null
            } else {
                error!(span, "time expected to be a positive int")
            }
        }
    }
}

/// Process exit
fn exit() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let exit_code = expect_as!(span, arg!(values, 0), Value::Int, "exit code");

            if exit_code >= 0 {
                if exit_code <= i32::MAX as i64 {
                    process::exit(exit_code as i32)
                } else {
                    error!(span, "exit code is too large")
                }
            } else {
                error!(span, "exit code expected to be a positive int")
            }
        }
    }
}

/// Process spawn
fn spawn() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |rt, span, values| {
            // Retrieving command
            let cmd = match arg!(values, 0) {
                Value::String(s) => s,
                _ => error!(span, "corrupted command"),
            };

            // Retrieving args
            let args = {
                let args = match arg_ref!(values, 1) {
                    Value::Instance(instance) => instance,
                    _ => error!(span, "corrupted args"),
                };

                // Safety: borrow is temporal for this line
                let internal = args.borrow_mut().fields.get("$internal").cloned().unwrap();

                match internal {
                    // Safety: borrow is temporal, value will be cloned
                    Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                        Some(vec) => vec.clone(),
                        _ => error!(span, "corrupted args"),
                    },
                    _ => {
                        error!(span, "corrupted args");
                    }
                }
            };

            // Generating command
            let mut cmd = Command::new(cmd);
            cmd.args(args.iter().map(|a| a.to_string()));

            // Spawning process
            let child = match cmd.spawn() {
                Ok(child) => child,
                Err(err) => error!(span, &format!("failed to span process: {err}")),
            };

            // Searching `Process` class
            let process_class = match rt.builtins.modules.get("process") {
                // Safety: borrow is temporal for the end of function
                Some(module) => match module.borrow().env.borrow().lookup("Process") {
                    Some(Value::Class(ty)) => ty,
                    _ => error!(span, "corrupted module"),
                },
                None => error!(span, "corrupted module"),
            };

            // Creating `Process` instance
            match rt.call_class(
                span,
                vec![Value::Any(MutRef::new(RefCell::new(child)))],
                process_class,
            ) {
                Ok(val) => val,
                Err(_) => bug!("control flow leak"),
            }
        }
    }
}

/// Helper: validates process
fn validate_process<F, V>(span: &Span, value: Value, f: F) -> V
where
    F: FnOnce(&mut Child) -> V,
{
    let instance = expect!(span, value, Value::Instance);

    // Safety: borrow is temporal for this line
    let internal = instance
        .borrow_mut()
        .fields
        .get("$internal")
        .cloned()
        .unwrap();

    match internal {
        // Safety: borrow is temporal and short
        Value::Any(process) => match process.borrow_mut().downcast_mut::<Child>() {
            Some(child) => f(child),
            _ => error!(span, "corrupted process"),
        },
        _ => {
            error!(span, "corrupted process");
        }
    }
}

/// Helper: validates process argument
fn validate_process_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(&mut Child) -> V,
{
    validate_process(span, arg!(values, 0), f)
}

/// `Process` init method
fn process_init_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            let instance = expect!(span, arg!(values, 0), Value::Instance);

            // Setting `$internal` field
            instance
                .borrow_mut()
                .fields
                .insert("$internal".to_string(), arg!(values, 1));

            Value::Null
        }
    }
}

/// `Process` pid method
fn process_pid_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_process_arg(span, &values, |child| Value::Int(child.id() as i64))
        }
    }
}

/// `Process` kill method
fn process_kill_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_process_arg(span, &values, |child| {
                _ = child.kill();
                Value::Null
            })
        }
    }
}

/// `Process` output method
fn process_output_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_process_arg(span, &values, |child| {
                let output = match &mut child.stdout {
                    Some(stdout) => {
                        let mut output = String::new();
                        let _ = stdout.read_to_string(&mut output);
                        output
                    }
                    None => "<failed to retrieve `stdout`>".to_string(),
                };
                Value::String(output)
            })
        }
    }
}

/// `Process` stderr method
fn process_stderr_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_process_arg(span, &values, |child| {
                let output = match &mut child.stderr {
                    Some(stderr) => {
                        let mut output = String::new();
                        let _ = stderr.read_to_string(&mut output);
                        output
                    }
                    None => "<failed to retrieve `stderr`>".to_string(),
                };
                Value::String(output)
            })
        }
    }
}

/// `Process` write method
fn process_write_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_process_arg(span, &values, |child| {
                match &mut child.stdin {
                    Some(stdin) => {
                        match stdin.write_all(arg_ref!(values, 1).to_string().as_bytes()) {
                            Ok(_) => {}
                            Err(err) => {
                                error!(span, &format!("failed to write into stdin: {err:?}"))
                            }
                        }
                    }
                    None => error!(span, "failed to retrieve `stdin`"),
                };
                Value::Null
            })
        }
    }
}

/// Provides `Process` class
fn provide_process_class() -> Ref<Class> {
    native_class! {
        name = Process,
        methods = {
            init => process_init_method(),
            pid => process_pid_method(),
            kill => process_kill_method(),
            output => process_output_method(),
            stderr => process_stderr_method(),
            write => process_write_method()
        }
    }
}

/// Provides `process` module env
pub fn provide_env() -> RealmRef {
    realm! {
        sleep => callable!(sleep()),
        exit => callable!(exit()),
        spawn => callable!(spawn()),
        pid => Value::Int(process::id() as i64),
        Process => class!(provide_process_class())
    }
}
