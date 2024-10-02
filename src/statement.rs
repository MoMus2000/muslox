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
    Assert {
        expression_a: Expr,
    },
    Block {
        statements: Vec<Statement>,
    },
    If {
        conditional: Expr,
        happy_path: Box<Statement>,
        sad_path: Option<Box<Statement>>,
    },
}

impl Statement {}
