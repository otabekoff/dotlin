use dotlin_cli::interpreter::{Interpreter, Value};
use dotlin_parser::ast::Expr;

#[test]
fn test_nested_and_escaped_templates() {
    let mut interp = Interpreter::new();
    // nested when expression inside template and escaped dollar
    let s =
        "escaped dollar: \\$notavalue, nested: ${ when (1) { 1 -> \"One\" else -> \"Other\" } }";
    let expr = Expr::LitStr(s.into(), 0..0);
    let v = interp.eval_expr(expr).unwrap();
    match v {
        Value::Str(st) => {
            assert!(st.contains("escaped dollar: $notavalue"));
            assert!(st.contains("nested: One") || st.contains("nested: Other"));
        }
        _ => panic!("expected string"),
    }
}
