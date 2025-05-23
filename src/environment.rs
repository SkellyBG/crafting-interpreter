use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, Value},
    token::Token,
};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Option<Value>) {
        self.values
            .insert(name.to_owned(), value.unwrap_or(Value::Nil));
    }

    pub fn get(&self, token: Token) -> Result<Value, RuntimeError> {
        self.values
            .get(&token.lexeme)
            .map(|v| v.clone())
            .ok_or_else(|| RuntimeError {
                message: format!("Undefined variable {}.", token.lexeme),
            })
    }
}
