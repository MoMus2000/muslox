use std::{collections::HashMap, fmt::format};

use crate::{environment::Environment, expr::Expr, statement::Statement, LiteralValue, LoxErr};

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), LoxErr> {
        for stmt in statements {
            match stmt {
                Statement::Var {
                    indentifier,
                    mut expression,
                } => {
                    let result = expression.evaluate(&mut self.env)?;
                    self.env.define(indentifier, result);
                }
                Statement::Expression { mut expression } => {
                    let expr = expression.evaluate(&mut self.env)?;
                }
                Statement::Print { mut expression } => {
                    let val = expression.evaluate(&mut self.env)?;
                    let val = match val {
                        LiteralValue::FValue(x) => format!("{}", x),
                        LiteralValue::False => format!("false"),
                        LiteralValue::True => format!("true"),
                        LiteralValue::StringValue(y) => format!("{}", y),
                        LiteralValue::Nil => format!("nil"),
                        _ => todo!(),
                    };
                    println!("{}", val);
                }
                Statement::Assert { mut expression_a } => {
                    match expression_a.evaluate(&mut self.env) {
                        Ok(res) => match res {
                            LiteralValue::True => {}
                            LiteralValue::False => {
                                panic!("Assertion Failed")
                            }
                            _ => panic!("Should not get to this point"),
                        },
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
