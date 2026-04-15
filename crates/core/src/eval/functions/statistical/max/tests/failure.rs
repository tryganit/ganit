use super::super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn max_error_propagates() {
    assert_eq!(
        max_fn(&[Value::Number(1.0), Value::Error(ErrorKind::Ref), Value::Number(5.0)]),
        Value::Error(ErrorKind::Ref)
    );
}

#[test]
fn max_first_error_wins() {
    assert_eq!(
        max_fn(&[Value::Error(ErrorKind::Ref), Value::Error(ErrorKind::Name)]),
        Value::Error(ErrorKind::Ref)
    );
}

#[test]
fn max_ignores_bool() {
    // Bool values are ignored; only Number(10) counts
    assert_eq!(
        max_fn(&[Value::Bool(true), Value::Number(10.0), Value::Bool(false)]),
        Value::Number(10.0)
    );
}
