use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::Expr;

// simple helper to create spans
fn span() -> std::ops::Range<usize> {
    0..0
}

#[test]
fn test_arithmetic_and_precedence() {
    let mut i = Interpreter::new();
    // 1 + 2 * 3 => 7
    let expr = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), span())),
        op: "+".into(),
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::LitNumber("2".into(), span())),
            op: "*".into(),
            right: Box::new(Expr::LitNumber("3".into(), span())),
            span: span(),
        }),
        span: span(),
    };
    let v = i.eval_expr(expr).unwrap();
    assert!(matches!(v, Value::Number(n) if (n - 7.0).abs() < 1e-9));

    // (1 + 2) * 3 => 9
    let expr2 = Expr::Binary {
        left: Box::new(Expr::Binary {
            left: Box::new(Expr::LitNumber("1".into(), span())),
            op: "+".into(),
            right: Box::new(Expr::LitNumber("2".into(), span())),
            span: span(),
        }),
        op: "*".into(),
        right: Box::new(Expr::LitNumber("3".into(), span())),
        span: span(),
    };
    let v2 = i.eval_expr(expr2).unwrap();
    assert!(matches!(v2, Value::Number(n) if (n - 9.0).abs() < 1e-9));
}

#[test]
fn test_comparisons_and_booleans() {
    let mut i = Interpreter::new();
    // 3 > 2 => true
    let gt = Expr::Binary {
        left: Box::new(Expr::LitNumber("3".into(), span())),
        op: ">".into(),
        right: Box::new(Expr::LitNumber("2".into(), span())),
        span: span(),
    };
    let v = i.eval_expr(gt).unwrap();
    assert!(matches!(v, Value::Bool(true)));

    // 1 == 1 => true
    let eq = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), span())),
        op: "==".into(),
        right: Box::new(Expr::LitNumber("1".into(), span())),
        span: span(),
    };
    let v2 = i.eval_expr(eq).unwrap();
    assert!(matches!(v2, Value::Bool(true)));

    // "a" == "b" => false
    let neq = Expr::Binary {
        left: Box::new(Expr::LitStr("a".into(), span())),
        op: "==".into(),
        right: Box::new(Expr::LitStr("b".into(), span())),
        span: span(),
    };
    let v3 = i.eval_expr(neq).unwrap();
    assert!(matches!(v3, Value::Bool(false)));
}
