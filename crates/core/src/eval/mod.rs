pub mod context;
pub mod coercion;
pub mod functions;

pub use context::Context;

use crate::parser::ast::Expr;
use crate::types::Value;

/// Placeholder — implemented in Task 7.
pub fn evaluate_expr(_expr: &Expr, _ctx: &mut Context) -> Value {
    todo!("implemented in Task 7")
}
