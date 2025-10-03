use std::error::Error;
use crate::token::Token;
use crate::token::TokenType;
use crate::expr::*;
use crate::stmt::Stmt;
use crate::stmt::StmtType;

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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Box<dyn Error>> {
	let mut stmts: Vec<Stmt> = Vec::new();
	while !self.is_at_end() {
	    stmts.push(self.declaration()?);
	}
	Ok(stmts)
    }

    fn is_at_end(&mut self) -> bool {
	type_match!(self.peek().t_type, TokenType::EOF)
    }

    fn peek(&mut self) -> &Token {
	&self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
	&self.tokens[self.current.saturating_sub(1)]
    }

    fn advance(&mut self) -> &Token {
	if !self.is_at_end() {
	    self.current += 1;
	}
	self.previous()
    }

    fn declaration(&mut self) -> Result<Stmt, Box<dyn Error>> {
	match self.peek().t_type {
	    TokenType::Int => {
		self.advance();
		self.int_decl()
	    },
	    TokenType::Real => {
		self.advance();
		self.real_decl()
	    },
	    TokenType::Str => {
		self.advance();
		self.str_decl()
	    },
	    _ => self.statement(),
	}
    }

    fn int_decl(&mut self) -> Result<Stmt, Box<dyn Error>> {
	let name = match self.consume(|t_type| type_match!(t_type, TokenType::Ident)) {
	    Ok(t) => t.lexeme.clone(),
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect variable name");
		return Err(e)
	    },
	};

	let initializer: Option<Box<dyn Expr>> = match self.peek().t_type {
	    TokenType::Equal => {
		self.advance();
		Some(self.expression()?)
	    },
	    _ => None,
	};

	match self.consume(|t_type| type_match!(t_type, TokenType::Semicolon)) {
	    Ok(_) => {},
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect ';' after declaration");
		return Err(e)
	    },
	};

	Ok(Stmt::new(StmtType::IntDecl(name, initializer)))
    }

    fn real_decl(&mut self) -> Result<Stmt, Box<dyn Error>> {
	let name = match self.consume(|t_type| type_match!(t_type, TokenType::Ident)) {
	    Ok(t) => t.lexeme.clone(),
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect variable name");
		return Err(e)
	    },
	};

	let initializer: Option<Box<dyn Expr>> = match self.peek().t_type {
	    TokenType::Equal => {
		self.advance();
		Some(self.expression()?)
	    },
	    _ => None,
	};

	match self.consume(|t_type| type_match!(t_type, TokenType::Semicolon)) {
	    Ok(_) => {},
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect ';' after declaration");
		return Err(e)
	    },
	};

	Ok(Stmt::new(StmtType::RealDecl(name, initializer)))
    }

    fn str_decl(&mut self) -> Result<Stmt, Box<dyn Error>> {
	let name = match self.consume(|t_type| type_match!(t_type, TokenType::Ident)) {
	    Ok(t) => t.lexeme.clone(),
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect variable name");
		return Err(e)
	    },
	};

	let initializer: Option<Box<dyn Expr>> = match self.peek().t_type {
	    TokenType::Equal => {
		self.advance();
		Some(self.expression()?)
	    },
	    _ => None,
	};

	match self.consume(|t_type| type_match!(t_type, TokenType::Semicolon)) {
	    Ok(_) => {},
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect ';' after declaration");
		return Err(e)
	    },
	};

	Ok(Stmt::new(StmtType::StrDecl(name, initializer)))
    }

    fn statement(&mut self) -> Result<Stmt, Box<dyn Error>> {
	match self.peek().t_type {
	    TokenType::Print => {
		self.advance();
		self.print_stmt()
	    },
	    _ => self.expr_stmt(),
	}
    }

    fn print_stmt(&mut self) -> Result<Stmt, Box<dyn Error>> {
	let value = self.expression()?;
	match self.consume(|t_type| type_match!(t_type, TokenType::Semicolon)) {
	    Ok(_) => {},
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect ';' after print value");
		return Err(e)
	    },
	};
	Ok(Stmt::new(StmtType::Print(value)))
    }

    fn expr_stmt(&mut self) -> Result<Stmt, Box<dyn Error>> {
	let expr = self.expression()?;
		match self.consume(|t_type| type_match!(t_type, TokenType::Semicolon)) {
	    Ok(_) => {},
	    Err(e) => {
		crate::report(self.peek().line, &format!(" at '{}'", self.peek().lexeme),
			      "expect ';' after expression");
		return Err(e)
	    },
		};
	Ok(Stmt::new(StmtType::Expression(expr)))
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	let mut expr = self.equality()?;

	match self.peek().t_type {
	    TokenType::Equal => {
		self.advance();
		let eq_line = self.previous().line;
		let eq_lexeme = format!("{}", self.previous().lexeme);
		let value = self.assignment()?;

		match expr.kind() {
		    ExprType::Variable => {
			let name: String = format!("{}", expr.as_any().downcast_ref::<Variable>()
			    .expect("downcast failed, fix parser::assignment")
			    .name);
			expr = Box::new(Assignment::new(name, value));
		    },
		    _ => {
			crate::report(eq_line, &format!(" at '{}'", eq_lexeme),
				      "invalid l-value");
			return Err(Box::new(ParseError{}));
		    },
		}
	    },
	    _ => {},
	}
	Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>, Box<dyn Error>> {
	let mut expr = self.comparison();

	while !self.is_at_end() && match self.peek().t_type {
	    TokenType::BangEqual | TokenType::EqualEqual => {
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
	    //the clone is only needed for the string lit
	    //which feels like something that could be improved
	    match self.peek().t_type.clone() {
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
		TokenType::StrLit(s) => {
		    self.advance();
		    Ok(Box::new(Literal::StrLit(s)))
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
		},
		TokenType::Ident => {
		    self.advance();
		    Ok(Box::new(Variable::new(format!("{}", self.previous().lexeme))))
		},
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
		TokenType::Function | TokenType::While |
		TokenType::For | TokenType::If |
		TokenType::Return | TokenType::Print => return,
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
