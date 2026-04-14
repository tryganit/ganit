use std::collections::HashMap;
use crate::types::Value;

/// Holds variable bindings for formula evaluation.
pub struct Context {
    pub vars: HashMap<String, Value>,
}

impl Context {
    pub fn new(vars: HashMap<String, Value>) -> Self {
        Self { vars }
    }

    pub fn empty() -> Self {
        Self { vars: HashMap::new() }
    }

    /// Look up a variable by name (case-insensitive). Returns `Value::Empty` if not found.
    pub fn get(&self, name: &str) -> Value {
        self.vars
            .get(&name.to_uppercase())
            .cloned()
            .unwrap_or(Value::Empty)
    }
}
