use crate::eval::coercion::to_bool;
use crate::eval::functions::{check_arity_len, EvalCtx};
use crate::eval::evaluate_expr;
use crate::parser::ast::Expr;
use crate::types::Value;

pub fn if_fn(args: &[Expr], ctx: &mut EvalCtx<'_>) -> Value {
    if let Some(err) = check_arity_len(args.len(), 2, 3) {
        return err;
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
