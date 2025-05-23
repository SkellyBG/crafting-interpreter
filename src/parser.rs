use std::mem::discriminant;

use crate::{
    intepreter_structs::{BinOp, Decl, Expr, Literal, Stmt, UnOp},
    token::{Token, TokenType},
    Lox,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

#[derive(Debug)]
struct ParserError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Decl> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }

        statements
    }

    fn declaration(&mut self) -> Option<Decl> {
        let res = if self.match_tokens(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement().map(Decl::Stmt)
        };

        if res.is_err() {
            self.synchronize();
        }
        res.ok()
    }

    fn var_declaration(&mut self) -> Result<Decl, ParserError> {
        let token = self.consume(
            TokenType::Identifier("a".to_owned()),
            "Expect variable name.",
        )?;

        let initializer = if self.match_tokens(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Decl::VarDecl {
            identifier: token,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_tokens(&[TokenType::Print]) {
            self.print_statement()
        } else {
            Ok(self.expression_statement()?)
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::PrintStmt(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::ExprStmt(value))
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: match operator.token_type {
                    TokenType::BangEqual => BinOp::BangEqual,
                    TokenType::EqualEqual => BinOp::EqualEqual,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: match operator.token_type {
                    TokenType::Greater => BinOp::Greater,
                    TokenType::GreaterEqual => BinOp::GreaterEqual,
                    TokenType::Less => BinOp::Less,
                    TokenType::LessEqual => BinOp::LessEqual,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: match operator.token_type {
                    TokenType::Plus => BinOp::Plus,
                    TokenType::Minus => BinOp::Minus,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: match operator.token_type {
                    TokenType::Slash => BinOp::Slash,
                    TokenType::Star => BinOp::Star,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: match operator.token_type {
                    TokenType::Bang => UnOp::Bang,
                    TokenType::Minus => UnOp::Minus,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::False));
        }

        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::True));
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_tokens(&[TokenType::Number(-1), TokenType::String("a".to_owned())]) {
            let literal = match self.previous().token_type {
                TokenType::Number(v) => Literal::Number(v),
                TokenType::String(v) => Literal::String(v),
                _ => unreachable!(),
            };
            return Ok(Expr::Literal(literal));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        if self.match_tokens(&[TokenType::Identifier("a".into())]) {
            return Ok(Expr::Variable {
                token: self.previous(),
            });
        }

        Lox::token_error(self.peek(), "Expect expression.");
        Err(ParserError)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Lox::token_error(self.peek(), message);
        Err(ParserError)
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && discriminant(&self.peek().token_type) == discriminant(token_type)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}
