use crate::{
    error::LoxError,
    expr::{Expr, ExprKind, Value},
};

pub struct Interpreter {}
impl Interpreter {
    pub fn eval_expression(expr_kind: &ExprKind) -> Result<Value, LoxError> {
        Expr::evaluate(&expr_kind)
    }
}
