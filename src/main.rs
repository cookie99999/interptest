use std::env;
use std::error::Error;

mod token;
mod scanner;
mod expr;
mod stmt;
mod typecheck;
mod parser;
mod environment;
mod interpreter;
use crate::scanner::Scanner;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::typecheck::TypeChecker;

fn main() {
    match env::args().len() {
	2 => run_file(env::args().nth(1).expect("No 1st argument even though the length matched 1...")),
	1 => run_prompt(),
	_ => panic!("Usage: interptest [path]"),
    }
}

fn run_file(path: String) {
    let mut i = Interpreter::new();
    let mut t = TypeChecker::new();
    let buf: Vec<u8> = std::fs::read(path).unwrap();
    match run(String::from_utf8(buf).expect("run_file: invalid UTF-8 sequence in buf"), &mut i, &mut t) {
	Ok(_) => {},
	Err(_) => println!("Finished with errors."),
    };
}

fn run_prompt() {
    let mut i = Interpreter::new();
    let mut t = TypeChecker::new();
    loop {
	println!("ready");
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	//do nothing with the return since the user will have just typed and seen a single line
	//and already seen any errors
	let _ = run(line, &mut i, &mut t);
    }
}

fn run(text: String, i: &mut Interpreter, t: &mut TypeChecker) -> Result<(), Box<dyn Error>> {
    let mut s: Scanner = Scanner::new(text);
    s.scan_tokens();

    let mut p = Parser::new(s.tokens);
    let ast = p.parse()?;
    for stmt in ast.iter() {
	println!("{}", stmt.print());
    }
    t.check(&ast)?;
    i.interpret(ast)?;
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
