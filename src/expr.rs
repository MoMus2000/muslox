use crate::{environment::Environment, scanner::*, LoxErr};
use std::cell::RefCell;
use std::rc::Rc;

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
    Var {
        identifier: String,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Logical { left, op, right } => {
                return format!("{} {} {}", op.lexeme, left.to_string(), right.to_string())
            }
            Expr::Binary { left, op, right } => {
                return format!("({} {} {})", op.lexeme, left.to_string(), right.to_string())
            }
            Expr::Grouping { expression } => return format!("(group {})", expression.to_string()),
            Expr::LiteralExpr { literal } => return format!("{}", literal.to_string()),
            Expr::Unary { operator, right } => {
                return format!("({} {})", operator.lexeme, right.to_string())
            }
            Expr::Var { identifier } => {
                return format!("var {} ", identifier);
            }
            Expr::Assignment { name, value } => {
                return format!("var {} = {}", name, value.to_string())
            }
        }
    }

    pub fn evaluate(
        &mut self,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<LiteralValue, LoxErr> {
        match self {
            Expr::Logical { left, op, right } => {
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment)?;

                match op.token_type {
                    TokenType::AND => {
                        let bool = left.to_boolean() && right.to_boolean();
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    TokenType::OR => {
                        let bool = left.to_boolean() || right.to_boolean();
                        match bool {
                            true => Ok(LiteralValue::True),
                            false => Ok(LiteralValue::False),
                        }
                    }
                    _ => Err("Invalid token type for op".into()),
                }
            }
            Expr::Assignment { name, value } => {
                let value = value.evaluate(environment.clone())?;
                let assign_success = (*environment).borrow_mut().assign(name, value.clone());
                match assign_success {
                    true => return Ok(value),
                    false => return Err(format!("Variable {} has not been declared", name).into()),
                }
            }
            Expr::Var { identifier } => match (*environment).borrow().get(identifier.to_string()) {
                Ok(ident) => Ok(ident.clone()),
                Err(_) => {
                    let error = format!("Undefined Var {}", identifier);
                    Err(error.into())
                }
            },
            Expr::LiteralExpr { literal } => Ok(literal.clone()),
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(environment)?;
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
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment)?;

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

                        for _ in 0..y as usize {
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
        let _ = Expr::Binary {
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
