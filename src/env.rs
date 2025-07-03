use std::collections::HashMap;

use crate::{err::LoxError, interpreter::Value, scanner::Token};

#[derive(Clone)]
pub struct Env {
    pub enclosing: Option<Box<Env>>,
    pub values: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.values.insert(name, value.unwrap_or_default());
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), LoxError> {
        let var_name = name.lexeme.clone();
        if self.values.contains_key(&var_name) {
            self.values.insert(var_name, value);
            return Ok(());
        }

        if let Some(ref mut outer) = self.enclosing {
            return outer.assign(name, value);
        }
        Err(LoxError::runtime_error(name, "Undefined variable."))
    }

    pub fn get(&self, name: &Token) -> Option<&Value> {
        match &self.enclosing {
            Some(outer) => outer.get(name),
            None => self.values.get(&name.lexeme),
        }
    }

    pub fn from_enclosing(outer: Env) -> Self {
        Env {
            values: HashMap::new(),
            enclosing: Some(Box::new(outer)),
        }
    }
}
