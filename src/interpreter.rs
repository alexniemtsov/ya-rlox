use std::str::FromStr;

use crate::{
    err::LoxError,
    parser::Expr,
    scanner::{Literal, Token, TokenType},
};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
    Nil,
}

impl Value {
    fn is_equal(&self, v: &Value) -> bool {
        match (self, v) {
            (Self::Number(l), Self::Number(r)) => {
                return l == r;
            }
            (Self::Bool(l), Self::Bool(r)) => {
                return l == r;
            }
            (Self::Str(l), Self::Str(r)) => {
                return l == r;
            }
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

pub struct Interpreter {}

impl Interpreter {}

pub type RuntimeResult = Result<Value, LoxError>;

impl LoxError {
    pub fn at_token(token: &Token, msg: impl Into<String>) -> Self {
        LoxError::new(token.line, token.lexeme.clone(), msg)
    }
}

impl Literal {
    pub fn evaluate(&self) -> Value {
        match self {
            Self::Nil => Value::Nil,
            Self::Boolean(v) => Value::Bool(*v),
            Self::String(v) => Value::Str(v.clone()),
            Self::Integer(v) => Value::Number(*v as f64),
            Self::Float(v) => Value::Number(*v),
        }
    }
}

impl Expr {
    pub fn evaluate(&self) -> RuntimeResult {
        match self {
            Expr::Literal(lit) => {
                return Ok(lit.evaluate());
            }
            Self::Unary { operator, right } => {
                let right = right.evaluate()?;
                match (&operator.type_, right) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenType::Minus, _) => Err(LoxError::at_token(
                        operator,
                        "Operand of '-' must be a number",
                    )),

                    (TokenType::Bang, Value::Bool(b)) => Ok(Value::Bool(b)),
                    (TokenType::Bang, Value::Nil) => Ok(Value::Bool(true)),
                    (TokenType::Bang, _) => Err(LoxError::at_token(
                        operator,
                        "Operand of '!' must be a logical expression",
                    )),

                    (_, _) => unreachable!("Invalid unary operator"),
                }
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&operator.type_, left, right) {
                    (TokenType::Minus, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    }
                    (TokenType::Plus, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l + r))
                    }
                    (TokenType::Plus, Value::Str(l), Value::Str(r)) => {
                        l.to_owned().push_str(r.as_str());
                        Ok(Value::Str(l))
                    }
                    (TokenType::Plus, _, _) => Err(LoxError::at_token(
                        operator,
                        "Operand must be number or str",
                    )),
                    (TokenType::Slash, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l / r))
                    }
                    (TokenType::Star, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    }

                    (TokenType::Greater, Value::Number(l), Value::Number(r)) => {
                        return Ok(Value::Bool(l > r));
                    }
                    (TokenType::GreaterEqual, Value::Number(l), Value::Number(r)) => {
                        return Ok(Value::Bool(l >= r));
                    }
                    (TokenType::Less, Value::Number(l), Value::Number(r)) => {
                        return Ok(Value::Bool(l < r));
                    }
                    (TokenType::LessEqual, Value::Number(l), Value::Number(r)) => {
                        return Ok(Value::Bool(l <= r));
                    }

                    (TokenType::BangEqual, _l, _r) => {
                        return Ok(Value::Bool(!_l.is_equal(&_r)));
                    }

                    (TokenType::EqualEqual, _l, _r) => {
                        return Ok(Value::Bool(_l.is_equal(&_r)));
                    }

                    (
                        TokenType::Slash
                        | TokenType::Minus
                        | TokenType::Star
                        | TokenType::Less
                        | TokenType::LessEqual
                        | TokenType::Greater
                        | TokenType::GreaterEqual,
                        _,
                        _,
                    ) => Err(LoxError::at_token(operator, "Operand must be number")),

                    _ => unreachable!("Invalid binary operator"),
                }
            }
            Self::Grouping(expr) => {
                return expr.evaluate();
            }
            Self::Variable(_) => {
                return Ok(Value::Nil);
            }
            Self::Assign { name, value } => {
                return Ok(Value::Nil);
            }
        }
    }

    pub fn accept() {}
}
