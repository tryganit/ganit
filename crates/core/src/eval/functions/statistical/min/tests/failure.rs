use super::super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn min_error_propagates() {
    assert_eq!(
        min_fn(&[Value::Number(1.0), Value::Error(ErrorKind::Ref), Value::Number(5.0)]),
        Value::Error(ErrorKind::Ref)
    );
}

#[test]
fn min_first_error_wins() {
    assert_eq!(
        min_fn(&[Value::Error(ErrorKind::Ref), Value::Error(ErrorKind::Name)]),
        Value::Error(ErrorKind::Ref)
    );
}

#[test]
fn min_ignores_bool() {
    // Bool values are ignored; only Number(2) counts
    assert_eq!(
        min_fn(&[Value::Bool(true), Value::Number(2.0), Value::Bool(false)]),
        Value::Number(2.0)
    );
}
