mod scanner;
mod token;

use scanner::Scanner;

pub fn run(byte: String) {
    let mut scanner = Scanner::new(&byte);

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

pub fn error(line: u64, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: u64, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message)
}
