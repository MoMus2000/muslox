use std::collections::HashMap;

use crate::{scanner::*, LoxErr};

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
    Assignment {
        identifier: String,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, op, right } => {
                return format!("({} {} {})", op.lexeme, left.to_string(), right.to_string())
            }
            Expr::Grouping { expression } => return format!("(group {})", expression.to_string()),
            Expr::LiteralExpr { literal } => return format!("{}", literal.to_string()),
            Expr::Unary { operator, right } => {
                return format!("({} {})", operator.lexeme, right.to_string())
            }
            Expr::Assignment { identifier } => {
                return format!("var {} ", identifier);
            }
        }
    }

    pub fn evaluate(
        &mut self,
        local_storage: &HashMap<String, LiteralValue>,
    ) -> Result<LiteralValue, LoxErr> {
        match self {
            Expr::Assignment { identifier } => match local_storage.get(identifier) {
                Some(ident) => Ok(ident.clone()),
                None => panic!("Undefined Var"),
            },
            Expr::LiteralExpr { literal } => Ok(literal.clone()),
            Expr::Grouping { expression } => expression.evaluate(local_storage),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(local_storage)?;
                match (right.clone(), operator.token_type.clone()) {
                    (LiteralValue::FValue(x), TokenType::MINUS) => {
                        return Ok(LiteralValue::FValue(-1.0 * x));
                    }
                    (_, TokenType::MINUS) => Err("Unable to negate this expression".into()),
                    (any, TokenType::BANG) => Ok(self.is_falsy(any)),
                    _ => panic!("Should not get to this point"),
                }
            }
            Expr::Binary { left, op, right } => {
                let left = left.evaluate(&local_storage)?;
                let right = right.evaluate(&local_storage)?;

                match (left, right, op.token_type.clone()) {
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::PLUS) => {
                        return Ok(LiteralValue::FValue(x + y));
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::MINUS) => {
                        return Ok(LiteralValue::FValue(x - y));
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::SLASH) => {
                        return Ok(LiteralValue::FValue(x / y));
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::STAR) => {
                        return Ok(LiteralValue::FValue(x * y));
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::GREATER) => {
                        let bool = x > y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::GREATEREQUAL) => {
                        let bool = x >= y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::LESS) => {
                        let bool = x < y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::LESSEQUAL) => {
                        let bool = x <= y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::EQUALEQUAL) => {
                        let boolean_res = x == y;
                        if boolean_res {
                            return Ok(LiteralValue::True);
                        }
                        Ok(LiteralValue::False)
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::EQUALEQUAL,
                    ) => {
                        let boolean_res = x == y;
                        if boolean_res {
                            return Ok(LiteralValue::True);
                        }
                        Ok(LiteralValue::False)
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::BANGEQUAL,
                    ) => {
                        let boolean_res = x != y;
                        if boolean_res {
                            return Ok(LiteralValue::True);
                        }
                        Ok(LiteralValue::False)
                    }
                    (LiteralValue::StringValue(x), LiteralValue::FValue(y), TokenType::STAR) => {
                        let mut concat = String::new();

                        for i in 0..y as usize {
                            concat.push_str(&x);
                        }

                        Ok(LiteralValue::StringValue(concat))
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::LESS,
                    ) => {
                        let bool = x < y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::GREATER,
                    ) => {
                        let bool = x > y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::GREATEREQUAL,
                    ) => {
                        let bool = x >= y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::LESSEQUAL,
                    ) => {
                        let bool = x <= y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (LiteralValue::FValue(x), LiteralValue::FValue(y), TokenType::BANGEQUAL) => {
                        let bool = x != y;
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    (
                        LiteralValue::StringValue(x),
                        LiteralValue::StringValue(y),
                        TokenType::PLUS,
                    ) => {
                        let mut concat = String::new();

                        concat.push_str(x.as_str());
                        concat.push_str(y.as_str());

                        Ok(LiteralValue::StringValue(concat))
                    }
                    _ => panic!("Should not get to this point"),
                }
            }
        }
    }

    fn is_falsy(&mut self, expr: LiteralValue) -> LiteralValue {
        match expr {
            LiteralValue::FValue(x) => {
                if x < 0.0 {
                    return LiteralValue::False;
                }
                LiteralValue::True
            }
            LiteralValue::StringValue(s) => {
                if s.len() == 0 {
                    return LiteralValue::True;
                }
                LiteralValue::False
            }
            LiteralValue::True => LiteralValue::False,
            LiteralValue::False => LiteralValue::True,
            LiteralValue::Nil => LiteralValue::True,
            _ => panic!("Should not reach this point"),
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
    }
}
