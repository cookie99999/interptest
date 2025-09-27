use std::env;

mod token;
mod scanner;
use crate::scanner::Scanner;
mod expr;
mod parser;
use crate::parser::Parser;

fn main() {
    let mut had_error: bool = false;
    
    match env::args().len() {
	2 => run_file(env::args().nth(1).expect("No 1st argument even though the length matched 1...")),
	1 => run_prompt(),
	_ => panic!("Usage: interptest [path]"),
    }
}

fn run_file(path: String) {
    let buf: Vec<u8> = std::fs::read(path).unwrap();
    run(String::from_utf8(buf).expect("run_file: invalid UTF-8 sequence in buf"));
}

fn run_prompt() {
    loop {
	println!("ready");
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	run(line);
    }
}

fn run(text: String) {
    let mut s: Scanner = Scanner::new(text);
    s.scan_tokens();

    let mut p = Parser::new(s.tokens);
    let ast = p.parse().unwrap();
    println!("{}", ast.print());
    match ast.evaluate() {
	Ok(v) => println!("{v}"),
	Err(_) => {},
    };
}

fn prerror(line: u32, msg: &str) {
    report(line, "", msg);
}

fn report(line: u32, where_at: &str, msg: &str) {
    println!("{line}: Error {where_at}: {msg}");
}
