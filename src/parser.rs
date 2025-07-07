use crate::err::LoxError;
use crate::scanner::{Literal, Token, TokenType};

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    // Unary are always Right associates
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
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
pub struct Ast {}

pub struct Parser {
    tokens: Vec<Token>,
    _current: usize,

    statements: Vec<Stmt>,
}
pub type ParseResult<T> = Result<T, LoxError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            _current: 0,

            statements: Vec::new(),
        }
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

    fn matches(&mut self, types: &[TokenType]) -> bool {
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
            Err(LoxError {
                line: 1,
                where_: "Consume:UnknownError".to_string(),
                msg: msg.to_string(),
            })
        }
    }
}

// Expressions
impl Parser {
    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;
        if self.matches(&[TokenType::Equal]) {
            let eq = self.prev().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(name) => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::from(value),
                    });
                }
                _ => {
                    return Err(LoxError::new(
                        eq.line,
                        eq.lexeme.clone(),
                        "Invalid assignment target.",
                    ));
                }
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
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
        while self.matches(&[
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

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
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

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
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
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
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

    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.prev().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;
        while self.matches(&[TokenType::And]) {
            let operator = self.prev().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.matches(&[TokenType::Number, TokenType::String]) {
            let prev: Token = self.prev().clone();

            return match prev.literal {
                Some(l) => Ok(Expr::Literal(l)),
                None => Err(LoxError {
                    line: prev.line,
                    where_: "ParseError".to_string(),
                    msg: "No literal value".to_string(),
                }),
            };
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _ = self.consume(&TokenType::RightParen, "Expect ')' after expression.");

            // todo: add validation if its already inside the grouping.

            let grp = Expr::Grouping(Box::new(expr));
            return Ok(grp);
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.prev().clone()));
        }

        let token = self.peek();

        Err(LoxError {
            line: token.line,
            where_: format!("Unexpected token `{}` at: {}", token.lexeme, self._current),
            msg: "ExpectedExpression".to_string(),
        })
    }
}

// Statements and declarations
impl Parser {
    pub fn parse(mut self) -> ParseResult<Vec<Stmt>> {
        while !self.is_eof() {
            let stmt = self.declaration();
            self.statements.push(stmt);
        }
        Ok(self.statements)
    }

    fn declaration(&mut self) -> Stmt {
        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self
            .consume(&TokenType::Identifier, "Expect variable name")
            .unwrap()
            .clone();
        let mut initializer: Option<Expr> = None;

        if self.matches(&[TokenType::Equal]) {
            initializer = Some(self.expression().unwrap());
        }

        _ = self.consume(&TokenType::Semicolon, "Expect ';' after expression.");

        Stmt::Var {
            name,
            init: initializer,
        }
    }

    fn statement(&mut self) -> Stmt {
        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.matches(&[TokenType::LeftBrace]) {
            return Stmt::Block {
                stmts: self.block(),
            };
        }

        return self.expression_statement();
    }

    fn if_statement(&mut self) -> Stmt {
        _ = self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.");
        let cond = self.expression().unwrap();
        _ = self.consume(&TokenType::RightParen, "Expect ')' after if condition.");

        let then_br = self.statement();
        let mut else_br = None;
        if self.matches(&[TokenType::Else]) {
            else_br = Some(Box::new(self.statement()));
        }

        Stmt::If {
            cond,
            then_br: Box::new(then_br),
            else_br,
        }
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_eof() {
            statements.push(self.declaration());
        }
        _ = self.consume(&TokenType::RightBrace, "Expect '}' after block.");
        statements
    }

    fn expression_statement(&mut self) -> Stmt {
        let expression = self.expression().unwrap();
        _ = self.consume(&TokenType::Semicolon, "Expect ';' after expression.");
        Stmt::Expression(expression)
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression().unwrap();
        _ = self.consume(&TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print(value)
    }
}

impl Parser {
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_eof() {
            if self.prev().type_ == TokenType::Semicolon {
                return;
            }
            if !matches!(
                self.peek().type_,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                self.advance();
            }
        }
    }
}
