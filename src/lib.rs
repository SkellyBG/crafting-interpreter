mod scanner;

use scanner::Scanner;

pub fn run(byte: String) {
    let scanner = Scanner::new(byte);

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn report(line: usize, location: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, location, message)
}
