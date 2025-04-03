use crate::{
    expr::{BinOp, Expr},
    token::{self, Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
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
        let left = self.factor();

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous()
            
        }

        left
    }

    fn factor(&mut self) -> Expr {}

    fn unary(&mut self) -> Expr {}

    fn primary(&mut self) -> Expr {}

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
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
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
