use std::collections::HashMap;

use crate::ast::Identifier;

use super::{Object, ObjectType};

pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Object>> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str, val: Box<dyn Object>) -> Option<&Box<dyn Object>> {
        self.store.insert(name.to_string(), val);
        self.store.get(name)
    }
}
