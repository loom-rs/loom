/// Native function macros
#[macro_export]
macro_rules! native_fun {
    (
        arity = $arity:expr,
        fun = |_, $span:ident, $values:ident| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, $span, $values| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |_, _, _| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, _, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |_, $span:ident, _| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, $span, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |$vm:ident, $span:ident, _| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|$vm, $span, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |$vm:ident, _, _| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|$vm, _, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |_, _, $values:ident| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, _, $values| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |$vm:ident, $span:ident, $values:ident| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|$vm, $span, $values| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |$vm:ident, _, $values:ident| $body:block
    ) => {
        dune::refs::Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|$vm, _, $values| $body),
        })
    };
}

/// Native method macros
#[macro_export]
macro_rules! native_method {
    (
        arity = $arity:expr,
        fun = |_, $span:ident, $values:ident| $body:block
    ) => {
        dune::value::Method::Native(Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, $span, $values| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |_, _, _| $body:block
    ) => {
        dune::value::Method::Native(Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, _, _| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |_, $span:ident, _| $body:block
    ) => {
        dune::value::Method::Native(Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, $span, _| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |$vm:ident, $span:ident, _| $body:block
    ) => {
        dune::value::Method::Native(Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|$vm, $span, _| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |_, _, $values:ident| $body:block
    ) => {
        dune::value::Method::Native(Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|_, _, $values| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |$vm:ident, $span:ident, $values:ident| $body:block
    ) => {
        dune::value::Method::Native(Ref::new(dune::value::Native {
            arity: $arity,
            function: Box::new(|$vm, $span, $values| $body),
        }))
    };
}

/// Native class macros
#[macro_export]
macro_rules! native_class {
    (
        name = $class_name:ident,
        methods = { $($method_name:ident => $fun:expr),* $(,)? }
    ) => {{
        dune::refs::Ref::new(Class {
            name: stringify!($class_name).to_string(),
            methods: HashMap::from([
                $(
                    ((stringify!($method_name)).to_string(), $fun),
                )*
            ])
        })
    }};
}

/// Scope macros
#[macro_export]
macro_rules! scope {
    (
        $($name:ident => $val:expr),* $(,)?
    ) => {{
        let mut scope = dune::frame::Scope::default();

        $(
            scope.insert(
                stringify!($name),
                $val
            );
        )*

        dune::refs::MutRef::new(std::cell::RefCell::new(scope))
    }};
}

/// Modules macros
#[macro_export]
macro_rules! modules {
    (
        $($name:ident),* $(,)?
    ) => {{
        HashMap::from([
        $(
            (
                stringify!($name).to_string(),
                dune::refs::MutRef::new(std::cell::RefCell::new(Module {
                    env: $name::provide_env(),
                }))
            ),
        )*
        ])
    }};
}

/// Callable macros
#[macro_export]
macro_rules! callable {
    ($callable:expr) => {
        dune::value::Value::Callable(dune::value::Callable::Native($callable))
    };
}

/// Class macros
#[macro_export]
macro_rules! class {
    ($callable:expr) => {
        dune::value::Value::Class($callable)
    };
}

/// Error macros
#[macro_export]
macro_rules! error {
    ($span:expr, $text:expr $(,)?) => {
        common::bail!(dune::errors::RuntimeError::Bail {
            text: $text.to_string(),
            src: $span.0.clone(),
            span: $span.1.clone().into()
        })
    };
}

/// Builtin class macros
#[macro_export]
macro_rules! builtin_class {
    ($vm:expr, $name:expr) => {{
        let value = $vm
            .builtins
            .borrow()
            .lookup($name)
            .unwrap_or_else(|| common::bug!(format!("no builtin `{}` found", $name)));

        match value {
            dune::value::Value::Class(class) => class,
            _ => common::bug!(format!("buitlin `{}` is not a class", $name)),
        }
    }};
}

/// Builtin module class macros
#[macro_export]
macro_rules! builtin_module_class {
    ($vm:expr, $mod:expr, $class:expr) => {
        match $vm.builtins.modules.get($mod) {
            // Safety: borrow is temporal for the end of function
            Some(module) => match module.borrow().env.borrow().lookup($class) {
                Some(Value::Class(class)) => class,
                _ => bug!(format!("no builtin `{}` found in `{}`", $class, $mod)),
            },
            None => bug!(format!("no builtin module `{}` found", $mod)),
        }
    };
}

/// Expect macros
#[macro_export]
macro_rules! expect {
    ($span:expr, $value:expr, $pat:path) => {
        match $value {
            $pat(value) => value,
            _ => {
                let full = stringify!($pat);
                let name = full.split("::").last().unwrap().to_lowercase();
                $crate::error!($span, format!("value `{}` expected to be `{name}`", $value));
            }
        }
    };
}
/// Expect as macros
#[macro_export]
macro_rules! expect_as {
    ($span:expr, $value:expr, $pat:path, $name:expr) => {
        match $value {
            $pat(value) => value,
            _ => {
                $crate::error!(
                    $span,
                    format!("value `{}` expected to be `{}`", $value, $name)
                );
            }
        }
    };
}

/// Arg macros
#[macro_export]
macro_rules! arg {
    ($values:expr, $idx:expr) => {
        $values.get($idx).unwrap().clone()
    };
}

/// Arg reference macros
#[macro_export]
macro_rules! arg_ref {
    ($values:expr, $idx:expr) => {
        $values.get($idx).unwrap()
    };
}

/// To string value macros
#[macro_export]
macro_rules! to_string_value {
    ($span:expr, $vm:expr, $value:expr) => {
        if let Value::Instance(i) = $value {
            // Note: borrow is temporal for this line
            match { i.borrow().fields.get("to_string").cloned() } {
                Some($dune::value::Value::Callable(callable)) => {
                    match $vm.call($span, callable.clone(), vec![]) {
                        Ok(value) => value,
                        Err(_) => geko_common::bug!("control flow leak"),
                    }
                }
                _ => $dune::value::Value::Instance(i.clone()),
            }
        } else {
            $value.clone()
        }
    };
}

/// To string macros
#[macro_export]
macro_rules! to_string {
    ($span:expr, $vm:expr, $value:expr) => {
        if let Value::Instance(i) = $value {
            // Note: borrow is temporal for this line
            match { i.borrow().fields.get("to_string").cloned() } {
                Some(dune::value::Value::Callable(callable)) => {
                    match $vm.call($span, callable.clone(), vec![]) {
                        Ok(value) => format!("{value:?}"),
                        Err(_) => geko_common::bug!("control flow leak"),
                    }
                }
                _ => format!("{:?}", $dune::value::Value::Instance(i.clone())),
            }
        } else {
            format!("{:?}", $value)
        }
    };
}
