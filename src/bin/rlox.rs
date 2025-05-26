use std::error::Error;
use std::{env, fmt, fs, process};

use ya_rlox::{parser::Parser, scanner::Scanner};

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
    //
    fn run(self) -> Result<(), BaseError> {
        // if self.scanner.trim() == "err" {
        //     let err = BaseError::new(1, "test.lox".to_string(), "Simulated error".to_string());
        //
        //     return Err(err);
        // }
        let tokens = Scanner::new(self.source).scan_tokens();
        println!("{:#?}", tokens);
        let ast = Parser::new(tokens).parse();
        println!("{:#?}", ast);

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct BaseError {
    line: usize,
    where_: String,
    msg: String,
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.where_, self.msg
        )
    }
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

impl Error for BaseError {}
