use std::error::Error;
//use std::io::Write;
use std::{env, fs, io, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => {
            if let Err(e) = run_prompt() {
                eprintln!("Error running prompt: {e}");
                process::exit(1);
            }
        }
        1 => {
            if let Err(e) = run_file(&args[0]) {
                eprintln!("Error running file '{}': {e}", args[0]);
                process::exit(1);
            }
        }
        _ => {
            println!("Usage: jlox [script]");
            process::exit(1);
        }
    }

    println!("Interpreter executed successfully");
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let s_in = io::stdin();

    loop {
        print!("> ");
        //io::stdout().flush();
        let mut line = String::new();
        let bytes_read = s_in.read_line(&mut line)?;
        if bytes_read == 0 {
            break; // EOF (Ctrl+D, Ctrl+Z)
        }

        run(&line);
    }

    Ok(())
}

fn run(source: &str) {
    println!("Echo: {}", source.trim());
    // todo: run
    //
    // new scanner
    // scan str to tokens Vec<Token>
    //
}

struct Lox {
    code: String,

    _error: bool,
}

struct BaseError {
    line: usize,
    where_: String,
    msg: String,
}

impl BaseError {
    // todo: Change to `impl Into<String>
    fn new(line: usize, where_: String, msg: String) -> Self {
        Self { line, where_, msg }
    }

    fn report(&self) {
        eprintln!("[line {}] Error {}: {}", self.line, self.where_, self.msg);
    }
}
