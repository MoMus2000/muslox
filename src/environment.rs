use std::{collections::HashMap, hash::Hash};

use crate::{LiteralValue, LoxErr};

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
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
}
