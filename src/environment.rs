use std::collections::HashMap;

use crate::{Token, expr::Value};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: &str, value: &Value) -> Value {
        self.values
            .insert(name.to_string(), value.clone())
            .expect("should always define a value, even nil")
    }
    pub fn get(&self, name: &Token) -> Value {
        self.values
            .get(&name.lexeme)
            .expect("env should always be able to get this value")
            .clone()
    }
}
impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
