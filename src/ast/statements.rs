use crate::{ast::expressions::Expr, lexer::token::Token};

pub enum Stmt {
    /// Defines local scope
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        /// Must be Expr::Variable
        superclass: Expr,
        /// Must be Stmt::Function
        methods: Box<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Expr,
    },
    Variable {
        name: Token,
        intialiser: Expr,
    },
    While {
        conditon: Expr,
        body: Box<Stmt>,
    },
}
