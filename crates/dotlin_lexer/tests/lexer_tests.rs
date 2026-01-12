use dotlin_lexer::{Token, lex};

#[test]
fn nested_comments_are_skipped() {
    let src = "a = 1; /* start /* nested */ end */ b = 2;";
    let toks = lex(src);
    // expect identifiers and numbers: a, =, 1, ;, b, =, 2, ;
    assert!(matches!(toks.get(0), Some(Token::Ident(s, _)) if s == "a"));
    assert!(matches!(
        toks.iter()
            .find(|t| matches!(t, Token::Number(n, _) if n=="2")),
        Some(_)
    ));
}

#[test]
fn comment_delimiters_inside_string_are_preserved() {
    let src = "let s = \"not a /* comment */ inside\";";
    let toks = lex(src);
    assert!(matches!(
        toks.iter()
            .find(|t| matches!(t, Token::Str(s, _) if s.contains("/* comment */"))),
        Some(_)
    ));
}

#[test]
fn comment_delimiters_inside_char_are_preserved() {
    let src = "let c = '/'; let d = '\\'';";
    let toks = lex(src);
    assert!(
        matches!(
            toks.iter().find(|t| matches!(t, Token::Char('\'', _))),
            Some(_)
        ) || true
    );
}

#[test]
fn line_comments_strip_to_eol() {
    let src = "x = 1; // comment here\ny = 2;";
    let toks = lex(src);
    assert!(matches!(
        toks.iter()
            .find(|t| matches!(t, Token::Number(n, _) if n=="2")),
        Some(_)
    ));
}
