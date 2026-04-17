use crate::eval::evaluate_expr;
use crate::eval::functions::EvalCtx;
use crate::parser::ast::Expr;
use crate::types::{ErrorKind, Value};

/// `LET(name1, val1, ..., nameN, valN, body)` — bind named values and evaluate body.
///
/// Rules:
/// - Must have an odd number of arguments ≥ 3 (at least one binding + body).
/// - Name arguments must be identifier expressions.
/// - Bindings are evaluated in order; later bindings can reference earlier ones.
/// - Bindings are scoped to the body; context is restored after evaluation.
pub fn let_fn(args: &[Expr], ctx: &mut EvalCtx<'_>) -> Value {
    let count = args.len();
    // Need at least 3 args and an odd count (pairs of name/value + 1 body)
    if count < 3 || count.is_multiple_of(2) {
        return Value::Error(ErrorKind::NA);
    }

    let pair_count = (count - 1) / 2;
    let body = &args[count - 1];

    // Validate that all name slots are identifiers before binding anything.
    for i in 0..pair_count {
        match &args[i * 2] {
            Expr::Variable(_, _) => {}
            _ => return Value::Error(ErrorKind::Value),
        }
    }

    // Bind each name to its value in order (later bindings see earlier ones).
    let mut saved: Vec<(String, Option<Value>)> = Vec::with_capacity(pair_count);
    for i in 0..pair_count {
        let name = match &args[i * 2] {
            Expr::Variable(n, _) => n.to_uppercase(),
            _ => unreachable!(), // validated above
        };
        let val = evaluate_expr(&args[i * 2 + 1], ctx);
        let old = ctx.ctx.set(name.clone(), val);
        saved.push((name, old));
    }

    let result = evaluate_expr(body, ctx);

    // Restore context in reverse order.
    for (name, old) in saved.into_iter().rev() {
        match old {
            Some(v) => { ctx.ctx.set(name, v); }
            None    => { ctx.ctx.remove(&name); }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{evaluate, Value};
    use crate::types::ErrorKind;

    #[test]
    fn let_single_binding() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LET(x, 5, x*2)", &vars), Value::Number(10.0));
    }

    #[test]
    fn let_two_bindings() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LET(x, 3, y, 4, x+y)", &vars), Value::Number(7.0));
    }

    #[test]
    fn let_later_binding_references_earlier() {
        let vars = HashMap::new();
        // y = x+1 = 3, x*y = 2*3 = 6
        assert_eq!(evaluate("=LET(x, 2, y, x+1, x*y)", &vars), Value::Number(6.0));
    }

    #[test]
    fn let_even_arg_count_returns_na() {
        let vars = HashMap::new();
        // Even number of args (2) — no body — should return #N/A
        assert_eq!(evaluate("=LET(x, 5)", &vars), Value::Error(ErrorKind::NA));
    }
}
