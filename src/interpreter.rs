use std::fmt::Display;

use crate::{
    expr::{BinOp, Expr, Literal, UnOp},
    Lox,
};

#[derive(Debug, PartialEq, Eq)]
enum Value {
    String(String),
    Number(i64),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            Value::String(v) => v,
            Value::Number(v) => &v.to_string(),
            Value::Boolean(v) => &v.to_string(),
            Value::Nil => "nil",
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
struct RuntimeError {
    message: String,
}
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expression: Expr) {
        match self.evaluate(expression) {
            Ok(v) => println!("{}", v),
            Err(e) => Lox::runtime_error(&e.message),
        }
    }

    fn evaluate(&self, expression: Expr) -> Result<Value, RuntimeError> {
        match expression {
            Expr::Literal(literal) => match literal {
                Literal::Number(v) => Ok(Value::Number(v)),
                Literal::String(v) => Ok(Value::String(v)),
                Literal::True => Ok(Value::Boolean(true)),
                Literal::False => Ok(Value::Boolean(false)),
                Literal::Nil => Ok(Value::Nil),
            },
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Unary { operator, right } => {
                let value = self.evaluate(*right)?;
                match operator {
                    UnOp::Minus => match value {
                        Value::Number(v) => Ok(Value::Number(-v)),
                        _ => Err(RuntimeError {
                            message: "Operand must be a number!".into(),
                        }),
                    },
                    UnOp::Bang => Ok(Value::Boolean(!self.is_truthy(&value))),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(*left)?;
                let right_value = self.evaluate(*right)?;

                match (&left_value, &right_value) {
                    (Value::Number(l_v), Value::Number(r_v)) => match operator {
                        BinOp::EqualEqual => Ok(Value::Boolean(l_v == r_v)),
                        BinOp::BangEqual => Ok(Value::Boolean(l_v != r_v)),
                        BinOp::Less => Ok(Value::Boolean(l_v < r_v)),
                        BinOp::LessEqual => Ok(Value::Boolean(l_v <= r_v)),
                        BinOp::Greater => Ok(Value::Boolean(l_v > r_v)),
                        BinOp::GreaterEqual => Ok(Value::Boolean(l_v >= r_v)),
                        BinOp::Plus => Ok(Value::Number(l_v + r_v)),
                        BinOp::Minus => Ok(Value::Number(l_v - r_v)),
                        BinOp::Star => Ok(Value::Number(l_v * r_v)),
                        BinOp::Slash => Ok(Value::Number(l_v / r_v)),
                    },
                    (Value::String(l_v), Value::String(r_v)) => match operator {
                        BinOp::EqualEqual => Ok(Value::Boolean(l_v == r_v)),
                        BinOp::BangEqual => Ok(Value::Boolean(l_v != r_v)),
                        BinOp::Plus => Ok(Value::String(format!("{l_v}{r_v}"))),
                        _ => Err(RuntimeError {
                            message: "Operand must be a number!".into(),
                        }),
                    },
                    _ => match operator {
                        BinOp::EqualEqual => Ok(Value::Boolean(left_value == right_value)),
                        BinOp::BangEqual => Ok(Value::Boolean(left_value != right_value)),
                        _ => Err(RuntimeError {
                            message: "Operands must be two numbers or two strings.".into(),
                        }),
                    },
                }
            }
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(v) => *v,
            _ => true,
        }
    }
}
