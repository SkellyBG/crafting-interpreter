use crate::{expr::Expr, token::Token};

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
        let expr = self.comparison();
    }

    fn comparison(&mut self) -> Expr {}

    fn term(&mut self) -> Expr {}

    fn factor(&mut self) -> Expr {}

    fn unary(&mut self) -> Expr {}

    fn primary(&mut self) -> Expr {}
}
