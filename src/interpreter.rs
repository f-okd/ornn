use core::num;
use std::fmt;

use crate::{
    ast::expressions::Expr,
    token::{Literal, Token, TokenType},
};

/// Values computed/stored at runtime.
#[derive(Debug, PartialEq)]
pub enum Value {
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {};
    }

    pub fn evaluate_expression(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal { value } => match value {
                Literal::Nil => Ok(Value::Nil),
                Literal::Bool(value) => Ok(Value::Bool(*value)),
                Literal::Number(number) => Ok(Value::Number(*number)),
                Literal::Str(str) => Ok(Value::String(str.clone())),
            },
            Expr::Grouping { expression } => self.evaluate_expression(expression),
            Expr::Unary { operator, right } => {
                let right = self.evaluate_expression(right)?;
                match operator.token_type {
                    // Subexpression be a number, we're verifying at runtime. (Dynamic typing)
                    TokenType::MINUS => match right {
                        Value::Number(num) => Ok(Value::Number(-num)),
                        _ => {
                            return Err(RuntimeError {
                                token: operator.clone(),
                                message: String::from("Operand must be a number"),
                            });
                        }
                    },
                    TokenType::BANG => Ok(Value::Bool(!right.is_truthy())),
                    _ => todo!(),
                }
            }
            Expr::Binary {
                left,
                right,
                operator,
            } => {
                let left = self.evaluate_expression(left)?;
                let right = self.evaluate_expression(right)?;

                match operator.token_type {
                    // Additional equality branch for mixed type configurations of left and right
                    TokenType::EQUAL_EQUAL => return Ok(Value::Bool(left == right)),
                    TokenType::BANG_EQUAL => return Ok(Value::Bool(left != right)),
                    _ => (),
                }

                match (&left, &right) {
                    (Value::Number(l), Value::Number(r)) => match operator.token_type {
                        TokenType::MINUS => {
                            return Ok(Value::Number(l - r));
                        }
                        TokenType::SLASH => {
                            return Ok(Value::Number(l / r));
                        }
                        TokenType::STAR => {
                            return Ok(Value::Number(l * r));
                        }
                        TokenType::PLUS => {
                            return Ok(Value::Number(l + r));
                        }
                        TokenType::GREATER => {
                            return Ok(Value::Bool(l > r));
                        }
                        TokenType::GREATER_EQUAL => {
                            return Ok(Value::Bool(l >= r));
                        }
                        TokenType::LESS => {
                            return Ok(Value::Bool(l < r));
                        }
                        TokenType::LESS_EQUAL => {
                            return Ok(Value::Bool(l <= r));
                        }
                        _ => unreachable!(),
                    },
                    (Value::String(l), Value::String(r)) => match operator.token_type {
                        TokenType::PLUS => {
                            let temp = format!("{}{}", l, r);
                            return Ok(Value::String(temp));
                        }
                        _ => {
                            return Err(RuntimeError {
                                token: operator.clone(),
                                message: String::from("Unsupported string operation"),
                            });
                        }
                    },
                    _ => {
                        return Err(RuntimeError {
                            token: operator.clone(),
                            message: format!(
                                "Operation '{:?}' unsupported for given operands '{}', '{}'",
                                operator, left, right
                            ),
                        });
                    }
                }
            }
            _ => Ok(Value::Nil),
        }
    }
}
