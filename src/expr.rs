#![allow(unused_variables, dead_code)]

use crate::{
    Literal, Token, TokenType,
    error::{LoxError, lox_general_error},
};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    // funcs etc to be added
}
impl Value {
    pub fn as_number(&self) -> Result<f64, LoxError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(lox_general_error(&format!(
                "Expected number, got {:?}",
                self
            ))),
        }
    }
    pub fn as_string(&self) -> Result<&str, LoxError> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(lox_general_error(&format!(
                "Expected string, got {:?}",
                self
            ))),
        }
    }
    pub fn as_boolean(&self) -> Result<bool, LoxError> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(lox_general_error(&format!(
                "Expected boolean, got {:?}",
                self
            ))),
        }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Assign {
        name: Token,
        value: Box<ExprKind>,
    },
    Binary {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Call {
        callee: Box<ExprKind>,
        token: Token,
        arguments: Vec<ExprKind>,
    },
    Get {
        object: Box<ExprKind>,
        name: Token,
    },
    Grouping {
        expression: Box<ExprKind>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Set {
        object: Box<ExprKind>,
        name: Token,
        value: Box<ExprKind>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<ExprKind>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Debug)]
pub struct Expr {}
impl Expr {
    pub fn evaluate(data: &ExprKind) -> Result<Value, LoxError> {
        match data {
            // ExprKind::Assign { name, value } => {}
            ExprKind::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = Expr::evaluate(left)?;
                let right_val = Expr::evaluate(right)?;
                match (&left_val, operator.token_type, &right_val) {
                    (Value::Number(left_val), TokenType::Plus, Value::Number(right_val)) => {
                        let addition_result = left_val + right_val;
                        Ok(Value::Number(addition_result))
                    }
                    (Value::Number(left_val), TokenType::Minus, Value::Number(right_val)) => {
                        let subtraction_result = left_val - right_val;
                        Ok(Value::Number(subtraction_result))
                    }
                    (Value::Number(left_val), TokenType::Star, Value::Number(right_val)) => {
                        let multiplication_result = left_val * right_val;
                        Ok(Value::Number(multiplication_result))
                    }
                    (Value::Number(left_val), TokenType::Slash, Value::Number(right_val)) => {
                        let division_result = left_val / right_val;
                        Ok(Value::Number(division_result))
                    }

                    _ => Err(lox_general_error("Binary evaluation error")),
                }
            }
            // ExprKind::Call {
            //     callee,
            //     token,
            //     arguments,
            // } => {}
            // ExprKind::Get { object, name } => {}
            ExprKind::Grouping { expression } => Expr::evaluate(expression),
            ExprKind::Literal { value } => Ok(match value {
                Literal::String(s) => Value::String(s.clone()),
                Literal::Number(n) => Value::Number(*n),
                Literal::Boolean(b) => Value::Boolean(*b),
                Literal::Nil => Value::Nil,
            }),
            // ExprKind::Logical {
            //     left,
            //     operator,
            //     right,
            // } => {}
            // ExprKind::Set {
            //     object,
            //     name,
            //     value,
            // } => {}
            // ExprKind::Super { keyword, method } => {}
            // ExprKind::This { keyword } => {}
            ExprKind::Unary { operator, right } => {
                let right_val = Expr::evaluate(right)?;
                match (operator.token_type, &right_val) {
                    (TokenType::Minus, Value::Number(right_val)) => Ok(Value::Number(-right_val)),
                    _ => Err(lox_general_error("Unary evaluation error")),
                }
            }
            // ExprKind::Variable { name } => {}
            _ => Ok(Value::Nil), // placeholder to keep rust happy while i fill out the others
        }
    }
}
