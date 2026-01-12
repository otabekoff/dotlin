//! Parser integration and a tiny AST for Dotlin.
use dotlin_lexer::{Token, lex};
pub mod ast;
mod parser;

pub use ast::Node;

/// Produce tokens from source (integration shim).
pub fn parse_to_tokens(src: &str) -> Vec<Token> {
    lex(src)
}

/// Parse source into a small vector of top-level AST nodes.
pub fn parse_to_ast(src: &str) -> Vec<Node> {
    let toks = lex(src);
    let mut p = parser::Parser::new(&toks);
    p.parse_top_level()
}

#[cfg(test)]
mod tests {
    use super::{Node, parse_to_ast, parse_to_tokens};
    use dotlin_lexer::Token;

    #[test]
    fn parser_integration_returns_tokens() {
        let src = "let x = 1; /*comment*/ x = x + 2;";
        let toks = parse_to_tokens(src);
        // expecting at least some identifier and number tokens
        assert!(toks.iter().any(|t| matches!(t, Token::Ident(_, _))));
        assert!(toks.iter().any(|t| matches!(t, Token::Number(_, _))));
    }

    #[test]
    fn basic_syntax_sample_parsing() {
        let src = r#"
            package my.demo
            import dotlin.text.*

            fun main(args: Array<String>) {
                println("Hello world!")
            }

            val x = 5
            var y = x + 1

            class Rectangle(val height: Double, val length: Double) {
                val perimeter = (height + length) * 2
            }
        "#;
        let nodes = parse_to_ast(src);
        // ensure some nodes are present
        assert!(nodes.iter().any(|n| matches!(n, Node::Package { .. })));
        assert!(nodes.iter().any(|n| matches!(n, Node::Import { .. })));
        assert!(nodes.iter().any(|n| matches!(n, Node::Function { .. })));
        assert!(nodes.iter().any(|n| matches!(n, Node::Variable { .. })));
        assert!(nodes.iter().any(|n| matches!(n, Node::Class { .. })));
    }
}
