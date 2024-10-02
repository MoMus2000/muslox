mod expr;
mod interpreter;
mod parser;
mod scanner;
mod statement;
use interpreter::Interpreter;
use parser::Parser;

use crate::scanner::*;

use std::error::Error;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::{env, io, io::BufRead};

type LoxErr = Box<dyn Error>;

pub fn run_file(path: &str) -> Result<(), LoxErr> {
    let mut interpreter = Interpreter::new();
    let contents = fs::read_to_string(path)?;
    run(&mut interpreter, &contents)?;
    Ok(())
}

pub fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), LoxErr> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    interpreter.interpret(statements)?;
    Ok(())
}

pub fn run_prompt() -> Result<(), LoxErr> {
    let mut interpreter = Interpreter::new();
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
        run(&mut interpreter, &buffer)?;
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
