use crate::scanner::Token;

enum ExprType {
    Literal,
    Unary,
    Binary,
    Groupping,
}
#[derive(Debug)]
pub struct Ast {}

pub struct Parser {
    tokens: Vec<Token>,
    _current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            _current: 0,
        }
    }

    pub fn parse(self) -> Ast {
        Ast {}
    }
}
