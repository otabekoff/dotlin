use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String, Span),
    Keyword(String, Span),
    Number(String, Span),
    Str(String, Span),
    Char(char, Span),
    Symbol(String, Span),
}
