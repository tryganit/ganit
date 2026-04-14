use std::collections::HashMap;
use crate::types::Value;

/// Holds variable bindings for formula evaluation.
pub struct Context {
    pub vars: HashMap<String, Value>,
}

impl Context {
    pub fn new(vars: HashMap<String, Value>) -> Self {
        let normalized = vars.into_iter()
            .map(|(k, v)| (k.to_uppercase(), v))
            .collect();
        Self { vars: normalized }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Value;

    #[test]
    fn get_case_insensitive() {
        let vars = [("myVar".to_string(), Value::Number(42.0))].into();
        let ctx = Context::new(vars);
        // All casing variants should find the same value
        assert_eq!(ctx.get("myVar"), Value::Number(42.0));
        assert_eq!(ctx.get("MYVAR"), Value::Number(42.0));
        assert_eq!(ctx.get("myvar"), Value::Number(42.0));
    }

    #[test]
    fn get_missing_returns_empty() {
        let ctx = Context::empty();
        assert_eq!(ctx.get("x"), Value::Empty);
    }
}
