use crate::types::Value;

/// `MIN(value1, ...)` — smallest numeric value in the arguments.
/// Ignores Text, Bool, Empty. Error values propagate (first error returned).
/// Returns `Value::Number(0.0)` if no numeric values are present.
pub fn min_fn(args: &[Value]) -> Value {
    let mut result: Option<f64> = None;
    for arg in args {
        match arg {
            Value::Error(_) => return arg.clone(),
            Value::Number(n) => {
                result = Some(match result {
                    None => *n,
                    Some(cur) => cur.min(*n),
                });
            }
            _ => {}
        }
    }
    Value::Number(result.unwrap_or(0.0))
}

#[cfg(test)]
mod tests;
