/// Imports
use crate::{
    builtins::utils,
    callable, native_fun, realm,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};
use rand::RngExt;

/// Math sin
fn sin() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::sin(*int as f64)),
                Value::Float(float) => Value::Float(f64::sin(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math sinh
fn sinh() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::sinh(*int as f64)),
                Value::Float(float) => Value::Float(f64::sinh(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math cos
fn cos() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::cos(*int as f64)),
                Value::Float(float) => Value::Float(f64::cos(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math cosh
fn cosh() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::cosh(*int as f64)),
                Value::Float(float) => Value::Float(f64::cosh(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math asin
fn asin() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::asin(*int as f64)),
                Value::Float(float) => Value::Float(f64::asin(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math asinh
fn asinh() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::asin(*int as f64)),
                Value::Float(float) => Value::Float(f64::asin(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math acos
fn acos() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::acos(*int as f64)),
                Value::Float(float) => Value::Float(f64::acos(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math acosh
fn acosh() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::acosh(*int as f64)),
                Value::Float(float) => Value::Float(f64::acosh(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math atan
fn atg() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::atan(*int as f64)),
                Value::Float(float) => Value::Float(f64::atan(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math atan 2
fn atg2() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            match (values.first().unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Float(f64::atan2(*y as f64, *x as f64)),
                (Value::Int(x), Value::Float(y)) => Value::Float(f64::atan2(*y, *x as f64)),
                (Value::Float(x), Value::Int(y)) => Value::Float(f64::atan2(*y as f64, *x)),
                (Value::Float(x), Value::Float(y)) => Value::Float(f64::atan2(*y, *x)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math tg
fn tg() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::tan(*int as f64)),
                Value::Float(float) => Value::Float(f64::tan(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math tgh
fn tgh() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::tanh(*int as f64)),
                Value::Float(float) => Value::Float(f64::tanh(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

// Math ctg
fn ctg() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(1.0 / f64::tan(*int as f64)),
                Value::Float(float) => Value::Float(1.0 / f64::tan(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

// Math ctgh
fn ctgh() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(1.0 / f64::tanh(*int as f64)),
                Value::Float(float) => Value::Float(1.0 / f64::tanh(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math sqrt
fn sqrt() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::sqrt(*int as f64)),
                Value::Float(float) => Value::Float(f64::sqrt(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math cbrt
fn cbrt() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::cbrt(*int as f64)),
                Value::Float(float) => Value::Float(f64::cbrt(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math log
fn log() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let a = match values.first().unwrap() {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => utils::error(span, "argument is expected to be a number"),
            };
            let b = match values.get(1).unwrap() {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => utils::error(span, "argument is expected to be a number"),
            };
            Value::Float(a.log(b))
        }
    }
}

/// Math min
fn min() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            match values.first().unwrap() {
                // Int min
                Value::Int(a) => match values.get(1).unwrap() {
                    Value::Int(b) => Value::Int(*a.min(b)),
                    Value::Float(b) => Value::Float((*a as f64).min(*b)),
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                // Float min
                Value::Float(a) => match values.get(1).unwrap() {
                    Value::Int(b) => Value::Float(a.min(*b as f64)),
                    Value::Float(b) => Value::Float(a.min(*b)),
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math max
fn max() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            match values.first().unwrap() {
                // Int max
                Value::Int(a) => match values.get(1).unwrap() {
                    Value::Int(b) => Value::Int(*a.max(b)),
                    Value::Float(b) => Value::Float((*a as f64).max(*b)),
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                // Float max
                Value::Float(a) => match values.get(1).unwrap() {
                    Value::Int(b) => Value::Float(a.max(*b as f64)),
                    Value::Float(b) => Value::Float(a.max(*b)),
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math clamp
fn clamp() -> Ref<Native> {
    native_fun! {
        arity = 3,
        fun = |_, span, values| {
            match values.first().unwrap() {
                // Int clamp
                Value::Int(x) => match (values.get(1).unwrap(), values.get(2).unwrap()) {
                    (Value::Int(a), Value::Int(b)) => {
                        if a > b {
                            utils::error(span, "clamp: min must be <= max")
                        }
                        Value::Int(*x.clamp(a, b))
                    }
                    (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                        let (min, max) = (*a as f64, *b);
                        if min > max {
                            utils::error(span, "clamp: min must be <= max")
                        }
                        Value::Float((*x as f64).clamp(min, max))
                    }
                    (Value::Float(a), Value::Float(b)) => {
                        if a > b {
                            utils::error(span, "clamp: min must be <= max")
                        }
                        Value::Float((*x as f64).clamp(*a, *b))
                    }
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                // Float clamp
                Value::Float(x) => match (values.get(1).unwrap(), values.get(2).unwrap()) {
                    (Value::Int(a), Value::Int(b)) => {
                        if a > b {
                            utils::error(span, "clamp: min must be <= max")
                        }
                        Value::Float(x.clamp(*a as f64, *b as f64))
                    }
                    (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                        let (min, max) = (*a as f64, *b);
                        if min > max {
                            utils::error(span, "clamp: min must be <= max")
                        }
                        Value::Float(x.clamp(min, max))
                    }
                    (Value::Float(a), Value::Float(b)) => {
                        if a > b {
                            utils::error(span, "clamp: min must be <= max")
                        }
                        Value::Float(x.clamp(*a, *b))
                    }
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math log2
fn log2() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::log2(*int as f64)),
                Value::Float(float) => Value::Float(f64::log2(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math log10
fn log10() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::log10(*int as f64)),
                Value::Float(float) => Value::Float(f64::log10(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math exp
fn exp() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::exp(*int as f64)),
                Value::Float(float) => Value::Float(f64::exp(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math exp
fn exp2() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => Value::Float(f64::exp2(*int as f64)),
                Value::Float(float) => Value::Float(f64::exp2(*float)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math abs
fn abs() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(int) => match int.checked_abs() {
                    Some(result) => Value::Int(result),
                    None => utils::error(span, "int overflow in abs"),
                },
                Value::Float(float) => Value::Float(float.abs()),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math floor
fn floor() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Float(float) => Value::Float(float.floor()),
                Value::Int(int) => Value::Int(*int),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math ceil
fn ceil() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Float(float) => Value::Float(float.ceil()),
                Value::Int(int) => Value::Int(*int),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math trunc
fn trunc() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Float(float) => Value::Float(float.trunc()),
                Value::Int(int) => Value::Int(*int),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math round
fn round() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Float(float) => Value::Float(float.round()),
                Value::Int(int) => Value::Int(*int),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math pow
fn pow() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            match values.first().unwrap() {
                // Int pow
                Value::Int(a) => match values.get(1).unwrap() {
                    // Int exp
                    Value::Int(b) => {
                        use std::convert::TryInto;

                        // Positive exponent
                        if *b >= 0 {
                            // Safe cast
                            let b_u32: u32 = (*b).try_into().unwrap_or_else(|_| {
                                utils::error(span, &format!("exponent {} is too large", b))
                            });

                            match a.checked_pow(b_u32) {
                                Some(result) => Value::Int(result),
                                None => utils::error(span, "int overflow in exp"),
                            }
                        }
                        // Negative exponent
                        else {
                            // Safe cast
                            let b_i32: i32 = (*b).try_into().unwrap_or_else(|_| {
                                utils::error(span, &format!("exponent {} is too small", b))
                            });

                            Value::Float((*a as f64).powi(b_i32))
                        }
                    }
                    // Float exp
                    Value::Float(b) => Value::Float((*a as f64).powf(*b)),
                    // Otherwise, raising error
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                // Float pow
                Value::Float(a) => match values.get(1).unwrap() {
                    // Int exp
                    Value::Int(b) => Value::Float(a.powi(*b as i32)),
                    // Float exp
                    Value::Float(b) => Value::Float(a.powf(*b)),
                    // Otherwise, raising error
                    _ => utils::error(span, "argument is expected to be a number"),
                },
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Math hypot
fn hypot() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            match (values.first().unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Float(f64::hypot(*x as f64, *y as f64)),
                (Value::Int(x), Value::Float(y)) => Value::Float(f64::hypot(*x as f64, *y)),
                (Value::Float(x), Value::Int(y)) => Value::Float(f64::hypot(*x, *y as f64)),
                (Value::Float(x), Value::Float(y)) => Value::Float(f64::hypot(*x, *y)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Random numner
fn rnd() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            match (values.first().unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Int(rand::rng().random_range(*x..*y)),
                (Value::Int(x), Value::Float(y)) => {
                    Value::Float(rand::rng().random_range((*x as f64)..*y))
                }
                (Value::Float(x), Value::Int(y)) => {
                    Value::Float(rand::rng().random_range(*x..(*y as f64)))
                }
                (Value::Float(x), Value::Float(y)) => {
                    Value::Float(rand::rng().random_range(*x..*y))
                }
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }
    }
}

/// Provides `math` module env
pub fn provide_env() -> RealmRef {
    realm! {
        sin => callable!(sin()),
        sinh => callable!(sinh()),
        cos => callable!(cos()),
        cosh => callable!(cosh()),
        asin => callable!(asin()),
        asinh => callable!(asinh()),
        acos => callable!(acos()),
        acosh => callable!(acosh()),
        atg => callable!(atg()),
        atg2 => callable!(atg2()),
        tg => callable!(tg()),
        tgh => callable!(tgh()),
        ctg => callable!(ctg()),
        ctgh => callable!(ctgh()),
        sqrt => callable!(sqrt()),
        cbrt => callable!(cbrt()),
        log => callable!(log()),
        min => callable!(min()),
        max => callable!(max()),
        clamp => callable!(clamp()),
        log2 => callable!(log2()),
        log10 => callable!(log10()),
        exp => callable!(exp()),
        exp2 => callable!(exp2()),
        abs => callable!(abs()),
        floor => callable!(floor()),
        ceil => callable!(ceil()),
        trunc => callable!(trunc()),
        round => callable!(round()),
        pow => callable!(pow()),
        hypot => callable!(hypot()),
        rnd => callable!(rnd())
    }
}
