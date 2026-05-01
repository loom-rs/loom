/// Native function macros
#[macro_export]
macro_rules! native_fun {
    (
        arity = $arity:expr,
        fun = |_, $span:ident, $values:ident| $body:block
    ) => {
        Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, $span, $values| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |_, _, _| $body:block
    ) => {
        Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, _, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |_, $span:ident, _| $body:block
    ) => {
        Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, $span, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |$rt:ident, $span:ident, _| $body:block
    ) => {
        Ref::new(Native {
            arity: $arity,
            function: Box::new(|$rt, $span, _| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |_, _, $values:ident| $body:block
    ) => {
        Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, _, $values| $body),
        })
    };
    (
        arity = $arity:expr,
        fun = |$rt:ident, $span:ident, $values:ident| $body:block
    ) => {
        Ref::new(Native {
            arity: $arity,
            function: Box::new(|$rt, $span, $values| $body),
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
        Method::Native(Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, $span, $values| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |_, _, _| $body:block
    ) => {
        Method::Native(Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, _, _| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |_, $span:ident, _| $body:block
    ) => {
        Method::Native(Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, $span, _| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |$rt:ident, $span:ident, _| $body:block
    ) => {
        Method::Native(Ref::new(Native {
            arity: $arity,
            function: Box::new(|$rt, $span, _| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |_, _, $values:ident| $body:block
    ) => {
        Method::Native(Ref::new(Native {
            arity: $arity,
            function: Box::new(|_, _, $values| $body),
        }))
    };
    (
        arity = $arity:expr,
        fun = |$rt:ident, $span:ident, $values:ident| $body:block
    ) => {
        Method::Native(Ref::new(Native {
            arity: $arity,
            function: Box::new(|$rt, $span, $values| $body),
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
        std::rc::Rc::new(Class {
            name: stringify!($class_name).to_string(),
            methods: HashMap::from([
                $(
                    ((stringify!($method_name)).to_string(), $fun),
                )*
            ])
        })
    }};
}

/// Realm macros
#[macro_export]
macro_rules! realm {
    (
        $($name:ident => $val:expr),* $(,)?
    ) => {{
        let mut realm = Realm::default();

        $(
            realm.define(
                stringify!($name),
                $val
            );
        )*

        std::rc::Rc::new(std::cell::RefCell::new(realm))
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
                std::rc::Rc::new(std::cell::RefCell::new(Module {
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
        $crate::rt::value::Value::Callable($crate::rt::value::Callable::Native($callable))
    };
}

/// Class macros
#[macro_export]
macro_rules! class {
    ($callable:expr) => {
        $crate::rt::value::Value::Class($callable)
    };
}

/// Error macros
#[macro_export]
macro_rules! error {
    ($span:expr, $text:expr $(,)?) => {
        geko_common::bail!($crate::errors::RuntimeError::Bail {
            text: $text.to_string(),
            src: $span.0.clone(),
            span: $span.1.clone().into()
        })
    };
}

/// Builtin class macros
#[macro_export]
macro_rules! builtin_class {
    ($rt:expr, $name:expr) => {{
        let value = $rt
            .builtins
            .env
            .borrow()
            .lookup($name)
            .unwrap_or_else(|| bug!(format!("no builtin `{}` found", $name)));

        match value {
            Value::Class(class) => class,
            _ => bug!(format!("buitlin `{}` is not a class", $name)),
        }
    }};
}
