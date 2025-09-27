use std::error::Error;
use crate::token::Token;
use crate::token::TokenType;
use crate::expr::*;

macro_rules! type_match {
    ($val:expr, $var:path) => {
	match $val {
	    $var{..} => true,
	    _ => false,
	}
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
	Parser {
	    tokens: tokens,
	    current: 0,
	}
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	self.expression()
    }

    fn is_at_end(&mut self) -> bool {
	type_match!(self.peek().t_type, TokenType::EOF)
    }

    fn peek(&mut self) -> &Token {
	&self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
	&self.tokens[self.current.saturating_sub(1)]
    }

    fn advance(&mut self) -> &Token {
	if !self.is_at_end() {
	    self.current += 1;
	}
	self.previous()
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	self.equality()
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	let mut expr = self.comparison();

	while !self.is_at_end() && match self.peek().t_type {
	    TokenType::BangEqual | TokenType::Equal => {
		self.advance();
		true
	    },
	    _ => false
	} {
	    let operator = self.previous().clone();
	    let right = self.comparison();
	    match right {
		Ok(r) => expr = Ok(Box::new(Binary::new(expr?, operator, r))),
		Err(e) => expr = Err(e),
	    }
	}
	expr
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	let mut expr = self.term();
	while !self.is_at_end() && match self.peek().t_type {
	    TokenType::Greater | TokenType::GreaterEqual |
	    TokenType::Less | TokenType::LessEqual => {
		self.advance();
		true
	    },
	    _ => false
	} {
	    let operator = self.previous().clone();
	    let right = self.term();
	    match right {
		Ok(r) => expr = Ok(Box::new(Binary::new(expr?, operator, r))),
		Err(e) => expr = Err(e),
	    }
	}
	expr
    }

    fn term(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	let mut expr = self.factor();
	while !self.is_at_end() && match self.peek().t_type {
	    TokenType::Minus | TokenType::Plus => {
		self.advance();
		true
	    },
	    _ => false
	} {
	    let operator = self.previous().clone();
	    let right = self.factor();
	    match right {
		Ok(r) => expr = Ok(Box::new(Binary::new(expr?, operator, r))),
		Err(e) => expr = Err(e),
	    }
	}
	expr
    }

    fn factor(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	let mut expr = self.unary();
	while !self.is_at_end() && match self.peek().t_type {
	    TokenType::Slash | TokenType::Star => {
		self.advance();
		true
	    },
	    _ => false
	} {
	    let operator = self.previous().clone();
	    let right = self.unary();
	    match right {
		Ok(r) => expr = Ok(Box::new(Binary::new(expr?, operator, r))),
		Err(e) => expr = Err(e),
	    }
	}
	expr
    }

    fn unary(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	if !self.is_at_end() && match self.peek().t_type {
	    TokenType::Bang | TokenType::Minus => {
		self.advance();
		true
	    },
	    _ => false
	} {
	    let operator = self.previous().clone();
	    let right = self.unary();
	    match right {
		Ok(r) => return Ok(Box::new(Unary::new(operator, r))),
		Err(e) => return Err(e),
	    }
	}
	let p = self.primary();
	match p {
	    Ok(exp) => Ok(exp),
	    Err(e) => Err(e),
	}
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	if !self.is_at_end() {
	    let strval = self.peek().strval.clone();
	    match self.peek().t_type {
		TokenType::False => {
		    self.advance();
		    Ok(Box::new(Literal::BoolLit(false)))
		},
		TokenType::True => {
		    self.advance();
		    Ok(Box::new(Literal::BoolLit(true)))
		},
		TokenType::Nil => {
		    self.advance();
		    Ok(Box::new(Literal::NilLit))
		},
		TokenType::RealLit(r) => {
		    self.advance();
		    Ok(Box::new(Literal::RealLit(r)))
		},
		TokenType::IntLit(i) => {
		    self.advance();
		    Ok(Box::new(Literal::IntLit(i)))
		},
		TokenType::StrLit => {
		    self.advance();
		    Ok(Box::new(Literal::StrLit(strval)))
		},
		TokenType::LParen => {
		    self.advance();
		    let expr = self.expression();
		    //don't care about return here
		    match self.consume(|t_type| type_match!(t_type, TokenType::RParen)) {
			Ok(_) => Ok(Box::new(Grouping::new(expr?))),
			Err(e) => {
			    crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
					  "missing ')' after expression");
			    Err(e)
			},
		    }
		}
		_ => {
		    crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
				  "expression expected");
		    Err(Box::new(ParseError{}))
		}
	    }
	} else {
	    crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
				  "unfinished expression");
	    Err(Box::new(ParseError{}))
	}
    }

    fn consume(&mut self, f: impl Fn(&'_ TokenType) -> bool) -> Result<&Token, Box<dyn Error>> {
	if !self.is_at_end() {
	    if f(&self.peek().t_type) {
		Ok(self.advance())
	    } else {
		Err(Box::new(ParseError{}))
	    }
	} else {
	    Err(Box::new(ParseError{}))
	}
    }

    fn synchronize(&mut self) {
	self.advance();
	while !self.is_at_end() {
	    if self.previous().t_type == TokenType::Semicolon {
		return;
	    }

	    match self.peek().t_type {
		TokenType::Function | TokenType::Procedure |
		TokenType::For | TokenType::If |
		TokenType::Return | TokenType::Repeat |
		TokenType::Do | TokenType::Print => return,
		_ => {},
	    };

	    self.advance();
	}
    }
}

#[derive (Debug)]
struct ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "parsing error")
    }
}

impl Error for ParseError {}
