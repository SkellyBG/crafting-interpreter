use std::fmt::Display;

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    String(String),
    Number(i64),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u64) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::Identifier(s) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, s),
            TokenType::String(s) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, s),
            TokenType::Number(n) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, n),
            _ => write!(f, "{:?} {}", self.token_type, self.lexeme),
        }
    }
}
