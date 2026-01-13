use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Package {
        name: String,
        span: Span,
    },
    Import {
        path: String,
        span: Span,
    },
    Function {
        name: String,
        params: Vec<(String, Option<String>, Option<Expr>)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
        expr_body: Option<Expr>,
        span: Span,
    },
    Variable {
        is_mut: bool,
        name: String,
        type_name: Option<String>,
        init: Option<Expr>,
        span: Span,
    },
    Class {
        name: String,
        span: Span,
    },
    Other {
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    LitStr(String, Span),
    LitNumber(String, Span),
    LitBool(bool, Span),
    Ident(String, Span),
    Unary {
        op: String,
        expr: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    Member {
        receiver: Box<Expr>,
        name: String,
        span: Span,
    },
    Index {
        target: Box<Expr>,
        index: usize,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
        span: Span,
    },
    When {
        scrutinee: Option<Box<Expr>>,
        arms: Vec<(Pattern, Expr)>,
        span: Span,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    ExprStmt(Expr, Span),
    VarDecl {
        is_mut: bool,
        name: String,
        type_name: Option<String>,
        init: Option<Expr>,
        span: Span,
    },
    Return(Option<Expr>, Span),
    Block(Vec<Stmt>, Span),
    For {
        var: String,
        iterable: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    LitNumber(String, Span),
    LitStr(String, Span),
    LitBool(bool, Span),
    Range(String, String, Span),
    Array(Vec<Pattern>, Span),
    Bind(String, Span),
    IsBind(String, String, Span),
    InExpr(Expr, Span),
    IsType(String, Span),
    NotIsType(String, Span),
    Else(Span),
}

impl Node {
    pub fn span(&self) -> &Span {
        match self {
            Node::Package { span, .. } => span,
            Node::Import { span, .. } => span,
            Node::Function { span, .. } => span,
            Node::Variable { span, .. } => span,
            Node::Class { span, .. } => span,
            Node::Other { span } => span,
        }
    }
}
