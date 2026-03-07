/// Imports
use crate::{
    builtins::utils,
    refs::{EnvRef, MutRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Method, Native, Type, Value},
    },
};
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    process::{self, Child, Command},
    rc::Rc,
    thread,
    time::Duration,
};
use tick_common::bug;

/// Thread sleep
fn sleep() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(time) => {
                if *time >= 0 {
                    thread::sleep(Duration::from_millis(*time as u64));
                    Value::Null
                } else {
                    utils::error(span, "time expected to be >= 0")
                }
            }
            _ => utils::error(span, "time expected to be an int"),
        }),
    });
}

/// Process exit
fn exit() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
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
        }),
    });
}

/// Process spawn
fn spawn() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            // Retrieving command
            let cmd = match values.get(0).cloned().unwrap() {
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

            // Searching `Process` type
            let process_ty = match rt.builtins.modules.get("process") {
                // Safety: borrow is temporal for the end of function
                Some(module) => match module.borrow().env.borrow().lookup("Process") {
                    Some(ty) => match ty {
                        Value::Type(ty) => ty,
                        _ => utils::error(span, "corrupted module"),
                    },
                    _ => utils::error(span, "corrupted module"),
                },
                None => utils::error(span, "corrupted module"),
            };

            // Creating `Process` instance
            match rt.call_type(
                span,
                vec![Value::Any(MutRef::new(RefCell::new(child)))],
                process_ty,
            ) {
                Ok(val) => val,
                Err(_) => bug!("control flow leak"),
            }
        }),
    });
}

/// `Process` init method
fn process_init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, _, values| {
            let list = values.get(0).cloned().unwrap();
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
        }),
    }))
}

/// `Process` pid method
fn process_pid_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
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
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Child>() {
                            Some(child) => Value::Int(child.id() as i64),
                            _ => utils::error(span, "corrupted process"),
                        },
                        _ => {
                            utils::error(span, "corrupted process");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// `Process` kill method
fn process_kill_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let process = values.get(0).cloned().unwrap();
            match process {
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
                            Some(child) => {
                                _ = child.kill();
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted process"),
                        },
                        _ => {
                            utils::error(span, "corrupted process");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// `Process` output method
fn process_output_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    // Safety: borrow is temporal for this line
                    let internal = instance
                        .borrow_mut()
                        .fields
                        .get("$internal")
                        .cloned()
                        .unwrap();

                    match internal {
                        // Safety: borrow is temporal
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Child>() {
                            Some(child) => {
                                let output = match &mut child.stdout {
                                    Some(stdout) => {
                                        let mut output = String::new();
                                        let _ = stdout.read_to_string(&mut output);
                                        output
                                    }
                                    None => "<failed to retrieve `stdout`>".to_string(),
                                };
                                Value::String(output)
                            }
                            _ => utils::error(span, "corrupted process"),
                        },
                        _ => {
                            utils::error(span, "corrupted process");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// `Process` stderr method
fn process_stderr_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
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
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Child>() {
                            Some(child) => {
                                let output = match &mut child.stderr {
                                    Some(stderr) => {
                                        let mut output = String::new();
                                        let _ = stderr.read_to_string(&mut output);
                                        output
                                    }
                                    None => "<failed to retrieve `stderr`>".to_string(),
                                };
                                Value::String(output)
                            }
                            _ => utils::error(span, "corrupted process"),
                        },
                        _ => {
                            utils::error(span, "corrupted process");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// `Process` write method
fn process_write_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
            match list {
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
                        Value::Any(list) => match list.borrow_mut().downcast_mut::<Child>() {
                            Some(child) => {
                                match &mut child.stdin {
                                    Some(stdin) => {
                                        match stdin.write_all(
                                            format!("{}", values.get(1).cloned().unwrap())
                                                .as_bytes(),
                                        ) {
                                            Ok(_) => {}
                                            Err(err) => utils::error(
                                                span,
                                                &format!("failed to write into stdin: {err:?}"),
                                            ),
                                        }
                                    }
                                    None => utils::error(span, "failed to retrieve `stdin`"),
                                };
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted process"),
                        },
                        _ => {
                            utils::error(span, "corrupted process");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Provides `Process` type
fn provide_process_type() -> Ref<Type> {
    Ref::new(Type {
        name: "Process".to_string(),
        methods: HashMap::from([
            // Init method
            ("init".to_string(), process_init_method()),
            // Pid method
            ("pid".to_string(), process_pid_method()),
            // Kill method
            ("kill".to_string(), process_kill_method()),
            // Output method
            ("output".to_string(), process_output_method()),
            // Stderr method
            ("stderr".to_string(), process_stderr_method()),
            // Write method
            ("write".to_string(), process_write_method()),
        ]),
    })
}

/// Provides `process` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("sleep", Value::Callable(Callable::Native(sleep())));
    env.force_define("exit", Value::Callable(Callable::Native(exit())));
    env.force_define("spawn", Value::Callable(Callable::Native(spawn())));
    env.force_define("Process", Value::Type(provide_process_type()));

    Rc::new(RefCell::new(env))
}
