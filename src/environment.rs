use std::collections::HashMap;

use crate::{LiteralValue, LoxErr};

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
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
        match fetched_val {
            Some(v) => Ok(v.clone()),
            None => Err("Calling undefined variable".into()),
        }
    }

    pub fn assign(&mut self, name: &str, value: LiteralValue) -> bool {
        let old_value = self.values.get(name);
        match (old_value, &mut self.enclosing) {
            (Some(_), _) => {
                self.values.insert(name.to_string(), value);
                true
            }
            (None, Some(env)) => env.assign(name, value),
            (None, None) => false,
        }
    }
}
