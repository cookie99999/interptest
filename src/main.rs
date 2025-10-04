use std::env;
use std::error::Error;

mod token;
mod scanner;
mod expr;
mod stmt;
mod parser;
mod environment;
mod interpreter;
use crate::scanner::Scanner;
use crate::interpreter::Interpreter;
use crate::parser::Parser;

fn main() {
    match env::args().len() {
	2 => run_file(env::args().nth(1).expect("No 1st argument even though the length matched 1...")),
	1 => run_prompt(),
	_ => panic!("Usage: interptest [path]"),
    }
}

fn run_file(path: String) {
    let mut i = Interpreter::new();
    let buf: Vec<u8> = std::fs::read(path).unwrap();
    run(String::from_utf8(buf).expect("run_file: invalid UTF-8 sequence in buf"), &mut i);
}

fn run_prompt() {
    let mut i = Interpreter::new();
    loop {
	println!("ready");
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	run(line, &mut i);
    }
}

fn run(text: String, i: &mut Interpreter) -> Result<(), Box<dyn Error>> {
    let mut s: Scanner = Scanner::new(text);
    s.scan_tokens();

    let mut p = Parser::new(s.tokens);
    let ast = p.parse()?;
    i.interpret(ast);
    Ok(())
}

fn prerror(line: u32, msg: &str) {
    report(line, "", msg);
}

fn report(line: u32, where_at: &str, msg: &str) {
    println!("{line}: Error {where_at}: {msg}");
}

#[derive (Debug)]
struct RuntimeError {}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "runtime error")
    }
}

impl Error for RuntimeError {}
