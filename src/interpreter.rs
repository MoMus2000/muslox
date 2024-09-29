use std::collections::HashMap;

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

                    println!("LOCAL STORAGE = {:?}", self.local_var);
                }
                Statement::Expression { mut expression } => {
                    let expr = expression.evaluate(&self.local_var)?;
                }
                Statement::Print { mut expression } => {
                    let val = expression.evaluate(&self.local_var)?;
                    println!("> {:?}", val);
                }
            }
        }
        Ok(())
    }
}
