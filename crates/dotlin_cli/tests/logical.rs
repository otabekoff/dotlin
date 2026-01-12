use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::Expr;

fn span() -> std::ops::Range<usize> {
    0..0
}

#[test]
fn test_boolean_literals_and_not() {
    let mut i = Interpreter::new();
    let t = Expr::LitBool(true, span());
    let f = Expr::LitBool(false, span());
    let nott = Expr::Unary {
        op: "!".into(),
        expr: Box::new(t.clone()),
        span: span(),
    };
    let r = i.eval_expr(nott).unwrap();
    assert!(matches!(r, Value::Bool(false)));

    let notf = Expr::Unary {
        op: "!".into(),
        expr: Box::new(f.clone()),
        span: span(),
    };
    let r2 = i.eval_expr(notf).unwrap();
    assert!(matches!(r2, Value::Bool(true)));
}

#[test]
fn test_and_or_short_circuit() {
    let mut i = Interpreter::new();
    // true || (panic) should be true without evaluating right side
    let left = Expr::LitBool(true, span());
    let right = Expr::Binary {
        left: Box::new(Expr::LitBool(false, span())),
        op: "==".into(),
        right: Box::new(Expr::LitBool(true, span())),
        span: span(),
    };
    let or = Expr::Binary {
        left: Box::new(left.clone()),
        op: "||".into(),
        right: Box::new(right),
        span: span(),
    };
    let r = i.eval_expr(or).unwrap();
    assert!(matches!(r, Value::Bool(true)));

    // false && (expr) -> false without evaluating right
    let and = Expr::Binary {
        left: Box::new(Expr::LitBool(false, span())),
        op: "&&".into(),
        right: Box::new(Expr::LitBool(true, span())),
        span: span(),
    };
    let r2 = i.eval_expr(and).unwrap();
    assert!(matches!(r2, Value::Bool(false)));
}
