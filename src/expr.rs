use std::fmt::Binary;

use crate::scanner::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    LiteralExpr {
        literal: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, op, right } => {
                return format!(
                    "({} ({} {}))",
                    op.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expr::Grouping { expression } => return format!("(group {})", expression.to_string()),
            Expr::LiteralExpr { literal } => return format!("{}", literal.to_string()),
            Expr::Unary { operator, right } => {
                return format!("({} {})", operator.lexeme, right.to_string())
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::{Expr, LiteralValue};

    #[test]
    fn test_pretty_print() {
        let minus_expr = Expr::Unary {
            operator: super::Token {
                token_type: super::TokenType::MINUS,
                lexeme: "-".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(Expr::LiteralExpr {
                literal: LiteralValue::FValue(2.0),
            }),
        };
        minus_expr.print();

        let operation = Expr::Binary {
            left: Box::new(minus_expr.clone()),
            op: super::Token {
                token_type: super::TokenType::STAR,
                lexeme: "*".to_string(),
                literal: None,
                line_number: 0,
            },
            right: Box::new(minus_expr),
        };

        operation.print();
    }
}
