use std::{collections::HashMap, fmt::format};

use crate::{expr::Expr, statement::Statement, LiteralValue, LoxErr};

pub struct Interpreter {
    local_var: HashMap<String, LiteralValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            local_var: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), LoxErr> {
        for stmt in statements {
            match stmt {
                Statement::Var {
                    indentifier,
                    mut expression,
                } => {
                    let result = expression.evaluate(&self.local_var)?;
                    self.local_var.insert(indentifier, result);
                }
                Statement::Expression { mut expression } => {
                    let expr = expression.evaluate(&self.local_var)?;
                }
                Statement::Print { mut expression } => {
                    let val = expression.evaluate(&self.local_var)?;
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
                    match expression_a.evaluate(&self.local_var) {
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
