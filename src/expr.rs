use std::error::Error;
use std::rc::Rc;
use std::fmt;
use crate::token::Token;
use crate::token::TokenType;

#[derive (Debug, PartialEq, Clone)]
pub enum Value {
    RealVal(f32),
    IntVal(u32),
    StrVal(Rc<String>),
    BoolVal(bool),
    NilVal,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Value::RealVal(r) => write!(f, "{r}"),
	    Value::IntVal(i) => write!(f, "{i}"),
	    Value::StrVal(s) => write!(f, "{s}"),
	    Value::BoolVal(b) => write!(f, "{b}"),
	    Value::NilVal => write!(f, "nil"),
	}
    }
}

#[derive (Debug)]
struct EvalError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "evaluation error")
    }
}

impl Error for EvalError {}

pub trait Expr {
    fn print(&self) -> String;
    fn evaluate(&self) -> Result<Value, Box<dyn Error>>;
}

pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>,
	       operator: Token,
	       right: Box<dyn Expr>) -> Self {
	Binary {
	    left: left,
	    operator: operator,
	    right: right,
	}
    }
}    

impl Expr for Binary {
    fn print(&self) -> String {
	format!("({} {} {})", self.operator.lexeme,
		self.left.print(), self.right.print())
    }

    fn evaluate(&self) -> Result<Value, Box<dyn Error>> {
	let left = self.left.evaluate()?;
	let right = self.right.evaluate()?;
	//todo: decide whether to saturate or wrap arithmetic
	match self.operator.t_type {
	    TokenType::Plus => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::RealVal(l + r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::IntVal(l + r))
		    },
		    //todo: string concat
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Minus => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::RealVal(l - r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::IntVal(l - r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Slash => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			//TODO: divide by zero handling
			Ok(Value::RealVal(l / r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::IntVal(l / r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Star => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::RealVal(l * r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::IntVal(l * r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Greater => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::BoolVal(l > r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::BoolVal(l > r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::GreaterEqual => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::BoolVal(l >= r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::BoolVal(l >= r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Less => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::BoolVal(l < r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::BoolVal(l < r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::LessEqual => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::BoolVal(l <= r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::BoolVal(l <= r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Equal => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::BoolVal(l == r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::BoolVal(l == r))
		    },
		    (Value::BoolVal(l), Value::BoolVal(r)) => {
			Ok(Value::BoolVal(l == r))
		    },
		    (Value::NilVal, Value::NilVal) => Ok(Value::BoolVal(true)),
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::BangEqual => {
		match (left, right) {
		    (Value::RealVal(l), Value::RealVal(r)) => {
			Ok(Value::BoolVal(l != r))
		    },
		    (Value::IntVal(l), Value::IntVal(r)) => {
			Ok(Value::BoolVal(l != r))
		    },
		    (Value::BoolVal(l), Value::BoolVal(r)) => {
			Ok(Value::BoolVal(l != r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::And => {
		match (left, right) {
		    (Value::BoolVal(l), Value::BoolVal(r)) => {
			Ok(Value::BoolVal(l && r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    TokenType::Or => {
		match (left, right) {
		    (Value::BoolVal(l), Value::BoolVal(r)) => {
			Ok(Value::BoolVal(l || r))
		    },
		    _ => {
			crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    _ => {
		//should be unreachable
		println!("binary operator '{}' supported in parser but not in evaluate", self.operator.lexeme);
		Err(Box::new(EvalError{}))
	    },
	}
    }
}

pub struct Grouping {
    expression: Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
	Grouping {
	    expression: expression,
	}
    }
}

impl Expr for Grouping {
    fn print(&self) -> String {
	format!("(group {})", self.expression.print())
    }

    fn evaluate(&self) -> Result<Value, Box<dyn Error>> {
	self.expression.evaluate()
    }
}

#[derive (Debug)]
pub enum Literal {
    BoolLit(bool),
    StrLit(Rc<String>),
    RealLit(f32),
    IntLit(u32),
    NilLit,
}

impl Expr for Literal {
    fn print(&self) -> String {
	match self {
	    Self::NilLit => format!("nil"),
	    Self::StrLit(s) => format!("{s}"),
	    Self::RealLit(r) => format!("{r}"),
	    Self::IntLit(i) => format!("{i}"),
	    Self::BoolLit(b) => format!("{b}"),
	}
    }

    fn evaluate(&self) -> Result<Value, Box<dyn Error>> {
	match self {
	    Literal::StrLit(s) => Ok(Value::StrVal(s.clone())),
	    Literal::RealLit(r) => Ok(Value::RealVal(*r)),
	    Literal::IntLit(i) => Ok(Value::IntVal(*i)),
	    Literal::BoolLit(b) => Ok(Value::BoolVal(*b)),
	    Literal::NilLit => Ok(Value::NilVal),
	    //no error possible unless the parsing is buggy
	}
    }
}

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}

impl Unary {
    pub fn new(operator: Token,
	       right: Box<dyn Expr>) -> Self {
	Unary {
	    operator: operator,
	    right: right,
	}
    }
}

impl Expr for Unary {
    fn print(&self) -> String {
	format!("({} {})", self.operator.lexeme,
		self.right.print())
    }

    fn evaluate(&self) -> Result<Value, Box<dyn Error>> {
	let right = self.right.evaluate()?;

	match self.operator.t_type {
	    TokenType::Minus => match right {
		Value::RealVal(r) => Ok(Value::RealVal(-r)),
		_ => {
		    crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "type incompatible with operator");
		    Err(Box::new(EvalError{}))
		},
	    },
	    TokenType::Bang => match right {
		Value::BoolVal(b) => Ok(Value::BoolVal(!b)),
		_ => {
		    crate::report(self.operator.line, &format!(" at '{}'", self.operator.lexeme),
				  "type incompatible with operator");
		    Err(Box::new(EvalError{}))
		},
	    },
	    _ => {
		//should be unreachable due to parsing logic
		println!("crazy unreachable error in Unary::evaluate");
		Err(Box::new(EvalError{}))
	    },
	}
    }
}
