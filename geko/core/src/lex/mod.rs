/// Modules
#[allow(unused_assignments)]
pub mod errors;
pub mod token;

/// Imports
use crate::lex::{
    errors::LexError,
    token::{Token, TokenKind},
};
use common::{bail, span::Span};
use miette::NamedSource;
use std::{str::Chars, sync::Arc};

/// Defines a lexer, an entity that eats
/// chars buffer and spits out tokens
pub struct Lexer<'s> {
    /// Current file source
    source: Arc<NamedSource<String>>,

    /// Chars lexer iterates on
    chars: Chars<'s>,

    /// Current and next
    idx: usize,
    current: Option<char>,
    next: Option<char>,
}

/// Implementation
impl<'s> Lexer<'s> {
    /// Creates new lexer
    pub fn new(file: Arc<NamedSource<String>>, source: &'s str) -> Self {
        let mut chars = source.chars();
        let (current, next) = (chars.next(), chars.next());
        Self {
            source: file,
            chars,
            current,
            next,
            idx: 0,
        }
    }

    /// Takes step in a chars iterator, increments index
    fn advance(&mut self) {
        self.current = self.next.take();
        self.next = self.chars.next();
        self.idx += 1;
    }

    /// Advances char and returns token
    fn advance_with(&mut self, tk: TokenKind, lexeme: &str) -> Token {
        self.advance();
        Token::new(
            Span(self.source.clone(), self.idx - 1..self.idx),
            tk,
            lexeme.to_string(),
        )
    }

    /// Advances char twice and returns token
    fn advance_twice_with(&mut self, tk: TokenKind, lexeme: &str) -> Token {
        self.advance();
        self.advance();
        Token::new(
            Span(self.source.clone(), self.idx - 2..self.idx),
            tk,
            lexeme.to_string(),
        )
    }

    /// Scans unicode codepoint.
    fn scan_unicode_codepoint(&mut self, small: bool) -> char {
        let start_location = self.idx - 1;

        // Calculating amount of hex digits
        let hex_digits_amount = if small { 4 } else { 8 };

        // Checking for a left brace
        if self.current != Some('{') {
            bail!(LexError::InvalidEscapeSequence {
                src: self.source.clone(),
                span: (start_location..self.idx).into(),
                cause: "expected unicode codepoint start `{`."
            })
        }
        self.advance();

        // Reading hex digits to buffer
        let mut buffer = String::new();
        for _ in 0..hex_digits_amount {
            match self.current {
                Some(ch) => {
                    if !ch.is_ascii_hexdigit() {
                        bail!(LexError::InvalidEscapeSequence {
                            src: self.source.clone(),
                            span: (start_location..self.idx).into(),
                            cause: "expected hex digit."
                        })
                    }
                    self.advance();
                    buffer.push(ch);
                }
                None => bail!(LexError::InvalidEscapeSequence {
                    src: self.source.clone(),
                    span: (start_location..self.idx).into(),
                    cause: "unexpected eof."
                }),
            }
        }

        // Checking for a right brace
        if self.current != Some('}') {
            bail!(LexError::InvalidEscapeSequence {
                src: self.source.clone(),
                span: (start_location..self.idx).into(),
                cause: "expected unicode codepoint end `}`."
            })
        }
        self.advance();

        // Parsing char from hex
        match char::from_u32(u32::from_str_radix(&buffer, 16).expect("Invalid hex")) {
            Some(c) => c,
            None => {
                bail!(LexError::InvalidEscapeSequence {
                    src: self.source.clone(),
                    span: (start_location..self.idx).into(),
                    cause: "failed to convert `unciode char` into `u32`."
                })
            }
        }
    }

    /// Scans byte codepoint.
    fn scan_byte_codepoint(&mut self) -> char {
        let start_location = self.idx - 1;

        // Checking for a left brace
        if self.current != Some('{') {
            bail!(LexError::InvalidEscapeSequence {
                src: self.source.clone(),
                span: (start_location..self.idx).into(),
                cause: "expected byte codepoint start `{`."
            })
        }
        self.advance();

        // Reading hex digits to buffer
        let mut buffer = String::new();
        for _ in 0..2 {
            match self.current {
                Some(ch) => {
                    if !ch.is_ascii_hexdigit() {
                        bail!(LexError::InvalidEscapeSequence {
                            src: self.source.clone(),
                            span: (start_location..self.idx).into(),
                            cause: "expected hex digit."
                        })
                    }
                    self.advance();
                    buffer.push(ch);
                }
                None => bail!(LexError::InvalidEscapeSequence {
                    src: self.source.clone(),
                    span: (start_location..self.idx).into(),
                    cause: "unexpected eof."
                }),
            }
        }

        // Checking for a right brace
        if self.current != Some('}') {
            bail!(LexError::InvalidEscapeSequence {
                src: self.source.clone(),
                span: (start_location..self.idx).into(),
                cause: "expected byte codepoint end `}`."
            })
        }
        self.advance();

        // Parsing char from hex
        match char::from_u32(u32::from_str_radix(&buffer, 16).expect("Invalid hex")) {
            Some(c) => c,
            None => {
                bail!(LexError::InvalidEscapeSequence {
                    src: self.source.clone(),
                    span: (start_location..self.idx).into(),
                    cause: "failed to convert `unciode char` into `u32`."
                })
            }
        }
    }

    /// Advances escape sequence.
    fn advance_escape_sequence(&mut self) -> char {
        // Eating `\` char
        self.advance();

        // Reading and eating escape char
        let ch = self.current;
        self.advance();

        // Checking character kind.
        match ch {
            Some('n') => '\n',
            Some('r') => '\r',
            Some('"') => '"',
            Some('`') => '`',
            Some('\\') => '\\',
            Some('u') => self.scan_unicode_codepoint(true),
            Some('U') => self.scan_unicode_codepoint(false),
            Some('x') => self.scan_byte_codepoint(),
            _ => bail!(LexError::UnknownEscapeSequence {
                src: self.source.clone(),
                span: (self.idx - 1..self.idx).into()
            }),
        }
    }

    /// Advances string
    fn advance_string(&mut self) -> Token {
        // Eating `"`
        self.advance();
        let start = self.idx;

        // Reading string to buffer before reaching `"`
        let mut buffer = String::new();
        while self.current != Some('"') {
            // Checking for next char
            match &self.current {
                Some('\\') => buffer.push(self.advance_escape_sequence()),
                Some(_) => {
                    buffer.push(self.current.unwrap());
                    self.advance();
                }
                None => bail!(LexError::UnclosedStringQuotes {
                    src: self.source.clone(),
                    span: (start..self.idx).into(),
                }),
            }
        }

        // Eating `"`
        self.advance();
        let end = self.idx;
        Token::new(
            Span(self.source.clone(), start..end),
            TokenKind::String,
            buffer,
        )
    }

    /// Eats digits sequence into specified buffer
    fn eat_digits(&mut self, buffer: &mut String) {
        // Reading digits before reaching non-digit char or eof
        while self.is_digit() && !self.is_eof() {
            buffer.push(self.current.unwrap());
            self.advance();
        }
    }

    /// Advances number
    fn advance_number(&mut self) -> Token {
        let start = self.idx;

        // Preparing number buffer
        let mut buffer = String::new();

        // Reading first sequence of digits
        self.eat_digits(&mut buffer);

        // If dot presented and it's not a double dot,
        // reading second sequence of digits
        if self.current == Some('.') && self.next != Some('.') {
            self.advance();
            buffer.push('.');
            self.eat_digits(&mut buffer);
        }

        // If exponent presented, reading it
        if matches!(self.current, Some('e') | Some('E')) {
            buffer.push(self.current.unwrap());
            self.advance();

            // Checking for sign
            if matches!(self.current, Some('+') | Some('-')) {
                buffer.push(self.current.unwrap());
                self.advance();
            }

            // Parsing last sequence of digits
            self.eat_digits(&mut buffer);
        }

        Token::new(
            Span(self.source.clone(), start..self.idx),
            TokenKind::Number,
            buffer,
        )
    }

    /// Token kind for id
    fn token_kind_for_id(value: &str) -> TokenKind {
        match value {
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "until" => TokenKind::Until,
            "in" => TokenKind::In,
            "use" => TokenKind::Use,
            "class" => TokenKind::Class,
            "enum" => TokenKind::Enum,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "continue" => TokenKind::Continue,
            "break" => TokenKind::Break,
            "as" => TokenKind::As,
            "true" => TokenKind::Bool,
            "false" => TokenKind::Bool,
            "fun" => TokenKind::Fun,
            "null" => TokenKind::Null,
            "trait" => TokenKind::Trait,
            "pick" => TokenKind::Pick,
            _ => TokenKind::Id,
        }
    }

    /// Advances id or keyword
    fn advance_id_or_kw(&mut self) -> Token {
        // Reading id before reaching
        // char that is not a letter, digit or underscore
        let start = self.idx;
        let mut buffer = String::new();
        while (self.is_id_letter() || self.is_digit()) && !self.is_eof() {
            buffer.push(self.current.unwrap());
            self.advance();
        }
        let end = self.idx;

        Token::new(
            Span(self.source.clone(), start..end),
            Self::token_kind_for_id(&buffer),
            buffer,
        )
    }

    /// Skips comment
    fn skip_comment(&mut self) {
        // Eating `#`
        self.advance();

        // Eating comment before reaching new line or eof
        while self.current != Some('\n') && !self.is_eof() {
            self.advance();
        }
    }

    /// Skips multiline comment
    fn skip_multiline_comment(&mut self) {
        // Eating `#[`
        let start = self.idx;
        self.advance();
        self.advance();

        // Eating comment before reaching `]#`
        while !(self.current == Some(']') && self.next == Some('#')) {
            // If eof -> reporting error
            if self.is_eof() {
                bail!(LexError::UnterminatedComment {
                    src: self.source.clone(),
                    span: (start..self.idx).into()
                })
            } else {
                self.advance();
            }
        }

        // Eating `]#`
        self.advance();
        self.advance();
    }

    /// Skips whitespaces and comments
    fn skip_trivia(&mut self) {
        loop {
            // Skipping whitespaces
            while self.is_whitespace() {
                self.advance();
            }

            // Skipping comments
            if self.current == Some('#') {
                // Skipping multiline comment
                if self.next == Some('[') {
                    self.skip_multiline_comment();
                }
                // Skipping single line comment
                else {
                    self.skip_comment();
                }
                continue;
            }

            break;
        }
    }

    /// Returns true if char is ` `, `\n`, `\t` or `\r`
    #[allow(clippy::match_like_matches_macro)]
    fn is_whitespace(&self) -> bool {
        match self.current {
            Some(' ') | Some('\n') | Some('\t') | Some('\r') => true,
            _ => false,
        }
    }

    /// Returns `true` if char is letter or underscore
    #[allow(clippy::match_like_matches_macro)]
    fn is_id_letter(&self) -> bool {
        match self.current {
            Some(it) if it.is_ascii_alphabetic() || it == '_' => true,
            _ => false,
        }
    }

    /// Returns `true` if current char is ascii digit
    #[allow(clippy::match_like_matches_macro)]
    fn is_digit(&self) -> bool {
        match self.current {
            Some(it) if it.is_ascii_digit() => true,
            _ => false,
        }
    }

    /// Returns `true` if `current` is `None`
    fn is_eof(&self) -> bool {
        self.current.is_none()
    }
}

/// Iterator implementation
impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skipping trivia chars
        self.skip_trivia();

        // Matching current and next
        match (self.current, self.next) {
            (Some('+'), Some('=')) => Some(self.advance_twice_with(TokenKind::PlusEq, "+=")),
            (Some('-'), Some('=')) => Some(self.advance_twice_with(TokenKind::MinusEq, "-=")),
            (Some('*'), Some('=')) => Some(self.advance_twice_with(TokenKind::StarEq, "*=")),
            (Some('/'), Some('=')) => Some(self.advance_twice_with(TokenKind::SlashEq, "/=")),
            (Some('%'), Some('=')) => Some(self.advance_twice_with(TokenKind::PercentEq, "%=")),
            (Some('&'), Some('=')) => Some(self.advance_twice_with(TokenKind::AmpEq, "&=")),
            (Some('|'), Some('=')) => Some(self.advance_twice_with(TokenKind::BarEq, "|=")),
            (Some('^'), Some('=')) => Some(self.advance_twice_with(TokenKind::CaretEq, "^=")),
            (Some('&'), Some('&')) => Some(self.advance_twice_with(TokenKind::DoubleAmp, "&&")),
            (Some('|'), Some('|')) => Some(self.advance_twice_with(TokenKind::DoubleBar, "||")),
            (Some('='), Some('=')) => Some(self.advance_twice_with(TokenKind::DoubleEq, "==")),
            (Some('!'), Some('=')) => Some(self.advance_twice_with(TokenKind::BangEq, "!=")),
            (Some('.'), Some('.')) => Some(self.advance_twice_with(TokenKind::DoubleDot, "..")),
            (Some('>'), Some('=')) => Some(self.advance_twice_with(TokenKind::Ge, ">=")),
            (Some('<'), Some('=')) => Some(self.advance_twice_with(TokenKind::Le, "<=")),
            (Some('>'), Some(':')) => Some(self.advance_twice_with(TokenKind::GtColon, ">:")),
            (Some('>'), Some('!')) => Some(self.advance_twice_with(TokenKind::GtBang, ">!")),
            (Some('-'), Some('>')) => Some(self.advance_twice_with(TokenKind::Arrow, "->")),
            (Some(':'), Some('=')) => Some(self.advance_twice_with(TokenKind::Walrus, ":=")),
            (Some('&'), _) => Some(self.advance_with(TokenKind::Amp, "&")),
            (Some('|'), _) => Some(self.advance_with(TokenKind::Bar, "|")),
            (Some('^'), _) => Some(self.advance_with(TokenKind::Caret, "^")),
            (Some('%'), _) => Some(self.advance_with(TokenKind::Percent, "%")),
            (Some('+'), _) => Some(self.advance_with(TokenKind::Plus, "+")),
            (Some('-'), _) => Some(self.advance_with(TokenKind::Minus, "-")),
            (Some('*'), _) => Some(self.advance_with(TokenKind::Star, "*")),
            (Some('/'), _) => Some(self.advance_with(TokenKind::Slash, "/")),
            (Some('!'), _) => Some(self.advance_with(TokenKind::Bang, "!")),
            (Some('='), _) => Some(self.advance_with(TokenKind::Eq, "=")),
            (Some('>'), _) => Some(self.advance_with(TokenKind::Gt, ">")),
            (Some('<'), _) => Some(self.advance_with(TokenKind::Lt, "<")),
            (Some('.'), _) => Some(self.advance_with(TokenKind::Dot, ".")),
            (Some(','), _) => Some(self.advance_with(TokenKind::Comma, ",")),
            (Some('{'), _) => Some(self.advance_with(TokenKind::Lbrace, "{")),
            (Some('}'), _) => Some(self.advance_with(TokenKind::Rbrace, "}")),
            (Some('['), _) => Some(self.advance_with(TokenKind::Lbracket, "[")),
            (Some(']'), _) => Some(self.advance_with(TokenKind::Rbracket, "]")),
            (Some('('), _) => Some(self.advance_with(TokenKind::Lparen, "(")),
            (Some(')'), _) => Some(self.advance_with(TokenKind::Rparen, ")")),
            (Some(':'), _) => Some(self.advance_with(TokenKind::Colon, ":")),
            (Some('"'), _) => Some(self.advance_string()),
            (Some(ch), _) => {
                if self.is_digit() {
                    Some(self.advance_number())
                } else if self.is_id_letter() {
                    Some(self.advance_id_or_kw())
                } else {
                    bail!(LexError::UnexpectedChar {
                        ch,
                        src: self.source.clone(),
                        span: (self.idx..self.idx).into(),
                    })
                }
            }
            (_, _) => None,
        }
    }
}
