use crate::{
    env::Env,
    err::LoxError,
    parser::{Expr, Stmt},
    scanner::{Literal, Token, TokenType},
};

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    Str(String),
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
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

    fn to_string(&self) -> String {
        match self {
            Self::Nil => "<NIL>".to_string(),
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Str(s) => s.clone(),
        }
    }
}

pub struct Interpreter {
    pub ast: Vec<Stmt>,

    env: Env,
}

impl Interpreter {
    pub fn new(ast: Vec<Stmt>) -> Self {
        Self {
            ast,
            env: Env::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), LoxError> {
        while let Some(stmt) = self.ast.pop() {
            self.execute(&stmt)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Var { name, init } => {
                let value = match init {
                    Some(expr) => Some(self.evaluate(expr)?),
                    None => None,
                };
                self.env.define(name.lexeme.clone(), value);
            }
            Stmt::Block { stmts } => {
                let nested = Env::from_enclosing(self.env.clone());
                self.execute_block(stmts.clone(), nested)?;
            }

            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("Print: {:?}", value);
            }
            _ => unimplemented!("Not yet"),
        }
        Ok(())
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, mut new_env: Env) -> Result<(), LoxError> {
        std::mem::swap(&mut self.env, &mut new_env);
        for stmt in statements {
            self.execute(&stmt)?;
        }
        std::mem::swap(&mut self.env, &mut new_env);
        Ok(())
    }

    pub fn ast(&self) -> &[Stmt] {
        &self.ast
    }

    pub fn evaluate(&mut self, expr: &Expr) -> RuntimeResult {
        match expr {
            Expr::Literal(lit) => {
                return Ok(lit.evaluate());
            }
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
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
            Expr::Assign { name, value } => {
                let val = self.evaluate(&value)?;
                self.env.assign(name, val.clone())?;
                Ok(val)
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

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
            Expr::Grouping(expr) => {
                return self.evaluate(expr);
            }
            Expr::Variable(token) => {
                // todo: Am I really need to clone the value? I assume when the value is returned
                // it should be operated as original
                if let Some(v) = self.env.get(&token) {
                    return Ok(v.clone());
                } else {
                    return Err(LoxError::new(
                        token.line,
                        token.lexeme.clone(),
                        "Undefined variable",
                    ));
                }
            }
        }
    }
}

impl Expr {}

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
