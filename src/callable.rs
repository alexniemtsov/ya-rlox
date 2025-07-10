use core::fmt;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    env::Env,
    interpreter::{Interpreter, RuntimeError, Value},
    parser::Stmt,
};

pub trait Callable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError>;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declr: Vec<Stmt>,

    closure: Env,
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        return 0_usize;
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
        Ok(Value::Nil)
    }
}

#[derive(Debug)]
pub struct LoxClass {}

#[derive(Debug)]
pub struct Clock;
impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        Ok(Value::Number(now.as_secs_f64()))
    }
}
