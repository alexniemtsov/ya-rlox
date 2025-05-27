use std::error::Error;
use std::fmt;

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
    // todo: Change to `impl Into<String>
    pub fn new(line: usize, where_: String, msg: String) -> Self {
        Self { line, where_, msg }
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error {}: {}", self.line, self.where_, self.msg);
    }
}

impl Error for LoxError {}
