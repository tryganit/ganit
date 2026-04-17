use super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn basic_split() {
    let result = split_fn(&[Value::Text("a,b,c".into()), Value::Text(",".into())]);
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Text("a".into()),
            Value::Text("b".into()),
            Value::Text("c".into()),
        ])
    );
}

#[test]
fn no_args_returns_na() {
    assert_eq!(split_fn(&[]), Value::Error(ErrorKind::NA));
}

#[test]
fn one_arg_returns_na() {
    assert_eq!(split_fn(&[Value::Text("a,b".into())]), Value::Error(ErrorKind::NA));
}

#[test]
fn too_many_args_returns_na() {
    // SPLIT accepts up to 4 args; 5 is too many
    let r = split_fn(&[
        Value::Text("a,b".into()),
        Value::Text(",".into()),
        Value::Bool(true),
        Value::Bool(true),
        Value::Text("extra".into()),
    ]);
    assert_eq!(r, Value::Error(ErrorKind::NA));
}

#[test]
fn remove_empty_true_removes_empty_strings() {
    // SPLIT("a,,b", ",", TRUE, TRUE) → ["a", "b"]
    let result = split_fn(&[
        Value::Text("a,,b".into()),
        Value::Text(",".into()),
        Value::Bool(true),
        Value::Bool(true),
    ]);
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Text("a".into()),
            Value::Text("b".into()),
        ])
    );
}

#[test]
fn remove_empty_false_keeps_empty_strings() {
    // SPLIT("a,,b", ",", TRUE, FALSE) → ["a", "", "b"]
    let result = split_fn(&[
        Value::Text("a,,b".into()),
        Value::Text(",".into()),
        Value::Bool(true),
        Value::Bool(false),
    ]);
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Text("a".into()),
            Value::Text("".into()),
            Value::Text("b".into()),
        ])
    );
}

#[test]
fn split_by_each_false_treats_delimiter_as_unit() {
    // SPLIT("a::b", "::", FALSE) treats "::" as a single delimiter
    let result = split_fn(&[
        Value::Text("a::b".into()),
        Value::Text("::".into()),
        Value::Bool(false),
    ]);
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Text("a".into()),
            Value::Text("b".into()),
        ])
    );
}

#[test]
fn default_remove_empty_true() {
    // SPLIT("a,,b", ",") with only 2 args defaults to remove_empty=TRUE
    let result = split_fn(&[Value::Text("a,,b".into()), Value::Text(",".into())]);
    assert_eq!(
        result,
        Value::Array(vec![
            Value::Text("a".into()),
            Value::Text("b".into()),
        ])
    );
}
