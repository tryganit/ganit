pub mod ast;
pub mod tokens;

pub use ast::Expr;
use crate::types::ParseError;

pub fn parse(_formula: &str) -> Result<Expr, ParseError> {
    todo!("implemented in Task 4")
}

pub fn validate(formula: &str) -> Result<(), ParseError> {
    parse(formula).map(|_| ())
}
