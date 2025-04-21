use std::mem::discriminant;

use crate::{
    expr::{BinOp, Expr, Literal, UnOp},
    token::{Token, TokenType},
    token_error,
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

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
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

        token_error(self.peek(), "Expect expression.".to_owned());
        Err(ParserError)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<TokenType, ParserError> {
        if self.check(&token_type) {
            return Ok(self.advance().token_type);
        }

        token_error(self.peek(), message.to_owned());
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
