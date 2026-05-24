/// Imports
use crate::{
    arg, builtin_module_class, callable, class, error, expect, expect_as, interpreter::Interpreter, native_class, native_fun, native_method, realm, refs::{MutRef, RealmRef, Ref}, rt::{
        realm::Realm,
        value::{Class, Method, Native, Value},
    }
};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, Timelike, Utc};
use geko_common::bug;
use geko_lex::token::Span;
use std::{cell::RefCell, collections::HashMap};

/// Helper: creates fresh `Time` with `NaiveTimeDelta`
fn fresh_time(rt: &mut Interpreter, span: &Span, time: NaiveDateTime) -> Value {
    // Searching `Time` class
    let class = builtin_module_class!(rt, "time", "Time");

    // Creating `Time` instance
    match rt.call_class(
        span,
        class,
        vec![Value::Any(MutRef::new(RefCell::new(time)))],
    ) {
        Ok(val) => val,
        Err(_) => bug!("control flow leak"),
    }
}

/// Helper: validates time
fn validate_time<F, V>(span: &Span, value: Value, f: F) -> V
where
    F: FnOnce(NaiveDateTime) -> V,
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
                Value::Any(time) => match time.borrow_mut().downcast_mut::<NaiveDateTime>() {
                    Some(time) => f(*time),
                    _ => error!(span, "corrupted time"),
                },
                _ => {
                    error!(span, "corrupted time");
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates time argument
fn validate_time_arg<F, V>(span: &Span, values: &[Value], idx: usize, f: F) -> V
where
    F: FnOnce(NaiveDateTime) -> V,
{
    validate_time(span, arg!(values, idx), f)
}

/// Helper: validates one time argument
fn validate_one_time_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(NaiveDateTime) -> V,
{
    validate_time(span, arg!(values, 0), f)
}

/// Helper: validates two time arguments
fn validate_two_time_args<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(NaiveDateTime, NaiveDateTime) -> V,
{
    validate_time_arg(span, values, 0, |from| {
        validate_time_arg(span, values, 1, |to| f(from, to))
    })
}

/// `Time` init method
fn time_init_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, _, values| {
            let list = arg!(values, 0);
            match list {
                Value::Instance(instance) => {
                    // Setting `$internal` field
                    instance
                        .borrow_mut()
                        .fields
                        .insert("$internal".to_string(), arg!(values, 1));

                    Value::Null
                }
                _ => unreachable!(),
            }
        }
    }
}

/// `Time` year method
fn time_year_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.year() as i64))
        }
    }
}

/// `Time` month method
fn time_month_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.month0() as i64))
        }
    }
}

/// `Time` week method
fn time_week_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::Int(time.iso_week().week() as i64)
            })
        }
    }
}

/// `Time` ordinal method
fn time_ordinal_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.ordinal0() as i64))
        }
    }
}

/// `Time` day method
fn time_day_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.day0() as i64))
        }
    }
}

/// `Time` weekday method
fn time_weekday_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::String(time.weekday().to_string())
            })
        }
    }
}

/// `Time` hour method
fn time_hour_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.hour() as i64))
        }
    }
}

/// `Time` minute method
fn time_minute_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.minute() as i64))
        }
    }
}

/// `Time` second method
fn time_second_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.second() as i64))
        }
    }
}

/// `Time` in seconds method
fn time_in_seconds_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.and_utc().timestamp()))
        }
    }
}

/// `Time` in millis method
fn time_in_millis_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::Int(time.and_utc().timestamp_millis())
            })
        }
    }
}

/// `Time` add weeks method
fn time_add_weeks_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match arg!(values, 1) {
                    Value::Int(weeks) => fresh_time(rt, span, time + Duration::weeks(weeks)),
                    _ => error!(span, "weeks expected to be an int"),
                }
            })
        }
    }
}

/// `Time` add days method
fn time_add_days_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let days = expect_as!(span, arg!(values, 1), Value::Int, "days");
                fresh_time(rt, span, time + Duration::days(days))
            })
        }
    }
}

/// `Time` add hours method
fn time_add_hours_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let hours = expect_as!(span, arg!(values, 1), Value::Int, "hours");
                fresh_time(rt, span, time + Duration::hours(hours))
            })
        }
    }
}

/// `Time` add minutes method
fn time_add_minutes_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let minutes = expect_as!(span, arg!(values, 1), Value::Int, "hours");
                fresh_time(rt, span, time + Duration::minutes(minutes))
            })
        }
    }
}

/// `Time` add seconds method
fn time_add_seconds_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let seconds = expect_as!(span, arg!(values, 1), Value::Int, "seconds");
                fresh_time(rt, span, time + Duration::seconds(seconds))
            })
        }
    }
}

/// `Time` add millis method
fn time_add_millis_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let millis = expect_as!(span, arg!(values, 1), Value::Int, "milliseconds");
                fresh_time(rt, span, time + Duration::milliseconds(millis))
            })
        }
    }
}

/// `Time` add nanos method
fn time_add_nanos_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let nanos = expect_as!(span, arg!(values, 1), Value::Int, "nanoseconds");
                fresh_time(rt, span, time + Duration::nanoseconds(nanos))
            })
        }
    }
}

/// `Time` add micros method
fn time_add_micros_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let micros = expect_as!(span, arg!(values, 1), Value::Int, "microseconds");
                fresh_time(rt, span, time + Duration::microseconds(micros))
            })
        }
    }
}

/// `Time` sub weeks method
fn time_sub_weeks_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let weeks = expect_as!(span, arg!(values, 1), Value::Int, "weeks");
                fresh_time(rt, span, time - Duration::weeks(weeks))
            })
        }
    }
}

/// `Time` sub days method
fn time_sub_days_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let days = expect_as!(span, arg!(values, 1), Value::Int, "days");
                fresh_time(rt, span, time - Duration::days(days))
            })
        }
    }
}

/// `Time` sub hours method
fn time_sub_hours_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let hours = expect_as!(span, arg!(values, 1), Value::Int, "hours");
                fresh_time(rt, span, time - Duration::hours(hours))
            })
        }
    }
}

/// `Time` sub minutes method
fn time_sub_minutes_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let minutes = expect_as!(span, arg!(values, 1), Value::Int, "minutes");
                fresh_time(rt, span, time - Duration::minutes(minutes))
            })
        }
    }
}

/// `Time` sub seconds method
fn time_sub_seconds_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let seconds = expect_as!(span, arg!(values, 1), Value::Int, "seconds");
                fresh_time(rt, span, time - Duration::seconds(seconds))
            })
        }
    }
}

/// `Time` sub millis method
fn time_sub_millis_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let millis = expect_as!(span, arg!(values, 1), Value::Int, "milliseconds");
                fresh_time(rt, span, time - Duration::milliseconds(millis))
            })
        }
    }
}

/// `Time` sub nanos method
fn time_sub_nanos_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let nanos = expect_as!(span, arg!(values, 1), Value::Int, "nanoseconds");
                fresh_time(rt, span, time - Duration::nanoseconds(nanos))
            })
        }
    }
}

/// `Time` sub micros method
fn time_sub_micros_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let micros = expect_as!(span, arg!(values, 1), Value::Int, "microseconds");
                fresh_time(rt, span, time - Duration::microseconds(micros))
            })
        }
    }
}

/// `Time` with year method
fn time_with_year_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let year = expect_as!(span, arg!(values, 1), Value::Int, "year");
                fresh_time(
                    rt,
                    span,
                    match time.with_year(year as i32) {
                        Some(dt) => dt,
                        _ => error!(span, "invalid year"),
                    },
                )
            })
        }
    }
}

/// `Time` with month method
fn time_with_month_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let month = expect_as!(span, arg!(values, 1), Value::Int, "month");

                if month >= 0 {
                    fresh_time(
                        rt,
                        span,
                        match time.with_month0(month as u32) {
                            Some(dt) => dt,
                            _ => error!(span, "invalid month"),
                        },
                    )
                } else {
                    error!(span, "month expected to be positive int")
                }
            })
        }
    }
}

/// `Time` with ordinal method
fn time_with_ordinal_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let day = expect_as!(span, arg!(values, 1), Value::Int, "day");

                if day >= 0 {
                    fresh_time(
                        rt,
                        span,
                        match time.with_ordinal0(day as u32) {
                            Some(dt) => dt,
                            _ => error!(span, "invalid day"),
                        },
                    )
                } else {
                    error!(span, "day expected to be positive int")
                }
            })
        }
    }
}

/// `Time` with day method
fn time_with_day_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let day = expect_as!(span, arg!(values, 1), Value::Int, "day");

                if day >= 0 {
                    fresh_time(
                        rt,
                        span,
                        match time.with_day0(day as u32) {
                            Some(dt) => dt,
                            _ => error!(span, "invalid day"),
                        },
                    )
                } else {
                    error!(span, "day expected to be positive int")
                }
            })
        }
    }
}

/// `Time` with hour method
fn time_with_hour_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let hour = expect_as!(span, arg!(values, 1), Value::Int, "hour");

                if hour >= 0 {
                    fresh_time(
                        rt,
                        span,
                        match time.with_hour(hour as u32) {
                            Some(dt) => dt,
                            _ => error!(span, "invalid hour"),
                        },
                    )
                } else {
                    error!(span, "hour expected to be positive int")
                }
            })
        }
    }
}

/// `Time` with minute method
fn time_with_minute_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let minute = expect_as!(span, arg!(values, 1), Value::Int, "minute");

                if minute >= 0 {
                    fresh_time(
                        rt,
                        span,
                        match time.with_minute(minute as u32) {
                            Some(dt) => dt,
                            _ => error!(span, "invalid minute"),
                        },
                    )
                } else {
                    error!(span, "minute expected to be positive int")
                }
            })
        }
    }
}

/// `Time` with second method
fn time_with_second_method() -> Method {
    native_method! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let second = expect_as!(span, arg!(values, 1), Value::Int, "second");

                if second >= 0 {
                    fresh_time(
                        rt,
                        span,
                        match time.with_minute(second as u32) {
                            Some(dt) => dt,
                            _ => error!(span, "invalid second"),
                        },
                    )
                } else {
                    error!(span, "second expected to be positive int")
                }
            })
        }
    }
}

/// `Time` format method
fn time_format_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                let str = expect!(span, arg!(values, 1), Value::String);
                Value::String(time.format(&str).to_string())
            })
        }
    }
}

/// `Time` gt method
fn time_gt_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a > b))
        }
    }
}

/// `Time` ge method
fn time_ge_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a >= b))
        }
    }
}

/// `Time` lt method
fn time_lt_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a < b))
        }
    }
}

/// `Time` le method
fn time_le_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a <= b))
        }
    }
}

/// Provides `Time` class
fn provide_time_class() -> Ref<Class> {
    native_class! {
        name = Time,
        methods = {
            init => time_init_method(),
            year => time_year_method(),
            month => time_month_method(),
            week => time_week_method(),
            ordinal => time_ordinal_method(),
            day => time_day_method(),
            weekday => time_weekday_method(),
            hour => time_hour_method(),
            minute => time_minute_method(),
            second => time_second_method(),
            in_seconds => time_in_seconds_method(),
            in_millis => time_in_millis_method(),
            add_weeks => time_add_weeks_method(),
            add_days => time_add_days_method(),
            add_hours => time_add_hours_method(),
            add_minutes => time_add_minutes_method(),
            add_seconds => time_add_seconds_method(),
            add_millis => time_add_millis_method(),
            add_nanos => time_add_nanos_method(),
            add_micros => time_add_micros_method(),
            sub_weeks => time_sub_weeks_method(),
            sub_days => time_sub_days_method(),
            sub_hours => time_sub_hours_method(),
            sub_minutes => time_sub_minutes_method(),
            sub_seconds => time_sub_seconds_method(),
            sub_millis => time_sub_millis_method(),
            sub_nanos => time_sub_nanos_method(),
            sub_micros => time_sub_micros_method(),
            with_year => time_with_year_method(),
            with_month => time_with_month_method(),
            with_ordinal => time_with_ordinal_method(),
            with_day => time_with_day_method(),
            with_hour => time_with_hour_method(),
            with_minute => time_with_minute_method(),
            with_second => time_with_second_method(),
            format => time_format_method(),
            gt => time_gt_method(),
            ge => time_ge_method(),
            lt => time_lt_method(),
            le => time_le_method()
        }
    }
}

/// Time local
fn local() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |rt, span, _| {
            fresh_time(rt, span, Local::now().naive_local())
        }
    }
}

/// Time utc
fn utc() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |rt, span, _| {
            fresh_time(rt, span, Utc::now().naive_local())
        }
    }
}

/// Time from seconds
fn from_seconds() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |rt, span, values| {
            match arg!(values, 0) {
                Value::Int(seconds) => fresh_time(
                    rt,
                    span,
                    match DateTime::from_timestamp_secs(seconds) {
                        Some(dt) => dt.naive_local(),
                        None => error!(span, "invalid timestamp"),
                    },
                ),
                _ => error!(span, "seconds expected to be an int"),
            }
        }
    }
}

/// Time from millis
fn from_millis() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |rt, span, values| {
            match arg!(values, 0) {
                Value::Int(seconds) => fresh_time(
                    rt,
                    span,
                    match DateTime::from_timestamp_millis(seconds) {
                        Some(dt) => dt.naive_local(),
                        None => error!(span, "invalid timestamp"),
                    },
                ),
                _ => error!(span, "millis expected to be an int"),
            }
        }
    }
}

/// Time from nanos
fn from_nanos() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |rt, span, values| {
            match arg!(values, 0) {
                Value::Int(nanos) => fresh_time(
                    rt,
                    span,
                    DateTime::from_timestamp_nanos(nanos).naive_local(),
                ),
                _ => error!(span, "nanos expected to be an int"),
            }
        }
    }
}

/// Provides `is` module env
pub fn provide_env() -> RealmRef {
    realm! {
        local => callable!(local()),
        utc => callable!(utc()),
        from_seconds => callable!(from_seconds()),
        from_millis => callable!(from_millis()),
        from_nanos => callable!(from_nanos()),
        Time => class!(provide_time_class())
    }
}
