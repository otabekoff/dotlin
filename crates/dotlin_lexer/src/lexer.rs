use crate::cursor::Cursor;
use crate::token::Token;

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "fun"
            | "val"
            | "var"
            | "class"
            | "open"
            | "if"
            | "else"
            | "for"
            | "while"
            | "when"
            | "return"
            | "package"
            | "import"
            | "println"
            | "print"
            | "readln"
            | "main"
            | "true"
            | "false"
            | "null"
            | "in"
            | "is"
    )
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut cur = Cursor::new(src);
    let mut out = Vec::new();

    while cur.peek().is_some() {
        let start = cur.pos();
        let ch = cur.next().unwrap();

        // handle comments
        if ch == '/' {
            if cur.next_if('/') {
                // line comment: consume until newline
                while let Some(nc) = cur.next() {
                    if nc == '\n' {
                        break;
                    }
                }
                continue;
            } else if cur.next_if('*') {
                // block comment with nesting
                let mut depth: u32 = 1;
                while let Some(nc) = cur.next() {
                    if nc == '/' && cur.peek() == Some('*') {
                        cur.next();
                        depth += 1;
                    } else if nc == '*' && cur.peek() == Some('/') {
                        cur.next();
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            break;
                        }
                    }
                }
                continue;
            } else {
                let end = cur.pos();
                out.push(Token::Symbol("/".to_string(), start..end));
                continue;
            }
        }

        // strings (simple escaped double-quoted)
        if ch == '"' {
            let mut s = String::new();
            while let Some(nc) = cur.next() {
                if nc == '\\' {
                    if let Some(esc) = cur.next() {
                        s.push('\\');
                        s.push(esc);
                        continue;
                    }
                } else if nc == '"' {
                    break;
                }
                s.push(nc);
            }
            let end = cur.pos();
            out.push(Token::Str(s, start..end));
            continue;
        }

        // char literal
        if ch == '\'' {
            let mut cval = None;
            while let Some(nc) = cur.next() {
                if nc == '\\' {
                    if let Some(esc) = cur.next() {
                        cval = Some(esc);
                    }
                } else if nc == '\'' {
                    break;
                } else {
                    cval = Some(nc);
                }
            }
            if let Some(cv) = cval {
                let end = cur.pos();
                out.push(Token::Char(cv, start..end));
            }
            continue;
        }

        // whitespace
        if ch.is_whitespace() {
            continue;
        }

        // identifiers (start with letter or underscore)
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut id = String::new();
            id.push(ch);
            while let Some(p) = cur.peek() {
                if p.is_ascii_alphanumeric() || p == '_' {
                    id.push(p);
                    cur.next();
                } else {
                    break;
                }
            }
            let end = cur.pos();
            if is_keyword(&id) {
                out.push(Token::Keyword(id, start..end));
            } else {
                out.push(Token::Ident(id, start..end));
            }
            continue;
        }

        // numbers: accept digits, underscores, decimal point and exponent (simple)
        if ch.is_ascii_digit() {
            let mut num = String::new();
            num.push(ch);
            while let Some(p) = cur.peek() {
                if p.is_ascii_digit()
                    || p == '_'
                    || p == '.'
                    || p == 'e'
                    || p == 'E'
                    || p == '+'
                    || p == '-'
                {
                    num.push(p);
                    cur.next();
                } else {
                    break;
                }
            }
            let end = cur.pos();
            out.push(Token::Number(num, start..end));
            continue;
        }

        // handle multi-char operators/punctuation
        if let Some(p) = cur.peek() {
            let two = format!("{}{}", ch, p);
            match two.as_str() {
                "->" | ".." | "==" | "!=" | "<=" | ">=" | "&&" | "||" | "::" | "+=" | "-="
                | "*=" | "/=" => {
                    cur.next();
                    let end = cur.pos();
                    out.push(Token::Symbol(two, start..end));
                    continue;
                }
                _ => {}
            }
        }

        let end = cur.pos();
        out.push(Token::Symbol(ch.to_string(), start..end));
    }

    out
}
