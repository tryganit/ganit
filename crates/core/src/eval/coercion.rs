use crate::display::display_number;
use crate::types::{ErrorKind, Value};

/// Coerce a Value to f64 for arithmetic. Returns `Err(Value::Error(...))` on failure.
pub fn to_number(v: Value) -> Result<f64, Value> {
    match v {
        Value::Number(n) => Ok(n),
        Value::Bool(b)   => Ok(if b { 1.0 } else { 0.0 }),
        Value::Empty     => Ok(0.0),
        Value::Text(s)   => s.parse::<f64>().map_err(|_| Value::Error(ErrorKind::Value)),
        Value::Error(_)  => Err(v),
        Value::Array(_)  => Err(Value::Error(ErrorKind::Value)),
    }
}

/// Coerce a Value to String for concatenation. Returns `Err(Value::Error(...))` on failure.
pub fn to_string_val(v: Value) -> Result<String, Value> {
    match v {
        Value::Text(s)  => Ok(s),
        Value::Number(n) => Ok(display_number(n)),
        Value::Bool(b)  => Ok(if b { "TRUE".to_string() } else { "FALSE".to_string() }),
        Value::Empty    => Ok(String::new()),
        Value::Error(_) => Err(v),
        Value::Array(_) => Err(Value::Error(ErrorKind::Value)),
    }
}

/// Coerce a Value to bool (for IF conditions). Returns `Err(Value::Error(...))` on failure.
pub fn to_bool(v: Value) -> Result<bool, Value> {
    match v {
        Value::Bool(b)  => Ok(b),
        Value::Number(n) => Ok(n != 0.0),
        Value::Error(_) => Err(v),
        Value::Text(_) | Value::Empty | Value::Array(_) => Err(Value::Error(ErrorKind::Value)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_to_number() {
        assert_eq!(to_number(Value::Bool(true)), Ok(1.0));
        assert_eq!(to_number(Value::Bool(false)), Ok(0.0));
    }

    #[test]
    fn text_to_number() {
        assert_eq!(to_number(Value::Text("5".into())), Ok(5.0));
        assert_eq!(to_number(Value::Text("abc".into())), Err(Value::Error(ErrorKind::Value)));
    }

    #[test]
    fn empty_to_number() {
        assert_eq!(to_number(Value::Empty), Ok(0.0));
    }

    #[test]
    fn error_propagates_through_to_number() {
        let err = Value::Error(ErrorKind::DivByZero);
        assert_eq!(to_number(err.clone()), Err(err));
    }

    #[test]
    fn number_to_string() {
        assert_eq!(to_string_val(Value::Number(1.5)), Ok("1.5".to_string()));
        assert_eq!(to_string_val(Value::Number(0.1 + 0.2)), Ok("0.3".to_string()));
    }

    #[test]
    fn bool_to_string() {
        assert_eq!(to_string_val(Value::Bool(true)), Ok("TRUE".to_string()));
        assert_eq!(to_string_val(Value::Bool(false)), Ok("FALSE".to_string()));
    }

    #[test]
    fn empty_to_string() {
        assert_eq!(to_string_val(Value::Empty), Ok(String::new()));
    }

    #[test]
    fn number_to_bool() {
        assert_eq!(to_bool(Value::Number(1.0)), Ok(true));
        assert_eq!(to_bool(Value::Number(0.0)), Ok(false));
    }

    #[test]
    fn text_to_bool_fails() {
        assert_eq!(to_bool(Value::Text("true".into())), Err(Value::Error(ErrorKind::Value)));
    }
}
