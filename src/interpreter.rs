use core::num;

use crate::{
    ast::expressions::Expr,
    token::{Literal, TokenType},
};

/// Values computed/stored at runtime.
enum Value {
    Nil,
    String(String),
    Number(f64),
    Bool(bool),
}

impl Value {
    fn is_truthy(&self) -> bool {
        match &self {
            Value::Nil => false,
            Value::Bool(val) => *val,
            _ => true,
        }
    }
}

struct Interpreter {}

impl Interpreter {
    fn new() -> Interpreter {
        return Interpreter {};
    }

    fn evaluate_expression(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal { value } => match value {
                Literal::Nil => Value::Nil,
                Literal::Bool(value) => Value::Bool(*value),
                Literal::Number(number) => Value::Number(*number),
                Literal::Str(str) => Value::String(str.clone()),
            },
            Expr::Grouping { expression } => self.evaluate_expression(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate_expression(right);
                match operator.token_type {
                    // Subexpression be a number, we're verifying at runtime. (Dynamic typing)
                    TokenType::MINUS => match right {
                        Value::Number(num) => Value::Number(-num),
                        _ => {
                            panic!("Operand must be a number")
                        }
                    },
                    TokenType::BANG => Value::Bool(!right.is_truthy()),
                    _ => todo!(),
                }
            }
            _ => Value::Nil,
        }
    }
}
