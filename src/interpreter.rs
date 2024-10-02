use crate::{environment::Environment, statement::Statement, LiteralValue, LoxErr};

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
                Statement::If {
                    mut conditional,
                    happy_path,
                    sad_path,
                } => {
                    let res = conditional.evaluate(&mut self.env)?;
                    match res {
                        LiteralValue::True => self.interpret(vec![*happy_path]),
                        LiteralValue::False => {
                            if sad_path.is_some() {
                                self.interpret(vec![*sad_path.unwrap()])?;
                            }
                            return Ok(());
                        }
                        _ => return Err("Error should not ever get to this point".into()),
                    }?;
                }
                Statement::Block { statements } => {
                    let mut new_env = Environment::new();
                    new_env.enclosing = Some(Box::new(self.env.clone()));
                    let old_env = self.env.clone();
                    self.env = new_env;
                    let block_result = self.interpret(statements);
                    self.env = old_env;
                    block_result?
                }
                Statement::Var {
                    indentifier,
                    mut expression,
                } => {
                    let result = expression.evaluate(&mut self.env)?;
                    self.env.define(indentifier, result);
                }
                Statement::Expression { mut expression } => {
                    expression.evaluate(&mut self.env)?;
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
