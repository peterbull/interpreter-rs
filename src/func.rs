use crate::expr::Value;
use crate::{Literal, Token, TokenType, error::ReefError, interpreter::Interpreter};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub func: fn(&Interpreter, Vec<Value>) -> Result<Value, ReefError>,
}

struct ReefFunction {
    arity: usize,
}

pub trait ReefCallable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, ReefError>;
}

impl ReefCallable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, ReefError> {
        (self.func)(interpreter, arguments)
    }
}
