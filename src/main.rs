use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, BufReader, Write},
    process,
};

use crafting_interpreter::Lox;

fn run_file(path: &str) -> Result<(), Box<dyn Error + 'static>> {
    let data: Vec<u8> = fs::read(path)?;

    Lox::new().run(&String::from_utf8(data)?);

    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error + 'static>> {
    let stdin = io::stdin();
    let mut buffered_reader = BufReader::new(stdin);

    let mut lox = Lox::new();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();

        let size = buffered_reader.read_line(&mut line)?;

        if size == 0 {
            break Ok(());
        }

        lox.run(&line);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        2.. => {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        }
    }
}
