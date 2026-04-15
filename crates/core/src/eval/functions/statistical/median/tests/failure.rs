use super::super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn median_no_numeric_returns_num_error() {
    // MEDIAN() → #NUM!
    assert_eq!(median_fn(&[]), Value::Error(ErrorKind::Num));
}

#[test]
fn median_error_propagates() {
    assert_eq!(
        median_fn(&[Value::Number(1.0), Value::Error(ErrorKind::Ref), Value::Number(3.0)]),
        Value::Error(ErrorKind::Ref)
    );
}

#[test]
fn median_first_error_wins() {
    assert_eq!(
        median_fn(&[Value::Error(ErrorKind::Ref), Value::Error(ErrorKind::Name)]),
        Value::Error(ErrorKind::Ref)
    );
}
