use std::error::Error;
use crate::token::Token;
use crate::token::TokenType;
use crate::expr::*;

macro_rules! check {
    ($var:path, $end:expr, $peek:expr) => {
	if $end {
	    false
	} else {
	    match $peek {
		$var{..} => true,
		_ => false,
	    }
	}
    }
}

macro_rules! type_match {
    ($val:expr, $var:path) => {
	match $val {
	    $var{..} => true,
	    _ => false,
	}
    }
}

macro_rules! match_token {
    ($advance:stmt, $end:expr, $peek:expr, $token:path) => {
	if check!($token, $end, $peek) {
	    $advance
	    true
	} else {
	    false
	}
    };
    
    ($advance:stmt, $end:expr, $peek:expr, $token:path, $($tokens:path),+) => {{
	let mut b: bool = false;
	if match_token!($advance, $end, $peek, $token) {
	    b = true;
	}
	if match_token!($advance, $end, $peek, $($tokens),+) {
	    b = true;
	}
	b
    }};
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
	Parser {
	    tokens: Vec::<Token>::new(),
	    current: 0,
	}
    }

    fn is_at_end(&mut self) -> bool {
	type_match!(self.peek().t_type, TokenType::EOF)
    }

    fn peek(&mut self) -> &Token {
	&self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
	&self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
	if !self.is_at_end() {
	    self.current += 1;
	}
	self.previous()
    }

    fn expression(&mut self) -> Box<dyn Expr> {
	self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expr> {
	let mut expr = self.comparison();
	while match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::BangEqual, TokenType::Equal) {
	    let operator = self.previous().clone();
	    let right = self.comparison();
	    expr = Box::new(Binary::new(expr, operator, right));
	}
	expr
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
	let mut expr = self.term();
	while match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual) {
	    let operator = self.previous().clone();
	    let right = self.term();
	    expr = Box::new(Binary::new(expr, operator, right));
	}
	expr
    }

    fn term(&mut self) -> Box<dyn Expr> {
	let mut expr = self.factor();
	while match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::Minus, TokenType::Plus) {
	    let operator = self.previous().clone();
	    let right = self.factor();
	    expr = Box::new(Binary::new(expr, operator, right));
	}
	expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
	let mut expr = self.unary();
	while match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::Slash, TokenType::Star) {
	    let operator = self.previous().clone();
	    let right = self.unary();
	    expr = Box::new(Binary::new(expr, operator, right));
	}
	expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
	if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::Bang, TokenType::Minus) {
	    let operator = self.previous().clone();
	    let right = self.unary();
	    return Box::new(Unary::new(operator, right));
	}
	self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expr> {
	if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::False) {
	    return Box::new(Literal::BoolLit(false));
	}
	else if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::True) {
	    return Box::new(Literal::BoolLit(true));
	}
	else if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::Nil) {
	    return Box::new(Literal::NilLit);
	}

	else if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::FloatLit) {
	    return Box::new(Literal::RealLit(self.previous().num_literal));
	}
	else if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::StringLit) {
	    return Box::new(Literal::StrLit(self.previous().str_literal));
	}

	else if match_token!(_ = self.advance(), self.is_at_end(), self.peek().t_type, TokenType::LParen) {
	    let expr = self.expression();
	    self.consume(TokenType::RParen, "Missing ')' after expression");
	    return Box::new(Grouping::new(expr));
	}

	else {
	    println!("getting to unary() without a valid token should be impossible hopefully");
	    return Box::new(Literal::StrLit("oops".to_string()));
	}
    }

    fn consume(&mut self, t_type: TokenType, message: &str) -> Result<Token, Box<dyn Error>> {
	if check!(t_type, self.is_at_end(), self.peek()) {
	    Ok(self.advance())
	} else {
	    println!("{}", message);
	    Err(Box::new(ParseError{}))
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
