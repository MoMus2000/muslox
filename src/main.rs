mod expr;
mod parser;
mod scanner;
use parser::Parser;

use crate::scanner::*;

use std::error::Error;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::{env, io, io::BufRead};

type LoxErr = Box<dyn Error>;

pub fn run_file(path: &str) -> Result<(), LoxErr> {
    let contents = fs::read_to_string(path)?;
    run(&contents)?;
    Ok(())
}

pub fn run(contents: &str) -> Result<(), LoxErr> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let eval = parser.parse().evaluate()?;
    match eval {
        LiteralValue::StringValue(x) => {
            println!("> \"{}\"", x);
        }
        _ => println!("> {}", eval.to_string()),
    }
    Ok(())
}

pub fn run_prompt() -> Result<(), LoxErr> {
    loop {
        print!("> ");
        let mut buffer = String::new();
        io::stdout().flush()?;
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let n = handle.read_line(&mut buffer)?;
        if n <= 1 {
            return Ok(());
        }
        buffer = buffer.trim_end().to_string();
        run(&buffer)?;
    }
}
fn main() -> Result<(), LoxErr> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64)
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}
