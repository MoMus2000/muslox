use crate::{LiteralValue, LoxErr};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: String, literal: LiteralValue) {
        self.values.insert(name, literal);
    }

    pub fn get(&self, key: String) -> Result<LiteralValue, LoxErr> {
        let fetched_val = self.values.get(&key);
        match (fetched_val, &self.enclosing) {
            (Some(v), _) => Ok(v.clone()),
            (None, Some(v)) => Ok((*v).borrow().get(key)?),
            (None, None) => Err("No value found".into()),
        }
    }

    pub fn assign(&mut self, name: &str, value: LiteralValue) -> bool {
        let old_value = self.values.get(name);
        match (old_value, &mut self.enclosing) {
            (Some(_), _) => {
                self.values.insert(name.to_string(), value);
                true
            }
            (None, Some(env)) => (*env).borrow_mut().assign(name, value),
            (None, None) => false,
        }
    }
}
