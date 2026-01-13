use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::{Expr, Pattern, Stmt};

#[test]
fn test_range_operator() {
    let mut interp = Interpreter::new();
    let expr = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), 0..0)),
        op: "..".into(),
        right: Box::new(Expr::LitNumber("3".into(), 0..0)),
        span: 0..0,
    };
    let v = interp.eval_expr(expr).unwrap();
    match v {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0].to_string(), "1");
            assert_eq!(arr[2].to_string(), "3");
        }
        _ => panic!("expected array from range operator"),
    }
}

#[test]
fn test_in_operator_with_range() {
    let mut interp = Interpreter::new();
    // build range 1..5
    let range = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), 0..0)),
        op: "..".into(),
        right: Box::new(Expr::LitNumber("5".into(), 0..0)),
        span: 0..0,
    };
    let expr_in = Expr::Binary {
        left: Box::new(Expr::LitNumber("3".into(), 0..0)),
        op: "in".into(),
        right: Box::new(range),
        span: 0..0,
    };
    let v = interp.eval_expr(expr_in).unwrap();
    match v {
        Value::Bool(b) => assert!(b),
        _ => panic!("expected bool from in operator"),
    }
}

#[test]
fn test_for_loop_iterates_array_and_assigns() {
    let mut interp = Interpreter::new();
    // declare items = ["a","b","c"]
    interp.env.declare(
        "items".into(),
        Some(Value::Array(vec![
            Value::Str("apple".into()),
            Value::Str("banana".into()),
            Value::Str("kiwi".into()),
        ])),
        false,
    );
    // ensure last variable exists
    if interp.env.has("last") == false {
        interp.env.declare("last".into(), None, true);
    }
    // for (item in items) { last = item }
    let for_stmt = Stmt::For {
        var: "item".into(),
        iterable: Expr::Ident("items".into(), 0..0),
        body: vec![Stmt::ExprStmt(
            Expr::Binary {
                left: Box::new(Expr::Ident("last".into(), 0..0)),
                op: "=".into(),
                right: Box::new(Expr::Ident("item".into(), 0..0)),
                span: 0..0,
            },
            0..0,
        )],
        span: 0..0,
    };

    interp.run_block(vec![for_stmt]).unwrap();
    let last = interp.env.get("last").unwrap();
    match last {
        Value::Str(s) => assert_eq!(s, "kiwi"),
        _ => panic!("expected last to be string"),
    }
}

#[test]
fn test_while_loop_increments() {
    let mut interp = Interpreter::new();
    interp
        .env
        .declare("idx".into(), Some(Value::Number(0.0)), true);
    interp.env.declare(
        "out".into(),
        Some(Value::Array(vec![
            Value::Str("a".into()),
            Value::Str("b".into()),
        ])),
        false,
    );

    // while (idx < 2) { idx += 1 }
    let while_stmt = Stmt::While {
        cond: Expr::Binary {
            left: Box::new(Expr::Ident("idx".into(), 0..0)),
            op: "<".into(),
            right: Box::new(Expr::LitNumber("2".into(), 0..0)),
            span: 0..0,
        },
        body: vec![Stmt::ExprStmt(
            Expr::Binary {
                left: Box::new(Expr::Ident("idx".into(), 0..0)),
                op: "+=".into(),
                right: Box::new(Expr::LitNumber("1".into(), 0..0)),
                span: 0..0,
            },
            0..0,
        )],
        span: 0..0,
    };

    interp.run_block(vec![while_stmt]).unwrap();
    let idx = interp.env.get("idx").unwrap();
    match idx {
        Value::Number(n) => assert_eq!(*n as i64, 2),
        _ => panic!("expected idx number"),
    }
}

#[test]
fn test_when_expression_matches_literals_and_types() {
    let mut interp = Interpreter::new();
    // when (1) { 1 -> "One" else -> "Other" }
    let when_expr = Expr::When {
        scrutinee: Some(Box::new(Expr::LitNumber("1".into(), 0..0))),
        arms: vec![
            (
                Pattern::LitNumber("1".into(), 0..0),
                Expr::LitStr("One".into(), 0..0),
            ),
            (Pattern::Else(0..0), Expr::LitStr("Other".into(), 0..0)),
        ],
        span: 0..0,
    };
    let v = interp.eval_expr(when_expr).unwrap();
    match v {
        Value::Str(s) => assert_eq!(s, "One"),
        _ => panic!("expected string result"),
    }
}

#[test]
fn test_when_in_range_pattern() {
    let mut interp = Interpreter::new();
    // when (x) { in 1..5 -> "in-range" else -> "out" }
    let range = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), 0..0)),
        op: "..".into(),
        right: Box::new(Expr::LitNumber("5".into(), 0..0)),
        span: 0..0,
    };
    // Construct when arms using Pattern::InExpr requires parser to produce it; instead construct AST directly
    let arms = vec![
        (
            dotlin_parser::ast::Pattern::InExpr(range.clone(), 0..0),
            Expr::LitStr("in-range".into(), 0..0),
        ),
        (
            dotlin_parser::ast::Pattern::Else(0..0),
            Expr::LitStr("out".into(), 0..0),
        ),
    ];
    let when_expr = Expr::When {
        scrutinee: Some(Box::new(Expr::LitNumber("3".into(), 0..0))),
        arms,
        span: 0..0,
    };
    let v = interp.eval_expr(when_expr).unwrap();
    match v {
        Value::Str(s) => assert_eq!(s, "in-range"),
        _ => panic!("expected string"),
    }
}

#[test]
fn test_when_array_literal_pattern() {
    let mut interp = Interpreter::new();
    interp.env.declare(
        "items".into(),
        Some(Value::Array(vec![
            Value::Str("a".into()),
            Value::Str("b".into()),
        ])),
        false,
    );
    let arms = vec![
        (
            dotlin_parser::ast::Pattern::Array(
                vec![
                    dotlin_parser::ast::Pattern::LitStr("a".into(), 0..0),
                    dotlin_parser::ast::Pattern::LitStr("b".into(), 0..0),
                ],
                0..0,
            ),
            Expr::LitStr("matched".into(), 0..0),
        ),
        (
            dotlin_parser::ast::Pattern::Else(0..0),
            Expr::LitStr("no".into(), 0..0),
        ),
    ];
    let when_expr = Expr::When {
        scrutinee: Some(Box::new(Expr::Ident("items".into(), 0..0))),
        arms,
        span: 0..0,
    };
    let v = interp.eval_expr(when_expr).unwrap();
    match v {
        Value::Str(s) => assert_eq!(s, "matched"),
        _ => panic!("expected string"),
    }
}

#[test]
fn test_when_is_bind_pattern() {
    let mut interp = Interpreter::new();
    let when_expr = Expr::When {
        scrutinee: Some(Box::new(Expr::LitStr("hello".into(), 0..0))),
        arms: vec![
            (
                dotlin_parser::ast::Pattern::IsBind("String".into(), "s".into(), 0..0),
                Expr::Ident("s".into(), 0..0),
            ),
            (
                dotlin_parser::ast::Pattern::Else(0..0),
                Expr::LitStr("no".into(), 0..0),
            ),
        ],
        span: 0..0,
    };
    let v = interp.eval_expr(when_expr).unwrap();
    match v {
        Value::Str(s) => assert_eq!(s, "hello"),
        _ => panic!("expected bound string"),
    }
}

#[test]
fn test_indices_and_lastindex_members() {
    let mut interp = Interpreter::new();
    interp.env.declare(
        "items".into(),
        Some(Value::Array(vec![
            Value::Str("a".into()),
            Value::Str("b".into()),
            Value::Str("c".into()),
        ])),
        false,
    );
    // items.indices -> [0,1,2]
    let indices_expr = Expr::Member {
        receiver: Box::new(Expr::Ident("items".into(), 0..0)),
        name: "indices".into(),
        span: 0..0,
    };
    let v = interp.eval_expr(indices_expr).unwrap();
    match v {
        Value::Array(arr) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0].to_string(), "0");
            assert_eq!(arr[2].to_string(), "2");
        }
        _ => panic!("expected array"),
    }

    // items.lastIndex -> 2
    let last_expr = Expr::Member {
        receiver: Box::new(Expr::Ident("items".into(), 0..0)),
        name: "lastIndex".into(),
        span: 0..0,
    };
    let v2 = interp.eval_expr(last_expr).unwrap();
    match v2 {
        Value::Number(n) => assert_eq!(n as i64, 2),
        _ => panic!("expected number"),
    }
}

#[test]
fn test_step_and_down_to_progressions() {
    let mut interp = Interpreter::new();
    // 1..10 step 2 -> 1,3,5,7,9
    let range = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), 0..0)),
        op: "..".into(),
        right: Box::new(Expr::LitNumber("10".into(), 0..0)),
        span: 0..0,
    };
    let stepped = Expr::Binary {
        left: Box::new(range),
        op: "step".into(),
        right: Box::new(Expr::LitNumber("2".into(), 0..0)),
        span: 0..0,
    };
    let v = interp.eval_expr(stepped).unwrap();
    match v {
        Value::Array(arr) => {
            let vals: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
            assert_eq!(vals, vec!["1", "3", "5", "7", "9"]);
        }
        _ => panic!("expected array"),
    }

    // 9 downTo 0 step 3 -> 9,6,3,0
    let down = Expr::Binary {
        left: Box::new(Expr::LitNumber("9".into(), 0..0)),
        op: "downTo".into(),
        right: Box::new(Expr::LitNumber("0".into(), 0..0)),
        span: 0..0,
    };
    let down_step = Expr::Binary {
        left: Box::new(down),
        op: "step".into(),
        right: Box::new(Expr::LitNumber("3".into(), 0..0)),
        span: 0..0,
    };
    let v2 = interp.eval_expr(down_step).unwrap();
    match v2 {
        Value::Array(arr) => {
            let vals: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
            assert_eq!(vals, vec!["9", "6", "3", "0"]);
        }
        _ => panic!("expected array"),
    }
}

#[test]
fn test_until_and_negative_step_progressions() {
    let mut interp = Interpreter::new();
    // 1 until 5 -> 1,2,3,4
    let until = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), 0..0)),
        op: "until".into(),
        right: Box::new(Expr::LitNumber("5".into(), 0..0)),
        span: 0..0,
    };
    let v = interp.eval_expr(until).unwrap();
    match v {
        Value::Array(arr) => {
            let vals: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
            assert_eq!(vals, vec!["1", "2", "3", "4"]);
        }
        _ => panic!("expected array"),
    }

    // 1..9 step -2 -> reverse-stepping -> 9,7,5,3,1
    let range = Expr::Binary {
        left: Box::new(Expr::LitNumber("1".into(), 0..0)),
        op: "..".into(),
        right: Box::new(Expr::LitNumber("9".into(), 0..0)),
        span: 0..0,
    };
    let neg_step = Expr::Binary {
        left: Box::new(range),
        op: "step".into(),
        right: Box::new(Expr::LitNumber("-2".into(), 0..0)),
        span: 0..0,
    };
    let v2 = interp.eval_expr(neg_step).unwrap();
    match v2 {
        Value::Array(arr) => {
            let vals: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
            assert_eq!(vals, vec!["9", "7", "5", "3", "1"]);
        }
        _ => panic!("expected array"),
    }
}
