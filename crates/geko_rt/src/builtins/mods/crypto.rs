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
use base64::{Engine, prelude::BASE64_STANDARD};
use md5::{Digest, Md5};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

/// Base64 encode
fn b64() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(BASE64_STANDARD.encode(values.first().cloned().unwrap().to_string()))
        }
    }
}

/// Base64 decode
fn de_b64() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match BASE64_STANDARD.decode(values.first().cloned().unwrap().to_string()) {
                Ok(bytes) => Value::String(String::from_utf8_lossy(&bytes).to_string()),
                Err(err) => utils::error(span, &format!("failed to decode `base64` string: {err}")),
            }
        }
    }
}

/// Sha1 encode
fn sha1() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(hex::encode(Sha1::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }
    }
}

/// Sha256 encode
fn sha256() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(hex::encode(Sha256::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }
    }
}

/// Sha224 encode
fn sha224() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(hex::encode(Sha224::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }
    }
}

/// Sha512 encode
fn sha512() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(hex::encode(Sha512::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }
    }
}

/// Sha384 encode
fn sha384() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(hex::encode(Sha384::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }
    }
}

/// Md5 encode
fn md5() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(hex::encode(Md5::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }
    }
}

/// Provides `is` module realm
pub fn provide_env() -> RealmRef {
    realm!(
        b64 => callable!(b64()),
        de_b64 => callable!(de_b64()),
        sha1 => callable!(sha1()),
        sha256 => callable!(sha256()),
        sha224 => callable!(sha224()),
        sha512 => callable!(sha512()),
        sha384 => callable!(sha384()),
        md5 => callable!(md5())
    )
}
