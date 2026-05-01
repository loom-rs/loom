/// Imports
use crate::{
    callable, native_fun, realm,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};
use sysinfo::System;

/// Total memory
fn total() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.total_memory() as i64)
        }
    }
}

/// Memory usage
fn used() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.used_memory() as i64)
        }
    }
}

/// Free memory
fn free() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.free_memory() as i64)
        }
    }
}

/// Free swapp
fn total_swap() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.total_swap() as i64)
        }
    }
}

/// Swap usage
fn used_swap() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.used_swap() as i64)
        }
    }
}

/// Free swap
fn free_swap() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.free_swap() as i64)
        }
    }
}

/// Size of
fn size_of() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::Int(std::mem::size_of_val(&values.first().cloned().unwrap()) as i64)
        }
    }
}

/// Align of
fn align_of() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::Int(std::mem::align_of_val(&values.first().cloned().unwrap()) as i64)
        }
    }
}

/// Provides `mem` module env
pub fn provide_env() -> RealmRef {
    realm! {
        total => callable!(total()),
        free => callable!(free()),
        used => callable!(used()),
        total_swap => callable!(total_swap()),
        used_swap => callable!(used_swap()),
        free_swap => callable!(free_swap()),
        size_of => callable!(size_of()),
        align_of => callable!(align_of())
    }
}
