mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::{Token, TokenType};

pub fn run(byte: String) {
    let mut scanner = Scanner::new(&byte);

    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());

    let interpreter = Interpreter::new();

    if let Some(expression) = parser.parse() {
        println!("{}", expression);
        interpreter.interpret(expression);
    }
}

pub fn error(line: u64, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: u64, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message)
}

pub fn token_error(token: Token, message: String) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end".to_owned(), message)
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), message);
    }
}
