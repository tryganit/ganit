use crate::eval::coercion::to_bool;
use crate::eval::functions::EvalCtx;
use crate::eval::evaluate_expr;
use crate::parser::ast::Expr;
use crate::types::{ErrorKind, Value};

pub fn if_fn(args: &[Expr], ctx: &mut EvalCtx<'_>) -> Value {
    if args.len() < 2 || args.len() > 3 {
        return Value::Error(ErrorKind::Value);
    }
    let condition = evaluate_expr(&args[0], ctx);
    let cond_bool = match to_bool(condition) {
        Ok(b) => b,
        Err(e) => return e,
    };
    if cond_bool {
        evaluate_expr(&args[1], ctx)
    } else if args.len() == 3 {
        evaluate_expr(&args[2], ctx)
    } else {
        Value::Bool(false)
    }
}

#[cfg(test)]
mod tests;
