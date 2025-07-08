use std::collections::HashMap;

use crate::{err::LoxError, interpreter::Value, scanner::Token};

#[derive(Clone, Debug)]
pub struct Env {
    scopes: Vec<HashMap<String, Value>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, val: Value) {
        self.scopes
            .last_mut()
            .expect("should ways be at least 1 scope")
            .insert(name, val);
    }
    pub fn assign(&mut self, name: &Token, val: Value) -> Result<(), LoxError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name.lexeme) {
                scope.insert(name.lexeme.clone(), val);
                return Ok(());
            }
        }
        Err(LoxError::at_token(name, "Undefined variable"))
    }

    pub fn get(&self, name: &Token) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(&name.lexeme) {
                return Some(v.clone());
            }
        }
        None
    }
}

// pub struct ScopedEnv {
//     pub enclosing: Option<Box<Env>>,
//     pub values: HashMap<String, Value>,
//
// }

// impl Env {
//     pub fn new() -> Self {
//         Self {
//             values: HashMap::new(),
//             enclosing: None,
//         }
//     }
//
//     pub fn define(&mut self, name: String, value: Option<Value>) {
//         self.values.insert(name, value.unwrap_or_default());
//     }
//
//     pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), LoxError> {
//         let var_name = name.lexeme.clone();
//         if self.values.contains_key(&var_name) {
//             self.values.insert(var_name, value);
//             return Ok(());
//         }
//
//         if let Some(ref mut outer) = self.enclosing {
//             return outer.assign(name, value);
//         }
//         Err(LoxError::runtime_error(name, "Undefined variable."))
//     }
//
//     pub fn get(&self, name: &Token) -> Option<&Value> {
//         if self.values.contains_key(&name.lexeme) {
//             return self.values.get(&name.lexeme);
//         }
//         if let Some(outer) = &self.enclosing {
//             return outer.get(name);
//         }
//         None
//     }
//
//     pub fn from_enclosing(outer: Env) -> Self {
//         Env {
//             values: HashMap::new(),
//             enclosing: Some(Box::new(outer)),
//         }
//     }
// }
