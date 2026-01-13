use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::{Expr, Node, Stmt};

// Helper to create a simple function node with block body
fn make_block_fn(
    name: &str,
    params: Vec<&str>,
    return_type: Option<&str>,
    body: Vec<Stmt>,
) -> Node {
    Node::Function {
        name: name.to_string(),
        params: params.into_iter().map(|s| s.to_string()).collect(),
        return_type: return_type.map(|s| s.to_string()),
        body,
        expr_body: None,
        span: 0..0,
    }
}

#[test]
fn explicit_return_typed_function() {
    let mut interp = Interpreter::new();
    // fun sum(a: Int, b: Int): Int { return a + b }
    let expr = Expr::Binary {
        left: Box::new(Expr::Ident("a".into(), 0..0)),
        op: "+".into(),
        right: Box::new(Expr::Ident("b".into(), 0..0)),
        span: 0..0,
    };
    let body = vec![Stmt::Return(Some(expr), 0..0)];
    let _node = make_block_fn("sum", vec!["a", "b"], Some("Int"), body.clone());
    interp.register_fn(
        "sum".into(),
        vec!["a".into(), "b".into()],
        body,
        None,
        Some("Int".into()),
    );
    let call = Expr::Call {
        callee: Box::new(Expr::Ident("sum".into(), 0..0)),
        args: vec![
            Expr::LitNumber("2".into(), 0..0),
            Expr::LitNumber("3".into(), 0..0),
        ],
        span: 0..0,
    };
    let res = interp.eval_expr(call).unwrap();
    match res {
        Value::Number(n) => assert_eq!(n as i64, 5),
        _ => panic!("expected number"),
    }
}

#[test]
fn expression_bodied_function_inferred_return() {
    let mut interp = Interpreter::new();
    // fun add(a: Int, b: Int) = a + b
    let expr = Expr::Binary {
        left: Box::new(Expr::Ident("a".into(), 0..0)),
        op: "+".into(),
        right: Box::new(Expr::Ident("b".into(), 0..0)),
        span: 0..0,
    };
    let _fn_node = Node::Function {
        name: "add".into(),
        params: vec!["a".into(), "b".into()],
        return_type: None,
        body: vec![],
        expr_body: Some(expr.clone()),
        span: 0..0,
    };
    interp.register_fn(
        "add".into(),
        vec!["a".into(), "b".into()],
        vec![],
        Some(expr),
        Some("Int".into()),
    );
    let call = Expr::Call {
        callee: Box::new(Expr::Ident("add".into(), 0..0)),
        args: vec![
            Expr::LitNumber("7".into(), 0..0),
            Expr::LitNumber("8".into(), 0..0),
        ],
        span: 0..0,
    };
    let res = interp.eval_expr(call).unwrap();
    match res {
        Value::Number(n) => assert_eq!(n as i64, 15),
        _ => panic!("expected number"),
    }
}

#[test]
fn unit_returning_function() {
    let mut interp = Interpreter::new();
    // fun log(x: Int): Unit { println(x); }
    let print_call = Expr::Call {
        callee: Box::new(Expr::Ident("println".into(), 0..0)),
        args: vec![Expr::Ident("x".into(), 0..0)],
        span: 0..0,
    };
    let print_stmt = Stmt::ExprStmt(print_call, 0..0);
    let _node = make_block_fn("log", vec!["x"], Some("Unit"), vec![print_stmt.clone()]);
    interp.register_fn(
        "log".into(),
        vec!["x".into()],
        vec![print_stmt],
        None,
        Some("Unit".into()),
    );
    let call = Expr::Call {
        callee: Box::new(Expr::Ident("log".into(), 0..0)),
        args: vec![Expr::LitNumber("42".into(), 0..0)],
        span: 0..0,
    };
    let res = interp.eval_expr(call).unwrap();
    match res {
        Value::Unit => (),
        _ => panic!("expected unit"),
    }
}
