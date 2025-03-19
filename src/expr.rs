use crate::token::Token;

enum Expr {
    Literal,
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Box<Token>,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Box<Token>,
        right: Box<Expr>,
    },
    Operator,
}
