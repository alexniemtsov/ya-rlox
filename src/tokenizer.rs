#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokkens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            type_,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "_T: type={:?} lex={} lit={:?} line={}",
            &self.type_, self.lexeme, self.literal, self.line
        )
    }
}
