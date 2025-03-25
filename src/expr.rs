use std::fmt::Display;

pub(super) enum Literal {
    Number(i64),
    String(String),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Literal::Number(value) => &value.to_string(),
            Literal::String(value) => value,
            Literal::True => "true",
            Literal::False => "false",
            Literal::Nil => "nil",
        };

        write!(f, "{}", string)
    }
}

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
        }
    }
}
