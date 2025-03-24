use crate::expr::Expr;

struct AstPrinter {}

impl AstPrinter {
    fn print(expr: Expr) {
        print!("{}", expr);
    }
}
