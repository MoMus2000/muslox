use crate::expr::Expr;

#[derive(Debug)]
pub enum Statement {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        indentifier: String,
        expression: Expr,
    },
}

impl Statement {}
