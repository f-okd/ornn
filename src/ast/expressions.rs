use crate::token::{Literal, Token};

pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: Token,
    },
    FunctionCall {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    /// Property access
    Get {
        object: Box<Expr>,
        name: Token,
    },
    /// Grouping expressions using parentheses
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    /// Logical AND, OR
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// Property assingment
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
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
        right: Box<Expr>,
    },
    /// Variable access expression
    Variable {
        name: Token,
    },
}
