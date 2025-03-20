use crate::token::Token;

enum BinOp {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

enum UnOp {
    Minus,
    Bang,
}

enum Expr {
    Literal,
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: UnOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: BinOp,
        right: Box<Expr>,
    },
    Operator,
}
