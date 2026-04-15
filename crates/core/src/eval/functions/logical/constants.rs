use crate::types::{ErrorKind, Value};

pub fn na_fn(args: &[Value]) -> Value {
    if !args.is_empty() {
        return Value::Error(ErrorKind::Value);
    }
    Value::Error(ErrorKind::NA)
}

pub fn true_fn(args: &[Value]) -> Value {
    if !args.is_empty() {
        return Value::Error(ErrorKind::NA);
    }
    Value::Bool(true)
}

pub fn false_fn(args: &[Value]) -> Value {
    if !args.is_empty() {
        return Value::Error(ErrorKind::NA);
    }
    Value::Bool(false)
}
