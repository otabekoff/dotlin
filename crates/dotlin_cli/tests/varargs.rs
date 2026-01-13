use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::{Expr};

fn span() -> std::ops::Range<usize> {
    0..0
}

#[test]
fn test_varargs_count() {
    let mut i = Interpreter::new();
    // fun count(args): Int = args.size
    let expr_body = Expr::Member {
        receiver: Box::new(Expr::Ident("args".into(), span())),
        name: "size".into(),
        span: span(),
    };
    i.register_fn(
        "count".into(),
        vec![("args".into(), None, None)],
        vec![],
        Some(expr_body),
        Some("Int".into()),
    );

    let call = Expr::Call {
        callee: Box::new(Expr::Ident("count".into(), span())),
        args: vec![
            Expr::LitNumber("1".into(), span()),
            Expr::LitNumber("2".into(), span()),
            Expr::LitNumber("3".into(), span()),
        ],
        span: span(),
    };
    let v = i.eval_expr(call).unwrap();
    assert!(matches!(v, Value::Number(n) if (n - 3.0).abs() < 1e-9));
}
