use crate::eval::functions::check_arity;
use crate::types::{ErrorKind, Value};

/// `INDIRECT(ref_text, [a1])` — returns the value of a cell reference given as a string.
/// In a standalone evaluator with no cell grid, always returns #REF!.
pub fn indirect_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 1, 2) {
        return err;
    }
    // No cell grid available → #REF! for any reference
    Value::Error(ErrorKind::Ref)
}
