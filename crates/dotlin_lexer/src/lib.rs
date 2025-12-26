use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+|//.*|/\*([^*]|\*[^/])*\*/")] // Skip whitespace and comments
pub enum Token {
    #[token("fun")]
    Fun,

    #[token("val")]
    Val,

    #[token("var")]
    Var,

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,

    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("return")]
    Return,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_string())]
    String(String),

    #[regex("'([^'\\\\]|\\[\\'bnfrt]|u[a-fA-F0-9]{4})'", |lex| lex.slice().chars().nth(1).unwrap_or('\0'))]
    Char(char),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Float(f64),

    #[regex("-?[0-9]+", |lex| lex.slice().parse().map_err(|_| ()))]
    Integer(i64),

    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEqual,
    #[token("-")]
    Minus,
    #[token("-=")]
    MinusEqual,
    #[token("*")]
    Star,
    #[token("*=")]
    StarEqual,
    #[token("/")]
    Slash,
    #[token("/=")]
    SlashEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    DoubleEqual,
    #[token("!=")]
    NotEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("!")]
    Not,
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token(".")]
    Dot,

    // Error fallback
    // In Logos 0.14+, we handle errors by checking the Result from next()
    Error,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) => Some(Token::Error),
            None => None,
        }
    }
}
