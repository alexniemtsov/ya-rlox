use crate::scanner::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    // Unary are always Right associates
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

#[derive(Debug)]
pub enum ParseError {
    Unknown,
    InvalidSymbol,
    ExpectedExpression,
    MissingLiteral(),
}

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
                right: Box::new(Expr::Literal(Literal::Integer(123))),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Float(
                22.35,
            ))))),
        }
    }

    pub fn parse(mut self) -> ParseResult<Expr> {
        self.expression()
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
            Err(ParseError::Unknown)
        }
    }
}

// Expressions
impl Parser {
    fn expression(&mut self) -> ParseResult<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            // todo: need to figure out should I .clone() it or not
            let operator: Token = self.prev().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;
        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.prev().clone();
            let right: Expr = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.prev().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.prev().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.prev().clone();
            let right = self.unary()?;
            let expr = Expr::Unary {
                operator,
                right: Box::new(right),
            };
            return Ok(expr);
        }
        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        if self.matches(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.matches(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.matches(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.matches(vec![TokenType::Number, TokenType::String]) {
            let prev: Token = self.prev().clone();

            let lit = prev.literal.ok_or(ParseError::MissingLiteral())?;

            return Ok(Expr::Literal(lit));
        }

        if self.matches(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _ = self.consume(&TokenType::RightParen, "Expect ')' after expression.");

            // todo: add validation if its already inside the grouping.

            let grp = Expr::Grouping(Box::new(expr));
            return Ok(grp);
        }

        Err(ParseError::ExpectedExpression)
    }
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
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_eof() {
            if self.prev().type_ == TokenType::Semicolon {
                return;
            }
            let acc = matches!(
                self.peek().type_,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            );

            if !acc {
                self.advance();
            }
        }
    }
}
