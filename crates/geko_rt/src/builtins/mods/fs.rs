/// Imports
use crate::{
    builtin_class,
    builtins::utils,
    callable, native_fun, realm,
    refs::{MutRef, RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};
use camino::Utf8PathBuf;
use geko_common::{bail, bug, io::IOError};
use geko_lex::token::Span;
use std::{
    cell::RefCell,
    fs::{self, File},
};

/// Helper: validates path
fn validate_path<F, V>(span: &Span, path: Value, f: F) -> V
where
    F: FnOnce(Utf8PathBuf) -> V,
{
    match path {
        Value::String(path) => f(Utf8PathBuf::from(path)),
        other => utils::error(span, &format!("`{other}` is not a valid path")),
    }
}

/// Helper: validates path argument by index
fn validate_path_arg<F, V>(span: &Span, values: &[Value], index: usize, f: F) -> V
where
    F: FnOnce(Utf8PathBuf) -> V,
{
    validate_path(span, values.get(index).cloned().unwrap(), f)
}

/// Helper: validates one path argument
fn validate_one_path_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(Utf8PathBuf) -> V,
{
    validate_path_arg(span, values, 0, f)
}

/// Helper: validates two path arguments
fn validate_two_path_args<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(Utf8PathBuf, Utf8PathBuf) -> V,
{
    validate_path_arg(span, values, 0, |from| {
        validate_path_arg(span, values, 1, |to| f(from, to))
    })
}

/// Is path exists?
fn is_exists() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("exists"))
                } else {
                    Value::Bool(path.exists())
                }
            })
        }
    }
}

/// Is path a directory?
fn is_dir() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("is_dir"))
                } else {
                    Value::Bool(path.is_dir())
                }
            })
        }
    }
}

/// Is path a file?
fn is_file() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("is_file"))
                } else {
                    Value::Bool(path.is_file())
                }
            })
        }
    }
}

/// Returns file name
fn file_name() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                path.file_name()
                    .map(|it| Value::String(it.to_string()))
                    .unwrap_or(Value::Null)
            })
        }
    }
}

/// Returns file stem
fn file_stem() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                path.file_stem()
                    .map(|it| Value::String(it.to_string()))
                    .unwrap_or(Value::Null)
            })
        }
    }
}

/// Get file extension
fn file_extension() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                Value::String(
                    path.extension()
                        .map(|it| it.to_string())
                        .unwrap_or(String::new()),
                )
            })
        }
    }
}

/// Make directory
fn mk_dir() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("mk_dir"))
                } else {
                    match fs::create_dir(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to make directory: `{err}`"))
                        }
                    }
                }
            })
        }
    }
}

/// Make directory and it's parents
fn mk_dir_all() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("mk_dir_all"))
                } else {
                    match fs::create_dir_all(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to make directory: `{err}`"))
                        }
                    }
                }
            })
        }
    }
}

/// Make file
fn mk_file() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("mk_file"))
                } else {
                    match File::create(path) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to create file: `{err}`")),
                    }
                }
            })
        }
    }
}

/// Remove empty directory
fn rm_dir() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rm_dir"))
                } else {
                    match fs::remove_dir(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to remove directory: `{err}`"))
                        }
                    }
                }
            })
        }
    }
}

/// Remove directory and it's contents
fn rm_dir_all() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rm_dir_all"))
                } else {
                    match fs::remove_dir_all(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to remove directory: `{err}`"))
                        }
                    }
                }
            })
        }
    }
}

/// Remove file
fn rm_file() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rm_file"))
                } else {
                    match fs::remove_file(path) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to remove file: `{err}`")),
                    }
                }
            })
        }
    }
}

/// Files list
fn read_dir() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |rt, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("read_dir"))
                } else {
                    // Retrieving contents
                    let contents = match fs::read_dir(path) {
                        Ok(entries) => entries
                            .map(|entry| match entry {
                                Ok(path) => Value::String(format!("{:?}", path.path())),
                                Err(err) => {
                                    utils::error(span, &format!("failed to read entry: `{err}`"))
                                }
                            })
                            .collect::<Vec<Value>>(),
                        Err(err) => {
                            utils::error(span, &format!("failed to read directory: `{err}`"))
                        }
                    };

                    // Retrieving list class
                    let class = builtin_class!(rt, "List");

                    // Calling class
                    match rt.call_class(span, Vec::new(), class) {
                        Ok(Value::Instance(list)) => {
                            list.borrow_mut().fields.insert(
                                "$internal".to_string(),
                                Value::Any(MutRef::new(RefCell::new(contents))),
                            );
                            Value::Instance(list)
                        }
                        _ => bug!("invalid list instantiation"),
                    }
                }
            })
        }
    }
}

/// Copy file
fn copy() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            validate_two_path_args(span, &values, |from, to| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("copy"))
                } else {
                    match fs::copy(from, to) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to copy file: `{err}`")),
                    }
                }
            })
        }
    }
}

/// Rename file
fn rename() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            validate_two_path_args(span, &values, |from, to| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rename"))
                } else {
                    match fs::rename(from, to) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to rename file: `{err}`")),
                    }
                }
            })
        }
    }
}

/// Read file text
fn read() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |rt, span, values| {
            validate_one_path_arg(span, &values, |path| Value::String(rt.io.read(&path)))
        }
    }
}

/// Write text to file
fn write() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |rt, span, values| {
            validate_one_path_arg(span, &values, |path| {
                match values.get(1).cloned().unwrap() {
                    Value::String(text) => {
                        rt.io.write(&path, text);
                        Value::Null
                    }
                    other => utils::error(span, &format!("`{other}` is not valid content")),
                }
            })
        }
    }
}

/// Provides `fs` module env
pub fn provide_env() -> RealmRef {
    realm! {
        is_exists => callable!(is_exists()),
        is_dir => callable!(is_dir()),
        is_file => callable!(is_file()),
        file_name => callable!(file_name()),
        file_stem => callable!(file_stem()),
        file_extension => callable!(file_extension()),
        mk_dir => callable!(mk_dir()),
        mk_dir_all => callable!(mk_dir_all()),
        mk_file => callable!(mk_file()),
        rm_dir => callable!(rm_dir()),
        rm_dir_all => callable!(rm_dir_all()),
        rm_file => callable!(rm_file()),
        read_dir => callable!(read_dir()),
        copy => callable!(copy()),
        rename => callable!(rename()),
        read => callable!(read()),
        write => callable!(write())
    }
}
