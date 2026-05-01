/// Imports
use crate::{
    builtins::utils,
    callable, class, native_class, native_fun, native_method, realm,
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
            match values.first().unwrap() {
                Value::Int(time) => {
                    if *time >= 0 {
                        thread::sleep(Duration::from_millis(*time as u64));
                        Value::Null
                    } else {
                        utils::error(span, "time expected to be >= 0")
                    }
                }
                _ => utils::error(span, "time expected to be an int"),
            }
        }
    }
}

/// Process exit
fn exit() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(code) => {
                    if *code >= 0 {
                        if *code <= i32::MAX as i64 {
                            process::exit(*code as i32)
                        } else {
                            utils::error(span, "exit code is too large")
                        }
                    } else {
                        utils::error(span, "exit code expected to be >= 0")
                    }
                }
                _ => utils::error(span, "exit code expected to be int"),
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
            let cmd = match values.first().cloned().unwrap() {
                Value::String(s) => s,
                _ => utils::error(span, "corrupted command"),
            };

            // Retrieving args
            let args = {
                let args = match values.get(1).cloned().unwrap() {
                    Value::Instance(instance) => instance,
                    _ => utils::error(span, "corrupted args"),
                };

                // Safety: borrow is temporal for this line
                let internal = args.borrow_mut().fields.get("$internal").cloned().unwrap();

                match internal {
                    // Safety: borrow is temporal, value will be cloned
                    Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                        Some(vec) => vec.clone(),
                        _ => utils::error(span, "corrupted args"),
                    },
                    _ => {
                        utils::error(span, "corrupted args");
                    }
                }
            };

            // Generating command
            let mut cmd = Command::new(cmd);
            cmd.args(args.iter().map(|a| a.to_string()));

            // Spawning process
            let child = match cmd.spawn() {
                Ok(child) => child,
                Err(err) => utils::error(span, &format!("failed to span process: {err}")),
            };

            // Searching `Process` class
            let process_class = match rt.builtins.modules.get("process") {
                // Safety: borrow is temporal for the end of function
                Some(module) => match module.borrow().env.borrow().lookup("Process") {
                    Some(Value::Class(ty)) => ty,
                    _ => utils::error(span, "corrupted module"),
                },
                None => utils::error(span, "corrupted module"),
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
    match value {
        Value::Instance(instance) => {
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
                    _ => utils::error(span, "corrupted process"),
                },
                _ => {
                    utils::error(span, "corrupted process");
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates process argument
fn validate_process_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(&mut Child) -> V,
{
    validate_process(span, values.first().cloned().unwrap(), f)
}

/// `Process` init method
fn process_init_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, _, values| {
            let list = values.first().cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    // Setting `$internal` field
                    instance
                        .borrow_mut()
                        .fields
                        .insert("$internal".to_string(), values.get(1).cloned().unwrap());

                    Value::Null
                }
                _ => unreachable!(),
            }
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
                        match stdin.write_all(values.get(1).unwrap().to_string().as_bytes()) {
                            Ok(_) => {}
                            Err(err) => {
                                utils::error(span, &format!("failed to write into stdin: {err:?}"))
                            }
                        }
                    }
                    None => utils::error(span, "failed to retrieve `stdin`"),
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
