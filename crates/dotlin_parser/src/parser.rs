use crate::ast::{Expr, Node, Stmt};
use dotlin_lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, idx: 0 }
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.idx)
    }

    fn next(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.idx);
        if t.is_some() {
            self.idx += 1;
        }
        t
    }

    fn parse_simple_expr(&mut self) -> Option<Expr> {
        // primary expressions: literals, identifiers, calls, parenthesis, indexing
        if let Some(t) = self.peek() {
            match t {
                Token::Symbol(s, _sspan) if s == "{" => {
                    // possible lambda expression: { x, y -> expr } or { expr } (implicit `it`)
                    if let Some(lambda) = self.parse_lambda() {
                        return Some(lambda);
                    }
                }
                Token::Symbol(s, _) if s == "(" => {
                    // grouping
                    self.next();
                    let expr = self.parse_expr(0);
                    if let Some(Token::Symbol(c, _)) = self.peek() {
                        if c == ")" {
                            self.next();
                        }
                    }
                    return expr;
                }
                Token::Ident(name, span) | Token::Keyword(name, span) => {
                    // possible call if followed by (
                    let name_clone = name.clone();
                    let span_clone = span.clone();
                    self.next();
                    // special-case 'when' expression
                    if name_clone == "when" {
                        // optional '(' scrutinee ')' or no scrutinee
                        let mut scrut: Option<Expr> = None;
                        if let Some(Token::Symbol(p, _)) = self.peek() {
                            if p == "(" {
                                self.next();
                                scrut = self.parse_expr(0);
                                if let Some(Token::Symbol(p2, _)) = self.peek() {
                                    if p2 == ")" {
                                        self.next();
                                    }
                                }
                            }
                        }
                        // expect '{' for arms
                        let mut arms: Vec<(crate::ast::Pattern, Expr)> = Vec::new();
                        if let Some(Token::Symbol(b, _)) = self.peek() {
                            if b == "{" {
                                self.next();
                                loop {
                                    match self.peek() {
                                        Some(Token::Symbol(s2, _)) if s2 == "}" => {
                                            self.next();
                                            break;
                                        }
                                        Some(_) => {
                                            // parse pattern (simple literals, is/not is, else)
                                            let pat = match self.peek() {
                                                Some(Token::Number(n, spann)) => {
                                                    // possible range literal pattern like 1..5
                                                    let lit = n.clone();
                                                    self.next();
                                                    let pat = if let Some(Token::Symbol(dots, _)) =
                                                        self.peek()
                                                    {
                                                        if dots == ".." {
                                                            // consume '..'
                                                            self.next();
                                                            if let Some(Token::Number(n2, _span2)) =
                                                                self.next()
                                                            {
                                                                crate::ast::Pattern::Range(
                                                                    lit.clone(),
                                                                    n2.clone(),
                                                                    spann.clone(),
                                                                )
                                                            } else {
                                                                // fallback to plain lit number pattern
                                                                crate::ast::Pattern::LitNumber(
                                                                    lit.clone(),
                                                                    spann.clone(),
                                                                )
                                                            }
                                                        } else {
                                                            crate::ast::Pattern::LitNumber(
                                                                lit.clone(),
                                                                spann.clone(),
                                                            )
                                                        }
                                                    } else {
                                                        crate::ast::Pattern::LitNumber(
                                                            lit.clone(),
                                                            spann.clone(),
                                                        )
                                                    };
                                                    pat
                                                }
                                                Some(Token::Str(s, spans)) => {
                                                    let lit = s.clone();
                                                    self.next();
                                                    crate::ast::Pattern::LitStr(lit, spans.clone())
                                                }
                                                Some(Token::Symbol(sym, syspan)) if sym == "[" => {
                                                    // array pattern like [1, "a", true]
                                                    self.next();
                                                    let mut elems: Vec<crate::ast::Pattern> =
                                                        Vec::new();
                                                    loop {
                                                        if let Some(Token::Symbol(c, _)) =
                                                            self.peek()
                                                        {
                                                            if c == "]" {
                                                                self.next();
                                                                break;
                                                            }
                                                        }
                                                        // try parse an inner expression and convert to pattern
                                                        if let Some(e) = self.parse_expr(0) {
                                                            match e {
                                                                Expr::LitNumber(nv, spv) => elems
                                                                    .push(
                                                                    crate::ast::Pattern::LitNumber(
                                                                        nv, spv,
                                                                    ),
                                                                ),
                                                                Expr::LitStr(sv, spv) => elems
                                                                    .push(
                                                                        crate::ast::Pattern::LitStr(
                                                                            sv, spv,
                                                                        ),
                                                                    ),
                                                                Expr::LitBool(bv, sb) => elems
                                                                    .push(
                                                                    crate::ast::Pattern::LitBool(
                                                                        bv, sb,
                                                                    ),
                                                                ),
                                                                Expr::Ident(namev, spv) => elems
                                                                    .push(
                                                                        crate::ast::Pattern::Bind(
                                                                            namev, spv,
                                                                        ),
                                                                    ),
                                                                _ => elems.push(
                                                                    crate::ast::Pattern::Else(
                                                                        syspan.clone(),
                                                                    ),
                                                                ),
                                                            }
                                                        } else {
                                                            // consume and break to avoid infinite loop
                                                            self.next();
                                                            break;
                                                        }
                                                        if let Some(Token::Symbol(com, _)) =
                                                            self.peek()
                                                        {
                                                            if com == "," {
                                                                self.next();
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    crate::ast::Pattern::Array(
                                                        elems,
                                                        syspan.clone(),
                                                    )
                                                }
                                                Some(Token::Keyword(kp, kpspan)) if kp == "is" => {
                                                    self.next();
                                                    if let Some(Token::Ident(tn, tsp)) = self.next()
                                                    {
                                                        // check for optional binding: is Type(name)
                                                        if let Some(Token::Symbol(lp, _)) =
                                                            self.peek()
                                                        {
                                                            if lp == "(" {
                                                                // consume '('
                                                                self.next();
                                                                if let Some(Token::Ident(
                                                                    bn,
                                                                    _bnspan,
                                                                )) = self.next()
                                                                {
                                                                    // expect ')'
                                                                    if let Some(Token::Symbol(
                                                                        rp,
                                                                        _,
                                                                    )) = self.peek()
                                                                    {
                                                                        if rp == ")" {
                                                                            self.next();
                                                                        }
                                                                    }
                                                                    crate::ast::Pattern::IsBind(
                                                                        tn.clone(),
                                                                        bn.clone(),
                                                                        tsp.clone(),
                                                                    )
                                                                } else {
                                                                    crate::ast::Pattern::IsType(
                                                                        tn.clone(),
                                                                        tsp.clone(),
                                                                    )
                                                                }
                                                            } else {
                                                                crate::ast::Pattern::IsType(
                                                                    tn.clone(),
                                                                    tsp.clone(),
                                                                )
                                                            }
                                                        } else {
                                                            crate::ast::Pattern::IsType(
                                                                tn.clone(),
                                                                tsp.clone(),
                                                            )
                                                        }
                                                    } else {
                                                        crate::ast::Pattern::Else(kpspan.clone())
                                                    }
                                                }
                                                Some(Token::Symbol(sy, syspan)) if sy == "!" => {
                                                    self.next();
                                                    if let Some(Token::Keyword(iskw, _isps)) =
                                                        self.next()
                                                    {
                                                        if iskw == "is" {
                                                            if let Some(Token::Ident(tn2, tsp2)) =
                                                                self.next()
                                                            {
                                                                crate::ast::Pattern::NotIsType(
                                                                    tn2.clone(),
                                                                    tsp2.clone(),
                                                                )
                                                            } else {
                                                                crate::ast::Pattern::Else(
                                                                    syspan.clone(),
                                                                )
                                                            }
                                                        } else {
                                                            crate::ast::Pattern::Else(
                                                                syspan.clone(),
                                                            )
                                                        }
                                                    } else {
                                                        crate::ast::Pattern::Else(syspan.clone())
                                                    }
                                                }
                                                Some(Token::Keyword(kp2, kp2span))
                                                    if kp2 == "else" =>
                                                {
                                                    let sp = kp2span.clone();
                                                    self.next();
                                                    crate::ast::Pattern::Else(sp)
                                                }
                                                Some(Token::Keyword(kpin, kpinspan))
                                                    if kpin == "in" =>
                                                {
                                                    // parse an expression after 'in' and store it
                                                    self.next();
                                                    if let Some(expr) = self.parse_expr(0) {
                                                        crate::ast::Pattern::InExpr(
                                                            expr,
                                                            kpinspan.clone(),
                                                        )
                                                    } else {
                                                        crate::ast::Pattern::Else(kpinspan.clone())
                                                    }
                                                }
                                                Some(Token::Ident(idn, idspan)) => {
                                                    // identifier in pattern -> binding
                                                    let name = idn.clone();
                                                    self.next();
                                                    crate::ast::Pattern::Bind(name, idspan.clone())
                                                }
                                                _ => {
                                                    self.next();
                                                    crate::ast::Pattern::Else(span_clone.clone())
                                                }
                                            };
                                            // optional '->' or '=>' or ':'? handle '->'
                                            if let Some(Token::Symbol(arrow, _)) = self.peek() {
                                                if arrow == "->" {
                                                    self.next();
                                                }
                                            }
                                            // parse arm expression
                                            if let Some(aexpr) = self.parse_expr(0) {
                                                arms.push((pat, aexpr));
                                            }
                                            // optional separator tokens
                                            if let Some(Token::Symbol(sc, _)) = self.peek() {
                                                if sc == ";" {
                                                    self.next();
                                                }
                                            }
                                        }
                                        None => break,
                                    }
                                }
                            }
                        }
                        // span for when
                        let span = span_clone.clone();
                        return Some(Expr::When {
                            scrutinee: scrut.map(Box::new),
                            arms,
                            span,
                        });
                    }
                    // handle boolean keywords as literals
                    if name_clone == "true" {
                        return Some(Expr::LitBool(true, span_clone));
                    }
                    if name_clone == "false" {
                        return Some(Expr::LitBool(false, span_clone));
                    }
                    if let Some(Token::Symbol(p, _)) = self.peek() {
                        if p == "(" {
                            // consume '('
                            self.next();
                            let mut args = Vec::new();
                            loop {
                                if let Some(Token::Symbol(sym, _)) = self.peek() {
                                    if sym == ")" {
                                        self.next();
                                        break;
                                    }
                                }
                                if let Some(arg) = self.parse_expr(0) {
                                    args.push(arg);
                                } else {
                                    // skip and advance
                                    self.next();
                                }
                                if let Some(Token::Symbol(sym2, _)) = self.peek() {
                                    if sym2 == "," {
                                        self.next();
                                    }
                                }
                            }
                            let span = span_clone.clone();
                            let mut expr = Expr::Call {
                                callee: Box::new(Expr::Ident(
                                    name_clone.clone(),
                                    span_clone.clone(),
                                )),
                                args,
                                span: span.clone(),
                            };
                            // allow a trailing lambda after a call: e.g., foo(...) { x -> ... }
                            if let Some(Token::Symbol(bc, _)) = self.peek() {
                                if bc == "{" {
                                    if let Some(lambda_expr) = self.parse_lambda() {
                                        // append as last arg
                                        if let Expr::Call {
                                            args: ref mut aargs,
                                            ..
                                        } = expr
                                        {
                                            aargs.push(lambda_expr);
                                        }
                                    }
                                }
                            }
                            // handle indexing after call: e.g., foo(...)[0]
                            while let Some(Token::Symbol(sbr, _)) = self.peek() {
                                if sbr == "[" {
                                    self.next();
                                    if let Some(Token::Number(nm, _)) = self.next() {
                                        if let Ok(idx) = nm.parse::<usize>() {
                                            // expect ]
                                            if let Some(Token::Symbol(cbr, _)) = self.peek() {
                                                if cbr == "]" {
                                                    self.next();
                                                }
                                            }
                                            expr = Expr::Index {
                                                target: Box::new(expr),
                                                index: idx,
                                                span: span.clone(),
                                            };
                                            continue;
                                        }
                                    }
                                    // if parsing failed, break
                                    break;
                                }
                                break;
                            }
                            // handle member access/calls and further chaining: .name or .name(...)
                            loop {
                                if let Some(Token::Symbol(dot, _)) = self.peek() {
                                    if dot == "." {
                                        // consume dot
                                        self.next();
                                        // expect ident
                                        if let Some(Token::Ident(mname, mspan)) = self.next() {
                                            // if followed by '(', it's a method call
                                            if let Some(Token::Symbol(lp, _)) = self.peek() {
                                                if lp == "(" {
                                                    self.next();
                                                    let mut margs = Vec::new();
                                                    loop {
                                                        if let Some(Token::Symbol(sym, _)) =
                                                            self.peek()
                                                        {
                                                            if sym == ")" {
                                                                self.next();
                                                                break;
                                                            }
                                                        }
                                                        if let Some(arg) = self.parse_expr(0) {
                                                            margs.push(arg);
                                                        } else {
                                                            self.next();
                                                        }
                                                        if let Some(Token::Symbol(sym2, _)) =
                                                            self.peek()
                                                        {
                                                            if sym2 == "," {
                                                                self.next();
                                                            }
                                                        }
                                                    }
                                                    // callee is a Member expression
                                                    expr = Expr::Call {
                                                        callee: Box::new(Expr::Member {
                                                            receiver: Box::new(expr),
                                                            name: mname.clone(),
                                                            span: mspan.clone(),
                                                        }),
                                                        args: margs,
                                                        span: mspan.clone(),
                                                    };
                                                    continue;
                                                }
                                            }
                                            // plain member access
                                            expr = Expr::Member {
                                                receiver: Box::new(expr),
                                                name: mname.clone(),
                                                span: mspan.clone(),
                                            };
                                            continue;
                                        }
                                        // if not ident after dot, break
                                        break;
                                    }
                                }
                                break;
                            }
                            return Some(expr);
                        }
                    }
                    // identifier possibly followed by member access/calls and indexing: e.g., args.contentToString(), args[0]
                    let mut expr = Expr::Ident(name_clone, span_clone.clone());
                    loop {
                        // indexing
                        if let Some(Token::Symbol(sbr, _)) = self.peek() {
                            if sbr == "[" {
                                self.next();
                                if let Some(Token::Number(nm, _)) = self.next() {
                                    if let Ok(idx) = nm.parse::<usize>() {
                                        if let Some(Token::Symbol(cbr, _)) = self.peek() {
                                            if cbr == "]" {
                                                self.next();
                                            }
                                        }
                                        expr = Expr::Index {
                                            target: Box::new(expr),
                                            index: idx,
                                            span: span_clone.clone(),
                                        };
                                        continue;
                                    }
                                }
                                break;
                            }
                        }
                        // member access/calls
                        if let Some(Token::Symbol(dot, _)) = self.peek() {
                            if dot == "." {
                                self.next();
                                if let Some(Token::Ident(mname, mspan)) = self.next() {
                                    // method call?
                                    if let Some(Token::Symbol(lp, _)) = self.peek() {
                                        if lp == "(" {
                                            self.next();
                                            let mut margs = Vec::new();
                                            loop {
                                                if let Some(Token::Symbol(sym, _)) = self.peek() {
                                                    if sym == ")" {
                                                        self.next();
                                                        break;
                                                    }
                                                }
                                                if let Some(arg) = self.parse_expr(0) {
                                                    margs.push(arg);
                                                } else {
                                                    self.next();
                                                }
                                                if let Some(Token::Symbol(sym2, _)) = self.peek() {
                                                    if sym2 == "," {
                                                        self.next();
                                                    }
                                                }
                                            }
                                            expr = Expr::Call {
                                                callee: Box::new(Expr::Member {
                                                    receiver: Box::new(expr),
                                                    name: mname.clone(),
                                                    span: mspan.clone(),
                                                }),
                                                args: margs,
                                                span: mspan.clone(),
                                            };
                                            continue;
                                        }
                                    }
                                    expr = Expr::Member {
                                        receiver: Box::new(expr),
                                        name: mname.clone(),
                                        span: mspan.clone(),
                                    };
                                    continue;
                                }
                                break;
                            }
                        }
                        break;
                    }
                    return Some(expr);
                }
                Token::Symbol(s, _) if s == "!" => {
                    // unary not
                    self.next();
                    if let Some(expr) = self.parse_expr(50) {
                        let span = match &expr {
                            Expr::LitStr(_, s) => s.clone(),
                            Expr::LitNumber(_, s) => s.clone(),
                            Expr::Ident(_, s) => s.clone(),
                            Expr::Call { span, .. } => span.clone(),
                            Expr::Index { span, .. } => span.clone(),
                            Expr::Binary { span, .. } => span.clone(),
                            Expr::Unary { span, .. } => span.clone(),
                            Expr::Member { span, .. } => span.clone(),
                            Expr::LitBool(_, s) => s.clone(),
                            Expr::When { span, .. } => span.clone(),
                            Expr::Lambda { span, .. } => span.clone(),
                        };
                        return Some(Expr::Unary {
                            op: "!".into(),
                            expr: Box::new(expr),
                            span,
                        });
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn parse_expr(&mut self, min_prec: u8) -> Option<Expr> {
        fn op_prec(op: &str) -> Option<u8> {
            match op {
                "*" | "/" => Some(40),
                "+" | "-" => Some(30),
                "==" | "!=" | "<" | ">" | "<=" | ">=" => Some(20),
                "in" => Some(20),
                ".." => Some(35),
                "downTo" => Some(35),
                "step" => Some(15),
                "=" | "+=" | "-=" | "*=" | "/=" => Some(10),
                _ => None,
            }
        }

        let mut lhs = self.parse_simple_expr()?;
        loop {
            // operator token may be a Symbol like "+" or a Keyword/Ident operator like "in" or "step"
            let op = match self.peek() {
                Some(Token::Symbol(op, _)) => op.clone(),
                Some(Token::Keyword(k, _)) => k.clone(),
                Some(Token::Ident(k, _)) => k.clone(),
                _ => break,
            };
            let prec = match op_prec(op.as_str()) {
                Some(p) => p,
                None => break,
            };
            if prec < min_prec {
                break;
            }
            // consume op
            self.next();
            // parse rhs with higher precedence (right-associative handling simple)
            let rhs = self.parse_expr(prec + 1).unwrap_or_else(|| lhs.clone());
            // build span from lhs
            let span = match &lhs {
                Expr::LitStr(_, s) => s.clone(),
                Expr::LitNumber(_, s) => s.clone(),
                Expr::LitBool(_, s) => s.clone(),
                Expr::Ident(_, s) => s.clone(),
                Expr::Call { span, .. } => span.clone(),
                Expr::Index { span, .. } => span.clone(),
                Expr::Binary { span, .. } => span.clone(),
                Expr::Unary { span, .. } => span.clone(),
                Expr::Member { span, .. } => span.clone(),
                Expr::When { span, .. } => span.clone(),
                Expr::Lambda { span, .. } => span.clone(),
            };

            lhs = Expr::Binary {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
                span,
            };
        }
        Some(lhs)
    }

    // parse a lambda expression starting at a '{'. consumes the closing '}'.
    fn parse_lambda(&mut self) -> Option<Expr> {
        // expect '{'
        if let Some(Token::Symbol(b, _bspan)) = self.peek() {
            if b != "{" {
                return None;
            }
        } else {
            return None;
        }
        // consume '{'
        let span_start = if let Some(Token::Symbol(_, s)) = self.next() {
            s.clone()
        } else {
            return None;
        };

        // attempt to parse parameters before '->'
        let mut params: Vec<String> = Vec::new();
        let mut saw_arrow = false;
        // peek ahead to see if we have an identifier list followed by '->'
        loop {
            match self.peek() {
                Some(Token::Symbol(sym, _)) if sym == "->" => {
                    self.next();
                    saw_arrow = true;
                    break;
                }
                Some(Token::Ident(_id, _)) => {
                    if let Some(Token::Ident(idv, _)) = self.next() {
                        params.push(idv.clone());
                    }
                    // optional comma
                    if let Some(Token::Symbol(com, _)) = self.peek() {
                        if com == "," {
                            self.next();
                            continue;
                        }
                    }
                    // continue to look for '->'
                    continue;
                }
                _ => break,
            }
        }

        // parse body expression
        let body = if saw_arrow {
            // parse expression until '}'
            let expr = self.parse_expr(0)?;
            expr
        } else {
            // no explicit params/arrow: treat content as single expression, implicit `it`
            let expr = self.parse_expr(0)?;
            // if params empty, set implicit 'it'
            if params.is_empty() {
                params.push("it".into());
            }
            expr
        };

        // consume optional '}'
        if let Some(Token::Symbol(c, _)) = self.peek() {
            if c == "}" {
                self.next();
            }
        }

        Some(Expr::Lambda {
            params,
            body: Box::new(body),
            span: span_start,
        })
    }

    pub fn parse_top_level(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(tok) = self.peek() {
            match tok {
                Token::Keyword(k, span) if k == "package" => {
                    // consume keyword
                    self.next();
                    // expect identifier or dotted path
                    if let Some(tk) = self.next() {
                        match tk {
                            Token::Ident(name, _) | Token::Keyword(name, _) => {
                                nodes.push(Node::Package {
                                    name: name.clone(),
                                    span: span.clone(),
                                })
                            }
                            _ => nodes.push(Node::Other { span: span.clone() }),
                        }
                    } else {
                        nodes.push(Node::Other { span: span.clone() });
                    }
                }
                Token::Keyword(k, span) if k == "import" => {
                    self.next();
                    if let Some(tk) = self.next() {
                        match tk {
                            Token::Ident(name, _) | Token::Keyword(name, _) => {
                                nodes.push(Node::Import {
                                    path: name.clone(),
                                    span: span.clone(),
                                })
                            }
                            _ => nodes.push(Node::Other { span: span.clone() }),
                        }
                    } else {
                        nodes.push(Node::Other { span: span.clone() });
                    }
                }
                Token::Keyword(k, span) if k == "fun" => {
                    self.next();
                    // Expect function name
                    let mut fname = None;
                    if let Some(tk) = self.next() {
                        match tk {
                            Token::Ident(name, _) | Token::Keyword(name, _) => {
                                fname = Some(name.clone())
                            }
                            _ => {}
                        }
                    }

                    // parse optional parameter list
                    let mut params = Vec::new();
                    if let Some(Token::Symbol(s, _)) = self.peek() {
                        if s == "(" {
                            // consume '('
                            self.next();
                            // parse simple comma-separated id list
                            loop {
                                match self.peek() {
                                    Some(Token::Symbol(sym, _)) if sym == ")" => {
                                        self.next();
                                        break;
                                    }
                                    Some(Token::Ident(_, _)) => {
                                        // consume identifier as parameter name
                                        if let Some(Token::Ident(n2, _)) = self.next() {
                                            params.push(n2.clone());
                                            // if type annotation follows, skip tokens until comma or ')' to ignore types
                                            if let Some(Token::Symbol(col, _)) = self.peek() {
                                                if col == ":" {
                                                    // consume ':'
                                                    self.next();
                                                    // skip tokens until comma or closing paren
                                                    loop {
                                                        match self.peek() {
                                                            Some(Token::Symbol(s2, _))
                                                                if s2 == "," || s2 == ")" =>
                                                            {
                                                                break;
                                                            }
                                                            Some(_) => {
                                                                self.next();
                                                            }
                                                            None => break,
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // consume optional comma
                                        if let Some(Token::Symbol(c, _)) = self.peek() {
                                            if c == "," {
                                                self.next();
                                            }
                                        }
                                    }
                                    Some(_) => {
                                        // skip unexpected token
                                        self.next();
                                    }
                                    None => break,
                                }
                            }
                        }
                    }

                    // optional return type or expression-bodied function
                    let mut return_type: Option<String> = None;
                    let mut expr_body: Option<Expr> = None;
                    let mut body: Vec<Stmt> = Vec::new();
                    if let Some(Token::Symbol(col, _)) = self.peek() {
                        if col == ":" {
                            // consume ':' and capture return type
                            self.next();
                            if let Some(tok) = self.next() {
                                match tok {
                                    Token::Ident(tn, _) | Token::Keyword(tn, _) => {
                                        return_type = Some(tn.clone());
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    // expression-bodied function: `= expr`
                    if let Some(Token::Symbol(eq, _)) = self.peek() {
                        if eq == "=" {
                            self.next();
                            if let Some(e) = self.parse_expr(0) {
                                expr_body = Some(e);
                            }
                            // consume optional semicolon
                            if let Some(Token::Symbol(sc, _)) = self.peek() {
                                if sc == ";" {
                                    self.next();
                                }
                            }
                        }
                    }

                    // now parse a block { ... } as function body if present
                    if expr_body.is_none() {
                        if let Some(Token::Symbol(sym, _)) = self.peek() {
                            if sym == "{" {
                                // consume '{'
                                self.next();
                                // parse until matching '}' (no nesting for now)
                                loop {
                                    match self.peek() {
                                        Some(Token::Symbol(s2, _)) if s2 == "}" => {
                                            self.next();
                                            break;
                                        }
                                        Some(Token::Keyword(k2, span2))
                                            if (k2 == "val" || k2 == "var") =>
                                        {
                                            let is_mut = k2 == "var";
                                            let span_k = span2.clone();
                                            self.next();
                                            if let Some(Token::Ident(name, _)) = self.next() {
                                                // optional type annotation: capture ": Type" if present
                                                let mut type_name: Option<String> = None;
                                                if let Some(Token::Symbol(col, _)) = self.peek() {
                                                    if col == ":" {
                                                        self.next(); // consume ':'
                                                        if let Some(tok) = self.next() {
                                                            match tok {
                                                                Token::Ident(tn2, _)
                                                                | Token::Keyword(tn2, _) => {
                                                                    type_name = Some(tn2.clone());
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                }

                                                // optional initializer
                                                let mut init = None;
                                                if let Some(Token::Symbol(eq, _)) = self.peek() {
                                                    if eq == "=" {
                                                        self.next();
                                                        if let Some(expr) = self.parse_expr(0) {
                                                            init = Some(expr);
                                                        }
                                                        // consume optional semicolon
                                                        if let Some(Token::Symbol(sc, _)) =
                                                            self.peek()
                                                        {
                                                            if sc == ";" {
                                                                self.next();
                                                            }
                                                        }
                                                    }
                                                }
                                                body.push(Stmt::VarDecl {
                                                    is_mut,
                                                    name: name.clone(),
                                                    type_name,
                                                    init,
                                                    span: span_k.clone(),
                                                });
                                            }
                                        }
                                        Some(Token::Keyword(k3, span3)) if k3 == "return" => {
                                            // consume 'return'
                                            self.next();
                                            let mut ret_expr = None;
                                            if let Some(e) = self.parse_expr(0) {
                                                ret_expr = Some(e);
                                            }
                                            // optional semicolon
                                            if let Some(Token::Symbol(sc, _)) = self.peek() {
                                                if sc == ";" {
                                                    self.next();
                                                }
                                            }
                                            body.push(Stmt::Return(ret_expr, span3.clone()));
                                        }
                                        Some(Token::Keyword(kw, kwspan))
                                            if kw == "for" || kw == "while" =>
                                        {
                                            // handle for and while
                                            if let Some(Token::Keyword(kword, kspan)) = self.next()
                                            {
                                                if kword == "while" {
                                                    // optional '('
                                                    if let Some(Token::Symbol(p, _)) = self.peek() {
                                                        if p == "(" {
                                                            self.next();
                                                        }
                                                    }
                                                    let cond = self.parse_expr(0).unwrap_or(
                                                        Expr::LitBool(false, kspan.clone()),
                                                    );
                                                    if let Some(Token::Symbol(p2, _)) = self.peek()
                                                    {
                                                        if p2 == ")" {
                                                            self.next();
                                                        }
                                                    }
                                                    // parse body
                                                    let mut body_stmts = Vec::new();
                                                    if let Some(Token::Symbol(b, _)) = self.peek() {
                                                        if b == "{" {
                                                            self.next();
                                                            loop {
                                                                match self.peek() {
                                                                    Some(Token::Symbol(s2, _))
                                                                        if s2 == "}" =>
                                                                    {
                                                                        self.next();
                                                                        break;
                                                                    }
                                                                    Some(_) => {
                                                                        if let Some(expr) =
                                                                            self.parse_expr(0)
                                                                        {
                                                                            if let Some(
                                                                                Token::Symbol(
                                                                                    sc,
                                                                                    _,
                                                                                ),
                                                                            ) = self.peek()
                                                                            {
                                                                                if sc == ";" {
                                                                                    self.next();
                                                                                }
                                                                            }
                                                                            let span = match &expr {
                                                                                Expr::LitStr(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::LitNumber(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::LitBool(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::Ident(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::Call {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Index {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Binary {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Unary {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Member {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::When {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Lambda {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                            };
                                                                            body_stmts.push(
                                                                                Stmt::ExprStmt(
                                                                                    expr, span,
                                                                                ),
                                                                            );
                                                                        } else {
                                                                            self.next();
                                                                        }
                                                                    }
                                                                    None => break,
                                                                }
                                                            }
                                                        } else {
                                                            if let Some(expr) = self.parse_expr(0) {
                                                                if let Some(Token::Symbol(sc, _)) =
                                                                    self.peek()
                                                                {
                                                                    if sc == ";" {
                                                                        self.next();
                                                                    }
                                                                }
                                                                let span = match &expr {
                                                                    Expr::LitStr(_, s) => s.clone(),
                                                                    Expr::LitNumber(_, s) => {
                                                                        s.clone()
                                                                    }
                                                                    Expr::LitBool(_, s) => {
                                                                        s.clone()
                                                                    }
                                                                    Expr::Ident(_, s) => s.clone(),
                                                                    Expr::Call { span, .. } => {
                                                                        span.clone()
                                                                    }
                                                                    Expr::Index {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::Binary {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::Unary {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::Member {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::When { span, .. } => {
                                                                        span.clone()
                                                                    }
                                                                    Expr::Lambda {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                };
                                                                body_stmts.push(Stmt::ExprStmt(
                                                                    expr, span,
                                                                ));
                                                            }
                                                        }
                                                    }
                                                    body.push(Stmt::While {
                                                        cond,
                                                        body: body_stmts,
                                                        span: kspan.clone(),
                                                    });
                                                } else if kword == "for" {
                                                    // optional '('
                                                    if let Some(Token::Symbol(p, _)) = self.peek() {
                                                        if p == "(" {
                                                            self.next();
                                                        }
                                                    }
                                                    // expect identifier
                                                    let mut var_name = "<anon>".to_string();
                                                    if let Some(Token::Ident(nm, _)) = self.next() {
                                                        var_name = nm.clone();
                                                    }
                                                    // expect 'in'
                                                    if let Some(Token::Keyword(ink, _)) =
                                                        self.peek()
                                                    {
                                                        if ink == "in" {
                                                            self.next();
                                                        }
                                                    } else if let Some(Token::Ident(ink2, _)) =
                                                        self.peek()
                                                    {
                                                        if ink2 == "in" {
                                                            self.next();
                                                        }
                                                    }
                                                    let iterable = self.parse_expr(0).unwrap_or(
                                                        Expr::LitStr(String::new(), kspan.clone()),
                                                    );
                                                    if let Some(Token::Symbol(p2, _)) = self.peek()
                                                    {
                                                        if p2 == ")" {
                                                            self.next();
                                                        }
                                                    }
                                                    // parse body
                                                    let mut body_stmts = Vec::new();
                                                    if let Some(Token::Symbol(b, _)) = self.peek() {
                                                        if b == "{" {
                                                            self.next();
                                                            loop {
                                                                match self.peek() {
                                                                    Some(Token::Symbol(s2, _))
                                                                        if s2 == "}" =>
                                                                    {
                                                                        self.next();
                                                                        break;
                                                                    }
                                                                    Some(_) => {
                                                                        if let Some(expr) =
                                                                            self.parse_expr(0)
                                                                        {
                                                                            if let Some(
                                                                                Token::Symbol(
                                                                                    sc,
                                                                                    _,
                                                                                ),
                                                                            ) = self.peek()
                                                                            {
                                                                                if sc == ";" {
                                                                                    self.next();
                                                                                }
                                                                            }
                                                                            let span = match &expr {
                                                                                Expr::LitStr(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::LitNumber(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::LitBool(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::Ident(
                                                                                    _,
                                                                                    s,
                                                                                ) => s.clone(),
                                                                                Expr::Call {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Index {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Binary {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Unary {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Member {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::When {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                                Expr::Lambda {
                                                                                    span,
                                                                                    ..
                                                                                } => span.clone(),
                                                                            };
                                                                            body_stmts.push(
                                                                                Stmt::ExprStmt(
                                                                                    expr, span,
                                                                                ),
                                                                            );
                                                                        } else {
                                                                            self.next();
                                                                        }
                                                                    }
                                                                    None => break,
                                                                }
                                                            }
                                                        } else {
                                                            if let Some(expr) = self.parse_expr(0) {
                                                                if let Some(Token::Symbol(sc, _)) =
                                                                    self.peek()
                                                                {
                                                                    if sc == ";" {
                                                                        self.next();
                                                                    }
                                                                }
                                                                let span = match &expr {
                                                                    Expr::LitStr(_, s) => s.clone(),
                                                                    Expr::LitNumber(_, s) => {
                                                                        s.clone()
                                                                    }
                                                                    Expr::LitBool(_, s) => {
                                                                        s.clone()
                                                                    }
                                                                    Expr::Ident(_, s) => s.clone(),
                                                                    Expr::Call { span, .. } => {
                                                                        span.clone()
                                                                    }
                                                                    Expr::Index {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::Binary {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::Unary {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::Member {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                    Expr::When { span, .. } => {
                                                                        span.clone()
                                                                    }
                                                                    Expr::Lambda {
                                                                        span, ..
                                                                    } => span.clone(),
                                                                };
                                                                body_stmts.push(Stmt::ExprStmt(
                                                                    expr, span,
                                                                ));
                                                            }
                                                        }
                                                    }
                                                    body.push(Stmt::For {
                                                        var: var_name,
                                                        iterable,
                                                        body: body_stmts,
                                                        span: kspan.clone(),
                                                    });
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            if let Some(expr) = self.parse_expr(0) {
                                                // consume optional semicolon
                                                if let Some(Token::Symbol(sc, _)) = self.peek() {
                                                    if sc == ";" {
                                                        self.next();
                                                    }
                                                }
                                                let span = match &expr {
                                                    Expr::LitStr(_, s) => s.clone(),
                                                    Expr::LitNumber(_, s) => s.clone(),
                                                    Expr::LitBool(_, s) => s.clone(),
                                                    Expr::Ident(_, s) => s.clone(),
                                                    Expr::Call { span, .. } => span.clone(),
                                                    Expr::Index { span, .. } => span.clone(),
                                                    Expr::Binary { span, .. } => span.clone(),
                                                    Expr::Unary { span, .. } => span.clone(),
                                                    Expr::Member { span, .. } => span.clone(),
                                                    Expr::When { span, .. } => span.clone(),
                                                    Expr::Lambda { span, .. } => span.clone(),
                                                };
                                                body.push(Stmt::ExprStmt(expr, span));
                                            } else {
                                                // skip token
                                                self.next();
                                            }
                                        }
                                        None => break,
                                    }
                                }
                            }
                        }
                    }

                    let name = fname.unwrap_or_else(|| "<anon>".into());
                    nodes.push(Node::Function {
                        name,
                        params,
                        return_type,
                        body,
                        expr_body,
                        span: span.clone(),
                    });
                }
                Token::Keyword(k, span) if k == "val" || k == "var" => {
                    let is_mut = k == "var";
                    self.next();
                    if let Some(tk) = self.next() {
                        match tk {
                            Token::Ident(name, _) | Token::Keyword(name, _) => {
                                // optional type annotation
                                let mut type_name: Option<String> = None;
                                if let Some(Token::Symbol(col, _)) = self.peek() {
                                    if col == ":" {
                                        self.next();
                                        if let Some(tok) = self.next() {
                                            match tok {
                                                Token::Ident(tn2, _) | Token::Keyword(tn2, _) => {
                                                    type_name = Some(tn2.clone());
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                // optional initializer
                                let mut init = None;
                                if let Some(Token::Symbol(eq, _)) = self.peek() {
                                    if eq == "=" {
                                        self.next();
                                        if let Some(expr) = self.parse_expr(0) {
                                            init = Some(expr);
                                        }
                                    }
                                }
                                nodes.push(Node::Variable {
                                    is_mut,
                                    name: name.clone(),
                                    type_name,
                                    init,
                                    span: span.clone(),
                                });
                            }
                            _ => nodes.push(Node::Other { span: span.clone() }),
                        }
                    } else {
                        nodes.push(Node::Other { span: span.clone() });
                    }
                }
                Token::Keyword(k, span) if k == "class" => {
                    self.next();
                    if let Some(tk) = self.next() {
                        match tk {
                            Token::Ident(name, _) | Token::Keyword(name, _) => {
                                nodes.push(Node::Class {
                                    name: name.clone(),
                                    span: span.clone(),
                                })
                            }
                            _ => nodes.push(Node::Other { span: span.clone() }),
                        }
                    } else {
                        nodes.push(Node::Other { span: span.clone() });
                    }
                }
                _ => {
                    // consume and ignore
                    if let Some(t) = self.next() {
                        // determine span for Other
                        let span = match t {
                            Token::Ident(_, s) => s.clone(),
                            Token::Keyword(_, s) => s.clone(),
                            Token::Number(_, s) => s.clone(),
                            Token::Str(_, s) => s.clone(),
                            Token::Char(_, s) => s.clone(),
                            Token::Symbol(_, s) => s.clone(),
                        };
                        nodes.push(Node::Other { span });
                    } else {
                        break;
                    }
                }
            }
        }
        nodes
    }

    // Public wrapper to parse a single expression from tokens (used by external helpers)
    pub fn parse_single_expr(&mut self) -> Option<Expr> {
        self.parse_expr(0)
    }
}
