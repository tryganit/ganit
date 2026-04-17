use crate::eval::functions::EvalCtx;
use crate::parser::ast::Expr;
use crate::types::{ErrorKind, Value};

/// Bare LAMBDA call without immediate invocation (e.g. `=LAMBDA()` or
/// `=LAMBDA(x,x*2)` stored in a cell but not called). Returns #N/A per
/// Google Sheets / Excel semantics.
///
/// Immediately-invoked LAMBDA (`=LAMBDA(x,x*2)(5)`) is handled by
/// `eval_apply` in `eval/mod.rs` via the `Expr::Apply` AST node.
pub fn lambda_fn(args: &[Expr], _ctx: &mut EvalCtx<'_>) -> Value {
    let _ = args;
    Value::Error(ErrorKind::NA)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{evaluate, Value};
    use crate::types::ErrorKind;

    #[test]
    fn lambda_single_arg() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LAMBDA(x, x*2)(5)", &vars), Value::Number(10.0));
    }

    #[test]
    fn lambda_two_args() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LAMBDA(x, y, x+y)(3, 4)", &vars), Value::Number(7.0));
    }

    #[test]
    fn lambda_square() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LAMBDA(x, x*x)(3)", &vars), Value::Number(9.0));
    }

    #[test]
    fn lambda_absolute_value_via_if() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LAMBDA(x, IF(x>0, x, -x))(-5)", &vars), Value::Number(5.0));
    }

    #[test]
    fn lambda_bare_returns_na() {
        let vars = HashMap::new();
        assert_eq!(evaluate("=LAMBDA(x, x*2)", &vars), Value::Error(ErrorKind::NA));
    }
}
