use std::{error::Error, fmt, mem::discriminant};

use crate::{
    error,
    expr::{BinOp, Expr, Literal, UnOp},
    token::{Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
struct ParserError {
    token: Token,
    message: String,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut left = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
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

        left
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
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

        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor();
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

        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
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

        left
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator: match operator.token_type {
                    TokenType::Bang => UnOp::Bang,
                    TokenType::Minus => UnOp::Minus,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::False]) {
            return Expr::Literal(Literal::False);
        }

        if self.match_tokens(&[TokenType::True]) {
            return Expr::Literal(Literal::True);
        }

        if self.match_tokens(&[TokenType::Number(-1), TokenType::String("a".to_owned())]) {
            let literal = match self.previous().token_type {
                TokenType::Number(v) => Literal::Number(v),
                TokenType::String(v) => Literal::String(v),
                _ => unreachable!(),
            };
            return Expr::Literal(literal);
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        unreachable!()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<TokenType, ParserError> {
        if self.check(&token_type) {
            return Ok(self.advance().token_type);
        }

        Err(ParserError {
            token: self.peek(),
            message: message.to_owned(),
        })
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
}
