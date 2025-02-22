use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, BufReader},
    process,
};

use crafting_interpreter::run;

fn run_file(path: &str) -> Result<(), Box<dyn Error + 'static>> {
    let data: Vec<u8> = fs::read(path)?;

    run(String::from_utf8(data)?);

    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error + 'static>> {
    let stdin = io::stdin();
    let mut buffered_reader = BufReader::new(stdin);

    loop {
        print!("> ");
        let mut line = String::new();

        let size = buffered_reader.read_line(&mut line)?;

        if size == 0 {
            break Ok(());
        }

        run(line);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        2.. => {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        }
    }
}
