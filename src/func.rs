use crate::environment::Environment;
use crate::expr::Value;
use crate::stmt::StmtKind;
use crate::{Literal, Token, TokenType, error::ReefError, interpreter::Interpreter};
use std::{env, fmt};

pub type InterpreterFn = fn(&Interpreter, Vec<Value>) -> Result<Value, ReefError>;

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    name: Token,
    parameters: Vec<Token>,
    body: Vec<StmtKind>,
}

impl FunctionDecl {
    fn from_statement(stmt: StmtKind) -> Result<Self, ReefError> {
        match &stmt {
            StmtKind::Function {
                name,
                parameters,
                body,
            } => Ok(FunctionDecl {
                name: name.clone(),
                parameters: parameters.clone(),
                body: body.clone(),
            }),
            _ => Err(ReefError::reef_general_error(
                "expected stmtkind function for reef callable",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub func: InterpreterFn,
}

#[derive(Debug, Clone)]
pub struct ReefFunction {
    pub declaration: FunctionDecl,
    pub arity: usize,
    pub func: InterpreterFn,
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

impl ReefFunction {
    pub fn new(
        declaration: StmtKind,
        arity: usize,
        func: InterpreterFn,
    ) -> Result<Self, ReefError> {
        let declaration = FunctionDecl::from_statement(declaration)?;
        Ok(Self {
            declaration,
            arity,
            func,
        })
    }
}

impl ReefCallable for ReefFunction {
    fn arity(&self) -> usize {
        self.arity
    }
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, ReefError> {
        let decl = self.declaration;

        let globals = Some(interpreter.globals.clone());
        let environment = Environment::new(globals);
        // for i in self.declaration.parameters.len() {
        //     let param = decl.parameters.get(i);
        //     environment.define()
        // }
        (self.func)(interpreter, arguments)
    }
}
