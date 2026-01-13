use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::{Expr, Stmt};

fn span() -> std::ops::Range<usize> {
    0..0
}

#[test]
fn test_defaults_simple() {
    let mut i = Interpreter::new();
    // fun add(a = 1, b = 2): Int = a + b
    let expr = Expr::Binary {
        left: Box::new(Expr::Ident("a".into(), span())),
        op: "+".into(),
        right: Box::new(Expr::Ident("b".into(), span())),
        span: span(),
    };
    let body = vec![Stmt::Return(Some(expr.clone()), span())];
    i.register_fn(
        "add".into(),
        vec![
            ("a".into(), None, Some(Expr::LitNumber("1".into(), span()))),
            ("b".into(), None, Some(Expr::LitNumber("2".into(), span()))),
        ],
        body,
        None,
        Some("Int".into()),
    );

    let call = Expr::Call {
        callee: Box::new(Expr::Ident("add".into(), span())),
        args: vec![],
        span: span(),
    };
    let v = i.eval_expr(call).unwrap();
    assert!(matches!(v, Value::Number(n) if (n - 3.0).abs() < 1e-9));
}

#[test]
fn test_defaults_reference_previous() {
    let mut i = Interpreter::new();
    // fun f(a = 2, b = a): Int { return b }
    let body = vec![Stmt::Return(Some(Expr::Ident("b".into(), span())), span())];
    i.register_fn(
        "f".into(),
        vec![
            ("a".into(), None, Some(Expr::LitNumber("2".into(), span()))),
            ("b".into(), None, Some(Expr::Ident("a".into(), span()))),
        ],
        body,
        None,
        Some("Int".into()),
    );

    let call = Expr::Call {
        callee: Box::new(Expr::Ident("f".into(), span())),
        args: vec![],
        span: span(),
    };
    let v = i.eval_expr(call).unwrap();
    assert!(matches!(v, Value::Number(n) if (n - 2.0).abs() < 1e-9));
}
