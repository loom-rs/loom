/// Prepares miette for tests
#[cfg(test)]
pub fn prepare_miette() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(false)
                .rgb_colors(miette::RgbColors::Preferred)
                .show_related_errors_as_nested()
                .context_lines(3)
                .build(),
        )
    }));
}

/// Asserts tokens
#[macro_export]
macro_rules! assert_tokens {
    ($text:expr) => {
        let result = match std::panic::catch_unwind(|| {
            $crate::assertion::prepare_miette();
            let src = std::sync::Arc::new(miette::NamedSource::new("test.ql", $text.to_string()));
            let lexer = geko_lex::lexer::Lexer::new(src.clone(), $text);
            format!("{:#?}", lexer.collect::<Vec<geko_lex::token::Token>>())
        }) {
            Ok(result) => result,
            Err(err) => {
                let panic_str = if let Some(s) = err.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<failed to retrieve panic message>".to_string()
                };
                let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                let cleaned = re.replace_all(&panic_str, "").to_string();
                format!("{}", cleaned)
            }
        };

        insta::assert_snapshot!(result);
    };
}

/// Asserts ast
#[macro_export]
macro_rules! assert_ast {
    ($text:expr) => {{
        let result = match std::panic::catch_unwind(|| {
            $crate::assertion::prepare_miette();
            let src = std::sync::Arc::new(miette::NamedSource::new("test.ql", $text.to_string()));
            let lexer = geko_lex::lexer::Lexer::new(src.clone(), $text);
            let mut parser = geko_parse::Parser::new(src, lexer);
            let ast = parser.parse();
            format!("{:#?}", ast)
        }) {
            Ok(result) => result,
            Err(err) => {
                let panic_str = if let Some(s) = err.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<failed to retrieve panic message>".to_string()
                };
                let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                let cleaned = re.replace_all(&panic_str, "").to_string();
                format!("{}", cleaned)
            }
        };

        insta::assert_snapshot!(result);
    }};
}

/// Asserts evaluation result
#[macro_export]
macro_rules! assert_eval {
    ($text:expr) => {{
        let result = match std::panic::catch_unwind(|| {
            $crate::assertion::prepare_miette();
            let mut io = $crate::io::TestIO {
                buffer: std::cell::RefCell::new(String::new()),
            };
            let mut interpreter = geko_rt::interpreter::Interpreter::new(&mut io);
            let _ = interpreter.interpret_module("test.ql", $text);
            format!("{:#?}", io.buffer.borrow())
        }) {
            Ok(result) => result,
            Err(err) => {
                let panic_str = if let Some(s) = err.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<failed to retrieve panic message>".to_string()
                };
                let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                let cleaned = re.replace_all(&panic_str, "").to_string();
                format!("{}", cleaned)
            }
        };

        insta::assert_snapshot!(result);
    }};
}

/// Asserts semantic analysis result
#[macro_export]
macro_rules! assert_sema {
    ($text:expr) => {{
        let result = match std::panic::catch_unwind(|| {
            $crate::assertion::prepare_miette();
            let src = std::sync::Arc::new(miette::NamedSource::new("test.geko", $text.to_string()));
            let lexer = geko_lex::lexer::Lexer::new(src.clone(), $text);
            let mut parser = geko_parse::Parser::new(src, lexer);
            let ast = parser.parse();
            let mut analyzer = geko_sema::Analyzer::default();
            analyzer.analyze_module(&ast);
            "Ok.".to_string()
        }) {
            Ok(result) => result,
            Err(err) => {
                let panic_str = if let Some(s) = err.downcast_ref::<&str>() {
                    (*s).to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<failed to retrieve panic message>".to_string()
                };
                let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                let cleaned = re.replace_all(&panic_str, "").to_string();
                format!("{}", cleaned)
            }
        };

        insta::assert_snapshot!(result);
    }};
}
