/// Imports
use crate::{
    error::RuntimeError,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use tick_common::bail;
use std::{
    cell::RefCell,
    f64::consts::{
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, LN_2,
        LN_10, LOG2_10, LOG2_E, LOG10_2, LOG10_E, PI, SQRT_2, TAU,
    },
    rc::Rc,
};

/// Math sin
fn sin() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::sin(*int as f64)),
            Value::Float(float) => Value::Float(f64::sin(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math sinh
fn sinh() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::sinh(*int as f64)),
            Value::Float(float) => Value::Float(f64::sinh(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math cos
fn cos() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::cos(*int as f64)),
            Value::Float(float) => Value::Float(f64::cos(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math cosh
fn cosh() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::cosh(*int as f64)),
            Value::Float(float) => Value::Float(f64::cosh(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math asin
fn asin() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::asin(*int as f64)),
            Value::Float(float) => Value::Float(f64::asin(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math asinh
fn asinh() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::asinh(*int as f64)),
            Value::Float(float) => Value::Float(f64::asinh(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math acos
fn acos() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::acos(*int as f64)),
            Value::Float(float) => Value::Float(f64::acos(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math acosh
fn acosh() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::acosh(*int as f64)),
            Value::Float(float) => Value::Float(f64::acosh(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math atan
fn atg() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::atan(*int as f64)),
            Value::Float(float) => Value::Float(f64::atan(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math atan 2
fn atg2() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            match (values.get(0).unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Float(f64::atan2(*y as f64, *x as f64)),
                (Value::Int(x), Value::Float(y)) => Value::Float(f64::atan2(*y, *x as f64)),
                (Value::Float(x), Value::Int(y)) => Value::Float(f64::atan2(*y as f64, *x)),
                (Value::Float(x), Value::Float(y)) => Value::Float(f64::atan2(*y, *x)),
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            }
        }),
    });
}

/// Math tg
fn tg() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::tan(*int as f64)),
            Value::Float(float) => Value::Float(f64::tan(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math tgh
fn tgh() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::tanh(*int as f64)),
            Value::Float(float) => Value::Float(f64::tanh(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

// Math ctg
fn ctg() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(1.0 / f64::tan(*int as f64)),
            Value::Float(float) => Value::Float(1.0 / f64::tan(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

// Math ctgh
fn ctgh() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(1.0 / f64::tanh(*int as f64)),
            Value::Float(float) => Value::Float(1.0 / f64::tanh(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math sqrt
fn sqrt() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::sqrt(*int as f64)),
            Value::Float(float) => Value::Float(f64::sqrt(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math cbrt
fn cbrt() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::cbrt(*int as f64)),
            Value::Float(float) => Value::Float(f64::cbrt(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math log
fn log() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let a = match values.get(0).unwrap() {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            };
            let b = match values.get(1).unwrap() {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            };
            Value::Float(a.log(b))
        }),
    });
}

/// Math min
fn min() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            // Int min
            Value::Int(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Int(*a.min(b)),
                Value::Float(b) => Value::Float((*a as f64).min(*b)),
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            // Float min
            Value::Float(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Float(a.min(*b as f64)),
                Value::Float(b) => Value::Float(a.min(*b)),
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math max
fn max() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            // Int max
            Value::Int(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Int(*a.max(b)),
                Value::Float(b) => Value::Float((*a as f64).max(*b)),
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            // Float max
            Value::Float(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Float(a.max(*b as f64)),
                Value::Float(b) => Value::Float(a.max(*b)),
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math clamp
fn clamp() -> Ref<Native> {
    return Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            // Int clamp
            Value::Int(x) => match (values.get(1).unwrap(), values.get(2).unwrap()) {
                (Value::Int(a), Value::Int(b)) => {
                    if a > b {
                        bail!(RuntimeError::Bail {
                            text: "clamp: min must be <= max".to_string(),
                            src: span.0.clone(),
                            span: span.1.clone().into()
                        })
                    }
                    Value::Int(*x.clamp(a, b))
                }
                (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                    let (min, max) = (*a as f64, *b);
                    if min > max {
                        bail!(RuntimeError::Bail {
                            text: "clamp: min must be <= max".to_string(),
                            src: span.0.clone(),
                            span: span.1.clone().into()
                        })
                    }
                    Value::Float((*x as f64).clamp(min, max))
                }
                (Value::Float(a), Value::Float(b)) => {
                    if a > b {
                        bail!(RuntimeError::Bail {
                            text: "clamp: min must be <= max".to_string(),
                            src: span.0.clone(),
                            span: span.1.clone().into()
                        })
                    }
                    Value::Float((*x as f64).clamp(*a, *b))
                }
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            // Float clamp
            Value::Float(x) => match (values.get(1).unwrap(), values.get(2).unwrap()) {
                (Value::Int(a), Value::Int(b)) => {
                    if a > b {
                        bail!(RuntimeError::Bail {
                            text: "clamp: min must be <= max".to_string(),
                            src: span.0.clone(),
                            span: span.1.clone().into()
                        })
                    }
                    Value::Float(x.clamp(*a as f64, *b as f64))
                }
                (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                    let (min, max) = (*a as f64, *b);
                    if min > max {
                        bail!(RuntimeError::Bail {
                            text: "clamp: min must be <= max".to_string(),
                            src: span.0.clone(),
                            span: span.1.clone().into()
                        })
                    }
                    Value::Float(x.clamp(min, max))
                }
                (Value::Float(a), Value::Float(b)) => {
                    if a > b {
                        bail!(RuntimeError::Bail {
                            text: "clamp: min must be <= max".to_string(),
                            src: span.0.clone(),
                            span: span.1.clone().into()
                        })
                    }
                    Value::Float(x.clamp(*a, *b))
                }
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math log2
fn log2() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::log2(*int as f64)),
            Value::Float(float) => Value::Float(f64::log2(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math log10
fn log10() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::log10(*int as f64)),
            Value::Float(float) => Value::Float(f64::log10(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math exp
fn exp() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => Value::Float(f64::exp(*int as f64)),
            Value::Float(float) => Value::Float(f64::exp(*float)),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math abs
fn abs() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Int(int) => match int.checked_abs() {
                Some(result) => Value::Int(result),
                None => bail!(RuntimeError::Bail {
                    text: "int overflow in abs".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            Value::Float(float) => Value::Float(float.abs()),
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math floor
fn floor() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Float(float) => Value::Float(float.floor()),
            _ => bail!(RuntimeError::Bail {
                text: "not a float".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math ceil
fn ceil() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Float(float) => Value::Float(float.ceil()),
            _ => bail!(RuntimeError::Bail {
                text: "not a float".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math trunc
fn trunc() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Float(float) => Value::Float(float.trunc()),
            _ => bail!(RuntimeError::Bail {
                text: "not a float".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math round
fn round() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            Value::Float(float) => Value::Float(float.round()),
            _ => bail!(RuntimeError::Bail {
                text: "not a float".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math pow
fn pow() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.get(0).unwrap() {
            // Int pow
            Value::Int(a) => match values.get(1).unwrap() {
                // Int exp
                Value::Int(b) => {
                    use std::convert::TryInto;

                    // Positive exponent
                    if *b >= 0 {
                        // Safe cast
                        let b_u32: u32 = (*b).try_into().unwrap_or_else(|_| {
                            bail!(RuntimeError::Bail {
                                text: format!("exponent {} is too large", b),
                                src: span.0.clone(),
                                span: span.1.clone().into(),
                            })
                        });

                        match a.checked_pow(b_u32) {
                            Some(result) => Value::Int(result),
                            None => bail!(RuntimeError::Bail {
                                text: "int overflow in exp".to_string(),
                                src: span.0.clone(),
                                span: span.1.clone().into()
                            }),
                        }
                    }
                    // Negative exponent
                    else {
                        // Safe cast
                        let b_i32: i32 = (*b).try_into().unwrap_or_else(|_| {
                            bail!(RuntimeError::Bail {
                                text: format!("exponent {} is too small", b),
                                src: span.0.clone(),
                                span: span.1.clone().into(),
                            })
                        });

                        Value::Float((*a as f64).powi(b_i32))
                    }
                }
                // Float exp
                Value::Float(b) => Value::Float((*a as f64).powf(*b)),
                // Otherwise, raising error
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            // Float pow
            Value::Float(a) => match values.get(1).unwrap() {
                // Int exp
                Value::Int(b) => Value::Float(a.powi(*b as i32)),
                // Float exp
                Value::Float(b) => Value::Float(a.powf(*b)),
                // Otherwise, raising error
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            },
            _ => bail!(RuntimeError::Bail {
                text: "not a number".to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }),
    });
}

/// Math hypot
fn hypot() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            match (values.get(0).unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Float(f64::hypot(*x as f64, *y as f64)),
                (Value::Int(x), Value::Float(y)) => Value::Float(f64::hypot(*x as f64, *y)),
                (Value::Float(x), Value::Int(y)) => Value::Float(f64::hypot(*x, *y as f64)),
                (Value::Float(x), Value::Float(y)) => Value::Float(f64::hypot(*x, *y)),
                _ => bail!(RuntimeError::Bail {
                    text: "not a number".to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            }
        }),
    });
}

/// Provides `math` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("log", Value::Callable(Callable::Native(log())));
    env.force_define("log2", Value::Callable(Callable::Native(log2())));
    env.force_define("log10", Value::Callable(Callable::Native(log10())));
    env.force_define("exp", Value::Callable(Callable::Native(exp())));
    env.force_define("abs", Value::Callable(Callable::Native(abs())));
    env.force_define("floor", Value::Callable(Callable::Native(floor())));
    env.force_define("ceil", Value::Callable(Callable::Native(ceil())));
    env.force_define("trunc", Value::Callable(Callable::Native(trunc())));
    env.force_define("round", Value::Callable(Callable::Native(round())));
    env.force_define("sin", Value::Callable(Callable::Native(sin())));
    env.force_define("cos", Value::Callable(Callable::Native(cos())));
    env.force_define("asin", Value::Callable(Callable::Native(asin())));
    env.force_define("acos", Value::Callable(Callable::Native(acos())));
    env.force_define("sinh", Value::Callable(Callable::Native(sinh())));
    env.force_define("cosh", Value::Callable(Callable::Native(cosh())));
    env.force_define("asinh", Value::Callable(Callable::Native(asinh())));
    env.force_define("acosh", Value::Callable(Callable::Native(acosh())));
    env.force_define("tg", Value::Callable(Callable::Native(tg())));
    env.force_define("tgh", Value::Callable(Callable::Native(tgh())));
    env.force_define("ctg", Value::Callable(Callable::Native(ctg())));
    env.force_define("ctgh", Value::Callable(Callable::Native(ctgh())));
    env.force_define("atg", Value::Callable(Callable::Native(atg())));
    env.force_define("atg2", Value::Callable(Callable::Native(atg2())));
    env.force_define("sqrt", Value::Callable(Callable::Native(sqrt())));
    env.force_define("cbrt", Value::Callable(Callable::Native(cbrt())));
    env.force_define("pow", Value::Callable(Callable::Native(pow())));
    env.force_define("hypot", Value::Callable(Callable::Native(hypot())));
    env.force_define("min", Value::Callable(Callable::Native(min())));
    env.force_define("max", Value::Callable(Callable::Native(max())));
    env.force_define("clamp", Value::Callable(Callable::Native(clamp())));
    env.force_define("pi", Value::Float(PI));
    env.force_define("tau", Value::Float(TAU));
    env.force_define("frac_pi_2", Value::Float(FRAC_PI_2));
    env.force_define("frac_pi_3", Value::Float(FRAC_PI_3));
    env.force_define("frac_pi_4", Value::Float(FRAC_PI_4));
    env.force_define("frac_pi_6", Value::Float(FRAC_PI_6));
    env.force_define("frac_pi_8", Value::Float(FRAC_PI_8));
    env.force_define("frac_1_pi", Value::Float(FRAC_1_PI));
    env.force_define("frac_1_sqrt_2", Value::Float(FRAC_1_SQRT_2));
    env.force_define("sqrt_2", Value::Float(SQRT_2));
    env.force_define("ln2", Value::Float(LN_2));
    env.force_define("ln10", Value::Float(LN_10));
    env.force_define("log10_e", Value::Float(LOG10_E));
    env.force_define("log10_2", Value::Float(LOG10_2));
    env.force_define("log2_e", Value::Float(LOG2_E));
    env.force_define("log2_10", Value::Float(LOG2_10));
    env.force_define("e", Value::Float(E));

    Rc::new(RefCell::new(env))
}
