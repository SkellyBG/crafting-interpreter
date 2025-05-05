mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::{Token, TokenType};

pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        let interpreter = Interpreter::new();

        Self { interpreter }
    }

    pub fn run(&mut self, byte: &str) {
        let mut scanner = Scanner::new(byte);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens.to_vec());

        let statements = parser.parse();

        self.interpreter.interpret(statements);
    }

    pub fn error(line: u64, message: &str) {
        Self::report(line, "", message);
    }

    pub fn report(line: u64, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message)
    }

    pub fn token_error(token: Token, message: &str) {
        if token.token_type == TokenType::Eof {
            Self::report(token.line, " at end", message)
        } else {
            Self::report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    pub fn runtime_error(message: &str) {
        eprintln!("Runtime error: {}", message);
    }
}
