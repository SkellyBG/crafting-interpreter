use std::fmt::Display;

use crate::token::Token;

#[derive(Debug)]
pub(super) enum Literal {
    Number(i64),
    String(String),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "{value}"),
            Literal::String(value) => write!(f, "\"{value}\""),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub(super) enum BinOp {
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

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            BinOp::EqualEqual => "==",
            BinOp::BangEqual => "!=",
            BinOp::Less => "<",
            BinOp::LessEqual => "<=",
            BinOp::Greater => ">",
            BinOp::GreaterEqual => ">=",
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Star => "*",
            BinOp::Slash => "/",
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub(super) enum UnOp {
    Minus,
    Bang,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            UnOp::Minus => "-",
            UnOp::Bang => "!",
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub(super) enum Expr {
    Literal(Literal),
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
    Variable {
        token: Token,
    },
    Assign {
        token: Token,
        value: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
            Expr::Variable { token } => write!(f, "{}", token),
            Expr::Assign { token, value } => write!(f, "({} {})", token, value),
        }
    }
}

#[derive(Debug)]

pub(super) enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    Block(Vec<Decl>),
}

#[derive(Debug)]

pub(super) enum Decl {
    VarDecl {
        identifier: Token,
        initializer: Option<Expr>,
    },
    Stmt(Stmt),
}
