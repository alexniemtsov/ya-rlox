use crate::scanner::{Literal as TokenLit, Token, TokenType};

enum ParserLiteral {
    Boolean(bool),
    Nil,
    TokenLit(TokenLit),
}

enum Expr {
    Literal(ParserLiteral),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),

    Variable(Token),
    Assign {
        name: Token,
        value: Box<Expr>,
    },
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
        for t in self.tokens.iter() {
            println!("{:#?}", t);
        }
        Ast {}
    }

    fn peek(&self) -> &Token {
        &self.tokens[self._current]
    }

    fn prev(&self) -> &Token {
        &self.tokens[self._current - 1]
    }

    fn is_eof(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self._current += 1;
        }
        self.prev()
    }

    fn check(&self, t: TokenType) -> bool {
        !self.is_eof() && self.peek().type_ == t
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        if types.iter().any(|t| self.check(*t)) {
            self.advance();
            true
        } else {
            false
        }
    }
}
