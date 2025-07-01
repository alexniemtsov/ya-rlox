use std::error::Error;
use std::{env, fs, process};

use ya_rlox::{err::LoxError, parser::Parser, scanner::Scanner};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let result = match args.len() {
        0 => run_file("test.lox"),
        // 0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: jlox [script]");
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(65);
    }

    println!("Interpreter executed successfully");
}
fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    // todo: implement batch processing of source
    let source = fs::read_to_string(path)?;
    let lox = Lox::new(source);
    lox.run()?;

    Ok(())
}

// fn run_prompt() -> Result<(), Box<dyn Error>> {
//     let s_in = io::stdin();
//     loop {
//         print!("> ");
//         let mut line = String::new();
//         io::Write::flush(&mut io::stdout())?;
//         let bytes_read = s_in.read_line(&mut line)?;
//         if bytes_read == 0 {
//             break; // EOF (Ctrl+D, Ctrl+Z)
//         }
//
//         let lox = Lox::new(line);
//         if let Err(e) = lox.run() {
//             eprintln!("Runtime error: {e}")
//         }
//     }
//
//     Ok(())
// }

struct Lox {
    source: String,
}

impl Lox {
    fn new(source: String) -> Self {
        Self { source }
    }

    // Running pipeline:
    // Lox owns `source`.
    // Lox passes ownership over `source` to Scanner
    // Scanner consumes source and return Tokens back
    // Parser consumes tokens and return AST.
    fn run(self) -> Result<(), LoxError> {
        println!("Source: \"{}\"", self.source);

        // todo: scan could implement Iterator
        let tokens = Scanner::new(self.source).scan_tokens();
        println!("{:#?}", tokens);
        match Parser::new(tokens).parse() {
            Ok(a) => println!("{:#?}", a),
            Err(e) => e.report(),
        };

        Ok(())
    }
}
