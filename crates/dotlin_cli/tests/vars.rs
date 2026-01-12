use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::{Expr, Stmt};

fn span() -> std::ops::Range<usize> {
    0..0
}

#[test]
fn val_assign_once_then_error_on_reassign() {
    let mut it = Interpreter::new();
    // val c: Int; c = 3
    let decl = Stmt::VarDecl {
        is_mut: false,
        name: "c".into(),
        type_name: Some("Int".into()),
        init: None,
        span: span(),
    };
    let assign = Stmt::ExprStmt(
        Expr::Binary {
            left: Box::new(Expr::Ident("c".into(), span())),
            op: "=".into(),
            right: Box::new(Expr::LitNumber("3".into(), span())),
            span: span(),
        },
        span(),
    );
    let reassign = Stmt::ExprStmt(
        Expr::Binary {
            left: Box::new(Expr::Ident("c".into(), span())),
            op: "=".into(),
            right: Box::new(Expr::LitNumber("4".into(), span())),
            span: span(),
        },
        span(),
    );

    let _ = it.run_block(vec![decl, assign]).unwrap();
    // value should be set
    let v = it.env.get("c").unwrap();
    assert!(matches!(v, Value::Number(n) if (*n - 3.0).abs() < 1e-9));

    // second assignment should fail
    let err = it.run_block(vec![reassign]);
    assert!(err.is_err());
}

#[test]
fn var_reassign_allowed() {
    let mut it = Interpreter::new();
    let decl = Stmt::VarDecl {
        is_mut: true,
        name: "x".into(),
        type_name: None,
        init: Some(Expr::LitNumber("0".into(), span())),
        span: span(),
    };
    let inc = Stmt::ExprStmt(
        Expr::Binary {
            left: Box::new(Expr::Ident("x".into(), span())),
            op: "+=".into(),
            right: Box::new(Expr::LitNumber("1".into(), span())),
            span: span(),
        },
        span(),
    );
    let _ = it.run_block(vec![decl]).unwrap();
    let _ = it.run_block(vec![inc.clone(), inc.clone()]).unwrap();
    let v = it.env.get("x").unwrap();
    assert!(matches!(v, Value::Number(n) if (*n - 2.0).abs() < 1e-9));
}

#[test]
fn top_level_mutation_from_function() {
    let mut it = Interpreter::new();
    // declare top-level x
    it.env.declare("x".into(), Some(Value::Number(0.0)), true);
    // function body: x += 1
    let body = vec![Stmt::ExprStmt(
        Expr::Binary {
            left: Box::new(Expr::Ident("x".into(), span())),
            op: "+=".into(),
            right: Box::new(Expr::LitNumber("1".into(), span())),
            span: span(),
        },
        span(),
    )];
    // run function (no params)
    let _ = it.run_function(&vec![], &vec![], body).unwrap();
    let v = it.env.get("x").unwrap();
    assert!(matches!(v, Value::Number(n) if (*n - 1.0).abs() < 1e-9));
}

#[test]
fn indexed_assignment_and_compound_index_behaviour() {
    let mut it = Interpreter::new();
    // declare top-level arr = [1,2]
    it.env.declare(
        "arr".into(),
        Some(Value::Array(vec![Value::Number(1.0), Value::Number(2.0)])),
        true,
    );

    // arr[1] = 5
    let assign = Stmt::ExprStmt(
        Expr::Binary {
            left: Box::new(Expr::Index {
                target: Box::new(Expr::Ident("arr".into(), span())),
                index: 1,
                span: span(),
            }),
            op: "=".into(),
            right: Box::new(Expr::LitNumber("5".into(), span())),
            span: span(),
        },
        span(),
    );

    let _ = it.run_block(vec![assign]).unwrap();
    if let Value::Array(a) = it.env.get("arr").unwrap().clone() {
        assert!(matches!(a[1], Value::Number(n) if (n - 5.0).abs() < 1e-9));
    } else {
        panic!("arr not array");
    }

    // compound assignment on indexed lvalue should error
    let comp = Stmt::ExprStmt(
        Expr::Binary {
            left: Box::new(Expr::Index {
                target: Box::new(Expr::Ident("arr".into(), span())),
                index: 0,
                span: span(),
            }),
            op: "+=".into(),
            right: Box::new(Expr::LitNumber("3".into(), span())),
            span: span(),
        },
        span(),
    );

    let err = it.run_block(vec![comp]);
    assert!(err.is_err());
}
