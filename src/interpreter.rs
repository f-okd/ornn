use core::num;

use crate::{
    ast::expressions::Expr,
    token::{Literal, TokenType},
};

/// Values computed/stored at runtime.
#[derive(Debug, PartialEq)]
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
            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.evaluate_expression(left);
                let right = self.evaluate_expression(right);

                match operator.token_type {
                    // Additional equality branch for mixed type configurations of left and right
                    TokenType::EQUAL_EQUAL => return Value::Bool(left == right),
                    TokenType::BANG_EQUAL => return Value::Bool(left != right),
                    _ => (),
                }

                match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => match operator.token_type {
                        TokenType::MINUS => {
                            return Value::Number(l - r);
                        }
                        TokenType::SLASH => {
                            return Value::Number(l / r);
                        }
                        TokenType::STAR => {
                            return Value::Number(l * r);
                        }
                        TokenType::PLUS => {
                            return Value::Number(l + r);
                        }
                        TokenType::GREATER => {
                            return Value::Bool(l > r);
                        }
                        TokenType::GREATER_EQUAL => {
                            return Value::Bool(l >= r);
                        }
                        TokenType::LESS => {
                            return Value::Bool(l < r);
                        }
                        TokenType::LESS_EQUAL => {
                            return Value::Bool(l <= r);
                        }
                        _ => unreachable!(),
                    },
                    (Value::String(l), Value::String(r)) => match operator.token_type {
                        TokenType::PLUS => {
                            let temp = format!("{}{}", l, r);
                            return Value::String(temp);
                        }
                        _ => panic!("Unsupported string operation"),
                    },
                    _ => panic!(
                        "Operation '{:?}' unsupported for given operands '{:?}', '{:?}'",
                        operator, left, right
                    ),
                }
            }
            _ => Value::Nil,
        }
    }
}
