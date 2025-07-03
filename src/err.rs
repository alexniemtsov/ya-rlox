use std::error::Error;
use std::fmt;

use crate::scanner::Token;

#[derive(Clone, Debug)]
pub struct LoxError {
    pub line: usize,
    pub where_: String,
    pub msg: String,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.where_, self.msg
        )
    }
}

impl LoxError {
    pub fn new(line: usize, where_: String, msg: impl Into<String>) -> Self {
        Self {
            line,
            where_,
            msg: msg.into(),
        }
    }

    pub fn runtime_error(token: &Token, msg: impl Into<String>) -> Self {
        Self::new(token.line, token.lexeme.clone(), msg)
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error {}: {}", self.line, self.where_, self.msg);
    }
}

impl Error for LoxError {}
