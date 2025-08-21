use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, Value},
    token::Token,
};

#[derive(Debug, Default)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Option<Value>) {
        self.values
            .insert(name.to_owned(), value.unwrap_or(Value::Nil));
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_owned(), value);
            Ok(())
        } else if let Some(enclosing_env) = &mut self.enclosing {
            enclosing_env.assign(name, value)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{name}'"),
            })
        }
    }

    pub fn get(&self, token: &Token) -> Result<Value, RuntimeError> {
        let value_option = self.values.get(&token.lexeme).cloned();

        match value_option {
            Some(value) => Ok(value),
            None => match &self.enclosing {
                Some(environment) => environment.get(token),
                None => Err(RuntimeError {
                    message: format!("Undefined variable {}.", token.lexeme),
                }),
            },
        }
    }
}
