use std::collections::HashMap;
use std::rc::Rc;
use crate::prerror;

use crate::token::TokenType;
use crate::token::Token;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
	Scanner {
	    source: source,
	    tokens: Vec::<Token>::new(),
	    start: 0,
	    current: 0,
	    line: 1,
	    keywords: HashMap::from([
		("begin", TokenType::Begin),
		("end", TokenType::End),
		("procedure", TokenType::Procedure),
		("function", TokenType::Function),
		("return", TokenType::Return),
		("if", TokenType::If),
		("for", TokenType::For),
		("repeat", TokenType::Repeat),
		("until", TokenType::Until),
		("else", TokenType::Else),
		("do", TokenType::Do),
		("to", TokenType::To),
		("program", TokenType::Program),
		("true", TokenType::True),
		("false", TokenType::False),
		("and", TokenType::And),
		("or", TokenType::Or),
		("nil", TokenType::Nil),
		("print", TokenType::Print),
		("real", TokenType::Real),
		("int", TokenType::Int),
		("str", TokenType::Str),
		("bool", TokenType::Bool),
		]),
	}
    }

    pub fn scan_tokens(&mut self) {
	//let mut tokens = Vec::<Token>::new();
	while !self.is_at_end() {
	    self.start = self.current;
	    self.scan_token();
	}

	self.tokens.push(Token::new(TokenType::EOF, "\0".to_string(), self.line));
    }

    fn is_at_end(&self) -> bool {
	self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self) {
	let c: char = self.advance();
	let eq_next = self.peek() == '=';
	match c {
	    '(' => self.tokens.push(Token::new(TokenType::LParen, c.to_string(), self.line)),
	    ')' => self.tokens.push(Token::new(TokenType::RParen, c.to_string(), self.line)),
	    ',' => self.tokens.push(Token::new(TokenType::Comma, c.to_string(), self.line)),
	    '.' => self.tokens.push(Token::new(TokenType::Dot, c.to_string(), self.line)),
	    '-' => self.tokens.push(Token::new(TokenType::Minus, c.to_string(), self.line)),
	    '+' => self.tokens.push(Token::new(TokenType::Plus, c.to_string(), self.line)),
	    ';' => self.tokens.push(Token::new(TokenType::Semicolon, c.to_string(), self.line)),
	    '*' => self.tokens.push(Token::new(TokenType::Star, c.to_string(), self.line)),
	    '=' => self.tokens.push(Token::new(TokenType::Equal, c.to_string(), self.line)),
	    '!' => match eq_next {
		true => {
		    self.advance();
		    self.tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), self.line))
		},
		false => self.tokens.push(Token::new(TokenType::Bang, c.to_string(), self.line)),
	    },
	    '<' => match eq_next {
		true => {
		    self.advance();
		    self.tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), self.line))
		},
		false => self.tokens.push(Token::new(TokenType::Less, c.to_string(), self.line)),
	    },
	    '>' => match eq_next {
		true => {
		    self.advance();
		    self.tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), self.line))
		},
		false => self.tokens.push(Token::new(TokenType::Greater, c.to_string(), self.line)),
	    },
	    ':' => match eq_next {
		true => {
		    self.advance();
		    self.tokens.push(Token::new(TokenType::LetEqual, ":=".to_string(), self.line))
		},
		false => prerror(self.line, "Unexpected character following ':'"),
	    },
	    '/' => {
		if self.check('/') {
		    while self.peek() != '\n' && !self.is_at_end() {
			self.advance();
		    }
		} else {
		    self.tokens.push(Token::new(TokenType::Slash, c.to_string(), self.line));
		}
	    },
	    ' ' | '\r' | '\t' => {},
	    '\n' => self.line += 1,
	    '"' => self.string(),
	    _ => {
		if self.is_digit(c) {
		    self.number();
		} else if self.is_alpha(c) {
		    self.identifier();
		} else {
		    prerror(self.line, "Unexpected character");
		}
	    },
	};
    }

    fn advance(&mut self) -> char {
	let c = self.source.chars().nth(self.current).unwrap();
	self.current += 1;
	c
    }

    fn check(&mut self, c: char) -> bool {
	if self.is_at_end() {
	    return false;
	}
	if self.source.chars().nth(self.current).unwrap() != c {
	    return false;
	}
	self.current += 1;
	true
    }

    fn peek(&mut self) -> char {
	if self.is_at_end() {
	    return '\0';
	}
	return self.source.chars().nth(self.current).unwrap();
    }

    fn string(&mut self) {
	while self.peek() != '"' && !self.is_at_end() {
	    if self.peek() == '\n' {
		self.line += 1;
	    }
	    self.advance();
	}

	if self.is_at_end() {
	    prerror(self.line, "Unterminated string");
	}

	self.advance(); //closing quote

	let string: String = self.source[self.start + 1..self.current - 1].to_string();
	let lexeme: String = self.source[self.start..self.current].to_string();
	self.tokens.push(Token::new(TokenType::StrLit(Rc::new(string)), lexeme, self.line));
    }

    fn is_digit(&mut self, c: char) -> bool {
	c >= '0' && c <= '9'
    }

    fn is_alpha(&mut self, c: char) -> bool {
	(c >= 'a' && c <= 'z') ||
	    (c >= 'A' && c <= 'Z') ||
	    c == '_'
    }

    fn is_alphanumeric(&mut self, c: char) -> bool {
	self.is_digit(c) || self.is_alpha(c)
    }

    fn number(&mut self) {
	let mut was_float = false;
	while let c = self.peek() && self.is_digit(c) {
	    self.advance();
	}

	let c = self.peek_next();
	if self.peek() == '.' && self.is_digit(c) {
	    was_float = true;
	    self.advance();
	    while let c = self.peek() && self.is_digit(c) {
		self.advance();
	    }
	}

	let numstr = &self.source[self.start..self.current];
	match was_float {
	    true => {
		let number = numstr.parse::<f32>().unwrap();
		self.tokens.push(Token::new(TokenType::RealLit(number), numstr.to_string(), self.line));
	    },
	    false => {
		let number = numstr.parse::<u32>().unwrap();
		self.tokens.push(Token::new(TokenType::IntLit(number), numstr.to_string(), self.line));
	    },
	};
    }

    fn identifier(&mut self) {
	while let c = self.peek() && self.is_alphanumeric(c) {
	    self.advance();
	}

	let text = &self.source[self.start..self.current];
	match self.keywords.get(text) {
	    Some(t_type) => self.tokens.push(Token::new(t_type.clone(), text.to_string(), self.line)),
	    None => self.tokens.push(Token::new(TokenType::Ident, text.to_string(), self.line)),
	};
    }

    fn peek_next(&mut self) -> char {
	if self.current + 1 >= self.source.chars().count() {
	    return '\0';
	}
	return self.source.chars().nth(self.current + 1).unwrap();
    }
}
