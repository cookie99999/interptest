use std::collections::HashMap;
use crate::prerror;

use crate::token::TokenType;
use crate::token::Token;

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<String, TokenType>,
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
		("begin".to_string(), TokenType::Begin),
		("end".to_string(), TokenType::End),
		("procedure".to_string(), TokenType::Procedure),
		("function".to_string(), TokenType::Function),
		("return".to_string(), TokenType::Return),
		("if".to_string(), TokenType::If),
		("for".to_string(), TokenType::For),
		("repeat".to_string(), TokenType::Repeat),
		("until".to_string(), TokenType::Until),
		("else".to_string(), TokenType::Else),
		("do".to_string(), TokenType::Do),
		("to".to_string(), TokenType::To),
		("program".to_string(), TokenType::Program),
		("true".to_string(), TokenType::True),
		("false".to_string(), TokenType::False),
		("and".to_string(), TokenType::And),
		("or".to_string(), TokenType::Or),
		("nil".to_string(), TokenType::Nil),
		("print".to_string(), TokenType::Print),
		("real".to_string(), TokenType::Real),
		("str".to_string(), TokenType::Str),
		("bool".to_string(), TokenType::Bool),
		]),
	}
    }

    pub fn scan_tokens(&mut self) {
	//let mut tokens = Vec::<Token>::new();
	while !self.is_at_end() {
	    self.start = self.current;
	    self.scan_token();
	}

	self.tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), 0.0, self.line));
    }

    fn is_at_end(&self) -> bool {
	self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self) {
	let c: char = self.advance();
	let eq_next = self.check('=');
	match c {
	    '(' => self.add_token(TokenType::LParen),
	    ')' => self.add_token(TokenType::RParen),
	    ',' => self.add_token(TokenType::Comma),
	    '.' => self.add_token(TokenType::Dot),
	    '-' => self.add_token(TokenType::Minus),
	    '+' => self.add_token(TokenType::Plus),
	    ';' => self.add_token(TokenType::Semicolon),
	    '*' => self.add_token(TokenType::Star),
	    '=' => self.add_token(TokenType::Equal),
	    '!' => self.add_token(match eq_next {
		true => TokenType::BangEqual,
		false => TokenType::Bang,
	    }),
	    '<' => self.add_token(match eq_next {
		true => TokenType::LessEqual,
		false => TokenType::Less,
	    }),
	    '>' => self.add_token(match eq_next {
		true => TokenType::GreaterEqual,
		false => TokenType::Greater,
	    }),
	    ':' => match eq_next {
		true => self.add_token(TokenType::LetEqual),
		false => prerror(self.line, "Unexpected character following ':'"),
	    },
	    '/' => {
		if self.check('/') {
		    while self.peek() != '\n' && !self.is_at_end() {
			self.advance();
		    }
		} else {
		    self.add_token(TokenType::Slash);
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

    fn add_token(&mut self, t_type: TokenType) {
	self.add_token_str(t_type, "".to_string());
    }

    fn add_token_str(&mut self, t_type: TokenType, literal: String) {
	let text: String = self.source[self.start..self.current].to_string();
	self.tokens.push(Token::new(t_type, text, literal, 0.0, self.line));
    }

    fn add_token_num(&mut self, t_type: TokenType, literal: f32) {
	let text: String = self.source[self.start..self.current].to_string();
	self.tokens.push(Token::new(t_type, text, "".to_string(), literal, self.line));
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
	self.add_token_str(TokenType::StringLit, string);
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
	while let c = self.peek() && self.is_digit(c) {
	    self.advance();
	}

	let c = self.peek_next();
	if self.peek() == '.' && self.is_digit(c) {
	    self.advance();
	    while let c = self.peek() && self.is_digit(c) {
		self.advance();
	    }
	}

	let numstr = &self.source[self.start..self.current];
	self.add_token_num(TokenType::FloatLit, numstr.parse::<f32>().unwrap());
    }

    fn identifier(&mut self) {
	while let c = self.peek() && self.is_alphanumeric(c) {
	    self.advance();
	}

	let text = &self.source[self.start..self.current];
	match self.keywords.get(text) {
	    Some(t_type) => self.add_token(*t_type),
	    None => self.add_token(TokenType::Ident),
	};
    }

    fn peek_next(&mut self) -> char {
	if self.current + 1 >= self.source.chars().count() {
	    return '\0';
	}
	return self.source.chars().nth(self.current + 1).unwrap();
    }
}
