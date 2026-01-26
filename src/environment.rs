use std::collections::HashMap;

use crate::{Token, error::ReefError, expr::ExprKind};

struct Environment {
    values: HashMap<String, ExprKind>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn get(&self, name: &Token) -> Result<&ExprKind, ReefError> {
        if let Some(name) = self.values.get(&name.lexeme) {
            return Ok(name);
        };
        Err(ReefError::reef_runtime_error(
            name,
            &format!("Undefined variable: {}.", name.lexeme),
        ))
    }
}
