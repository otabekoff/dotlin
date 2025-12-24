use dotlin_ast::*;
use dotlin_lexer::{Lexer, Token};
use std::iter::Peekable;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Expected identifier, found: {0:?}")]
    ExpectedIdentifier(Token),
}

#[derive(Debug)]
pub enum ReplNode {
    Decl(Declaration),
    Stmt(Statement),
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            lexer: Lexer::new(code).peekable(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    fn advance(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.advance() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();
        while self.peek().is_some() {
            declarations.push(self.parse_declaration()?);
        }
        Ok(Program { declarations })
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        match self.peek() {
            Some(Token::Fun) => {
                let func = self.parse_function()?;
                Ok(Declaration::Function(func))
            }
            Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
        self.expect(Token::Fun)?;

        let name = match self.advance() {
            Some(Token::Identifier(id)) => id,
            Some(t) => return Err(ParseError::ExpectedIdentifier(t)),
            None => return Err(ParseError::UnexpectedEOF),
        };

        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if self.peek() != Some(&Token::RParen) {
            loop {
                let p_name = match self.advance() {
                    Some(Token::Identifier(id)) => id,
                    Some(t) => return Err(ParseError::ExpectedIdentifier(t)),
                    None => return Err(ParseError::UnexpectedEOF),
                };
                self.expect(Token::Colon)?;
                let p_type = self.parse_type()?;
                params.push(Param {
                    name: p_name,
                    typ: p_type,
                });

                if self.peek() == Some(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;

        let mut return_type = None;
        if self.peek() == Some(&Token::Colon) {
            self.advance();
            return_type = Some(self.parse_type()?);
        }

        let body = self.parse_block()?;

        Ok(FunctionDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let base_type = match self.advance() {
            Some(Token::Identifier(id)) => {
                if id == "Array" {
                    // Parse Array<T> syntax
                    if self.peek() == Some(&Token::Less) {
                        self.advance(); // consume <
                        let element_type = self.parse_type()?;
                        self.expect(Token::Greater)?;
                        return Ok(Type::Array(Box::new(element_type)));
                    } else {
                        // Just a regular identifier
                        Type::Named(id)
                    }
                } else if id == "Map" {
                    // Parse Map<K, V> syntax
                    if self.peek() == Some(&Token::Less) {
                        self.advance(); // consume <
                        let key_type = self.parse_type()?;
                        self.expect(Token::Comma)?;
                        let value_type = self.parse_type()?;
                        self.expect(Token::Greater)?;
                        return Ok(Type::Map(Box::new(key_type), Box::new(value_type)));
                    } else {
                        // Just a regular identifier
                        Type::Named(id)
                    }
                } else {
                    Type::Named(id)
                }
            }
            Some(t) => return Err(ParseError::UnexpectedToken(t)),
            None => return Err(ParseError::UnexpectedEOF),
        };
        
        // Check for array syntax like Int[]
        if self.peek() == Some(&Token::LBracket) {
            self.advance(); // consume [
            self.expect(Token::RBracket)?; // expect and consume ]
            return Ok(Type::Array(Box::new(base_type)));
        }
        
        Ok(base_type)
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            if token == &Token::RBrace {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::RBrace)?;
        Ok(Block { statements })
    }

    pub fn parse_repl_input(&mut self) -> Result<ReplNode, ParseError> {
        if self.peek() == Some(&Token::Fun) {
            let decl = self.parse_declaration()?;
            Ok(ReplNode::Decl(decl))
        } else {
            let stmt = self.parse_statement()?;
            Ok(ReplNode::Stmt(stmt))
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        if self.peek() == Some(&Token::LBrace) {
            return Ok(Statement::Block(self.parse_block()?));
        }

        if self.peek() == Some(&Token::If) {
            self.advance(); // if
            self.expect(Token::LParen)?;
            let condition = self.parse_expression()?;
            self.expect(Token::RParen)?;
            let then_branch = Box::new(self.parse_statement()?);
            let else_branch = if self.peek() == Some(&Token::Else) {
                self.advance();
                Some(Box::new(self.parse_statement()?))
            } else {
                None
            };
            return Ok(Statement::If {
                condition,
                then_branch,
                else_branch,
            });
        }

        if self.peek() == Some(&Token::While) {
            self.advance();
            self.expect(Token::LParen)?;
            let condition = self.parse_expression()?;
            self.expect(Token::RParen)?;
            let body = Box::new(self.parse_statement()?);
            return Ok(Statement::While { condition, body });
        }

        if self.peek() == Some(&Token::Return) {
            self.advance();
            let mut value = None;
            if let Some(token) = self.peek() {
                if token != &Token::RBrace {
                    value = Some(self.parse_expression()?);
                }
            }
            return Ok(Statement::Return(value));
        }

        // Simple statement parsing logic
        if self.peek() == Some(&Token::Val) || self.peek() == Some(&Token::Var) {
            // Handle variable declaration
            self.advance(); // consume val/var

            let name = match self.advance() {
                Some(Token::Identifier(id)) => id,
                t => return Err(ParseError::UnexpectedToken(t.unwrap_or(Token::Error))),
            };

            let initializer = if self.peek() == Some(&Token::Equal) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };

            Ok(Statement::VariableDecl {
                name,
                typ: None,
                initializer,
            })
        } else {
            let expr = self.parse_expression()?;
            Ok(Statement::Expression(expr))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_equality()?;

        if self.peek() == Some(&Token::Equal) {
            self.advance();
            let value = self.parse_expression()?; // Right-associative

            if let ExpressionKind::Variable(name) = *expr.kind {
                return Ok(Expression::new(ExpressionKind::Assignment { name, value }));
            } else {
                // Using UnexpectedToken for now, ideally "Invalid assignment target"
                return Err(ParseError::UnexpectedToken(Token::Equal));
            }
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        while let Some(token) = self.peek() {
            match token {
                Token::DoubleEqual | Token::NotEqual => {
                    let op = match self.advance().unwrap() {
                        Token::DoubleEqual => BinaryOp::Equal,
                        Token::NotEqual => BinaryOp::NotEqual,
                        _ => unreachable!(),
                    };
                    let right = self.parse_comparison()?;
                    expr = Expression::new(ExpressionKind::Binary {
                        left: expr,
                        operator: op,
                        right,
                    });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => {
                    let op = match self.advance().unwrap() {
                        Token::Less => BinaryOp::Less,
                        Token::LessEqual => BinaryOp::LessEqual,
                        Token::Greater => BinaryOp::Greater,
                        Token::GreaterEqual => BinaryOp::GreaterEqual,
                        _ => unreachable!(),
                    };
                    let right = self.parse_term()?;
                    expr = Expression::new(ExpressionKind::Binary {
                        left: expr,
                        operator: op,
                        right,
                    });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = match self.advance().unwrap() {
                        Token::Plus => BinaryOp::Add,
                        Token::Minus => BinaryOp::Sub,
                        _ => unreachable!(),
                    };
                    let right = self.parse_factor()?;
                    expr = Expression::new(ExpressionKind::Binary {
                        left: expr,
                        operator: op,
                        right,
                    });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_unary()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Star | Token::Slash => {
                    let op = match self.advance().unwrap() {
                        Token::Star => BinaryOp::Mul,
                        Token::Slash => BinaryOp::Div,
                        _ => unreachable!(),
                    };
                    let right = self.parse_unary()?;
                    expr = Expression::new(ExpressionKind::Binary {
                        left: expr,
                        operator: op,
                        right,
                    });
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if let Some(token) = self.peek() {
            match token {
                Token::Minus | Token::Not => {
                    let op = match self.advance().unwrap() {
                        Token::Minus => UnaryOp::Minus,
                        Token::Not => UnaryOp::Not,
                        _ => unreachable!(),
                    };
                    let right = self.parse_unary()?;
                    return Ok(Expression::new(ExpressionKind::Unary {
                        operator: op,
                        operand: right,
                    }));
                }
                _ => {}
            }
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::LParen => {
                    self.advance(); // consume (
                    let mut args = Vec::new();
                    if self.peek() != Some(&Token::RParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.peek() == Some(&Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    expr = Expression::new(ExpressionKind::Call {
                        callee: expr,
                        arguments: args,
                    });
                }
                Token::Dot => {
                    self.advance(); // consume .
                    let member = match self.advance() {
                        Some(Token::Identifier(id)) => id,
                        Some(t) => return Err(ParseError::ExpectedIdentifier(t)),
                        None => return Err(ParseError::UnexpectedEOF),
                    };
                    expr = Expression::new(ExpressionKind::MemberAccess {
                        object: expr,
                        member,
                    });
                }
                Token::LBracket => {
                    // Parse array/map indexing: expr[expr]
                    self.advance(); // consume [
                    let index = self.parse_expression()?;
                    self.expect(Token::RBracket)?;
                    expr = Expression::new(ExpressionKind::Index {
                        array: expr,
                        index,
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.advance() {
            Some(Token::Integer(i)) => Ok(Expression::new(ExpressionKind::Literal(
                Literal::Integer(i),
            ))),
            Some(Token::Float(f)) => {
                Ok(Expression::new(ExpressionKind::Literal(Literal::Float(f))))
            }
            Some(Token::True) => Ok(Expression::new(ExpressionKind::Literal(Literal::Boolean(
                true,
            )))),
            Some(Token::False) => Ok(Expression::new(ExpressionKind::Literal(Literal::Boolean(
                false,
            )))),
            Some(Token::String(s)) => {
                // Remove quotes
                let content = s.trim_matches('"').to_string();
                Ok(Expression::new(ExpressionKind::Literal(Literal::String(
                    content,
                ))))
            }
            Some(Token::Identifier(id)) => Ok(Expression::new(ExpressionKind::Variable(id))),
            Some(Token::LParen) => {
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Some(Token::LBracket) => {
                // Parse array literal: [expr, expr, ...]
                let mut elements = Vec::new();
                if self.peek() != Some(&Token::RBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if self.peek() == Some(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RBracket)?;
                Ok(Expression::new(ExpressionKind::ArrayLiteral { elements }))
            }
            Some(Token::LBrace) => {
                // Parse HashMap literal: {key: value, key2: value2, ...}
                let mut pairs = Vec::new();
                if self.peek() != Some(&Token::RBrace) {
                    loop {
                        let key = self.parse_expression()?;
                        self.expect(Token::Colon)?; // Expect colon between key and value
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                        
                        if self.peek() == Some(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RBrace)?;
                Ok(Expression::new(ExpressionKind::HashMapLiteral { pairs }))
            }
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}
