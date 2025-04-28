use crate::expr::{Expr, Literal};

enum Value {
    String,
    Number,
    Boolean,
    Nil,
}

struct Interpreter {}

impl Interpreter {
    fn interpret(expression: Expr) {}

    fn evaluate(&self, expression: Expr) -> Value {
        match expression {
            Expr::Literal(literal) => match literal {
                Literal::Number(_) => todo!(),
                Literal::String(_) => todo!(),
                Literal::True => todo!(),
                Literal::False => todo!(),
                Literal::Nil => todo!(),
            },
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Unary { operator, right } => todo!(),
            Expr::Binary {
                left,
                operator,
                right,
            } => todo!(),
        }
    }
}
