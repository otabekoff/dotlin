use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::{Expr, Stmt};

fn span() -> std::ops::Range<usize> {
    0..0
}

#[test]
fn test_function_return_value() {
    let mut i = Interpreter::new();
    // create function add(a, b) { return a + b }
    let body = vec![Stmt::Return(
        Some(Expr::Binary {
            left: Box::new(Expr::Ident("a".into(), span())),
            op: "+".into(),
            right: Box::new(Expr::Ident("b".into(), span())),
            span: span(),
        }),
        span(),
    )];
    i.register_fn(
        "add".into(),
        vec!["a".into(), "b".into()],
        body,
        None,
        Some("Int".into()),
    );

    // call add(2,3)
    let call = Expr::Call {
        callee: Box::new(Expr::Ident("add".into(), span())),
        args: vec![
            Expr::LitNumber("2".into(), span()),
            Expr::LitNumber("3".into(), span()),
        ],
        span: span(),
    };
    let v = i.eval_expr(call).unwrap();
    assert!(matches!(v, Value::Number(n) if (n - 5.0).abs() < 1e-9));
}

#[test]
fn test_return_early() {
    let mut i = Interpreter::new();
    // function foo() { println("before"); return 42; println("after"); }
    let body = vec![
        Stmt::ExprStmt(
            Expr::Call {
                callee: Box::new(Expr::Ident("println".into(), span())),
                args: vec![Expr::LitStr("before".into(), span())],
                span: span(),
            },
            span(),
        ),
        Stmt::Return(Some(Expr::LitNumber("42".into(), span())), span()),
        Stmt::ExprStmt(
            Expr::Call {
                callee: Box::new(Expr::Ident("println".into(), span())),
                args: vec![Expr::LitStr("after".into(), span())],
                span: span(),
            },
            span(),
        ),
    ];
    i.register_fn("foo".into(), vec![], body, None, Some("Int".into()));
    let call = Expr::Call {
        callee: Box::new(Expr::Ident("foo".into(), span())),
        args: vec![],
        span: span(),
    };
    let v = i.eval_expr(call).unwrap();
    assert!(matches!(v, Value::Number(n) if (n - 42.0).abs() < 1e-9));
}
