#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(FunctionDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named(String),
    // TODO: Generic types, Function types, etc.
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Block(Block),
    VariableDecl {
        name: String,
        typ: Option<Type>,
        initializer: Option<Expression>,
    },
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: Box<ExpressionKind>,
    pub resolved_type: Option<Type>,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self {
            kind: Box::new(kind),
            resolved_type: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Literal(Literal),
    Variable(String),
    Assignment {
        name: String,
        value: Expression,
    },
    Call {
        callee: Expression,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Expression,
        operator: BinaryOp,
        right: Expression,
    },
    Unary {
        operator: UnaryOp,
        operand: Expression,
    },
    MemberAccess {
        object: Expression,
        member: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Minus,
}
