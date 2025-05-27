use crate::scanner::{ScanLiteral, Token, TokenType};

// todo: merge with ScanLiteral
#[derive(Debug)]
pub enum ParserLiteral {
    Boolean(bool),
    Nil,
    ScanLiteral(ScanLiteral),
}

#[derive(Debug)]
pub enum Expr {
    Literal(ParserLiteral),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token, // should be TokenType
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),

    Variable(Token),
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        init: Option<Expr>,
    },
    Block {
        stmts: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_br: Box<Stmt>,
        else_br: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    Func {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
}

pub struct ParseError {}

#[derive(Debug)]
pub struct Ast {}

pub struct Parser {
    tokens: Vec<Token>,
    _current: usize,
}
pub type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            _current: 0,
        }
    }

    pub fn test(&self) -> Expr {
        Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal(ParserLiteral::ScanLiteral(
                    ScanLiteral::Integer(123),
                ))),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(
                ParserLiteral::ScanLiteral(ScanLiteral::Float(22.35)),
            )))),
        }
    }

    pub fn parse(mut self) -> ParseResult<Vec<Stmt>> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_eof() {
            // stmts.push(self.declaration()?);
        }
        Ok(stmts)
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

    fn check(&self, t: &TokenType) -> bool {
        !self.is_eof() && &self.peek().type_ == t
    }

    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        if types.iter().any(|t| self.check(t)) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, t: &TokenType, msg: &str) -> ParseResult<&Token> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(self.error(msg))
        }
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError {}
    }
}

// Expressions
impl Parser {
    // fn expression(&mut self) -> ParseResult<Expr> {
    //     self.assignment()
    // }
}

// Statements and declarations
impl Parser {
    // fn declaration(&mut self) -> ParseResult<Stmt> {
    //     if self.matches(vec![TokenType::Var]) {
    //         self.function() // todo: handle
    //     } else if self.matches(vec![TokenType::Var]) {
    //         self.var_declare()
    //     } else {
    //         self.statement()
    //     }
    // }
    //
    // fn statement(&self) -> ParseResult<Stmt> {
    //     //todo: match statement: expr, print, block, if, while
    // }
}

impl Parser {
    // fn eval(expr: &Expr) -> Value {
    //
    // }
}
