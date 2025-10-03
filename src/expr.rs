use std::error::Error;
use std::any::Any;
use std::rc::Rc;
use std::fmt;
use crate::token::Token;
use crate::token::TokenType;
use crate::environment::Environment;

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

//so far only need this in the assignment parsing,
//feels like it could be refactored out
pub enum ExprType {
    Binary,
    Unary,
    Grouping,
    Literal,
    Assignment,
    Variable,
}

pub trait Expr {
    fn print(&self) -> String;
    fn evaluate(&self, env: &mut Environment) -> Result<Value, Box<dyn Error>>;
    fn kind(&self) -> ExprType;
    //another hack only needed for assignment
    fn as_any(&self) -> &dyn Any;
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

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn kind(&self) -> ExprType {
	ExprType::Binary
    }

    fn evaluate(&self, env: &mut Environment) -> Result<Value, Box<dyn Error>> {
	let left = self.left.evaluate(env)?;
	let right = self.right.evaluate(env)?;
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
	    TokenType::EqualEqual => {
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

    fn kind(&self) -> ExprType {
	ExprType::Grouping
    }

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn evaluate(&self, env: &mut Environment) -> Result<Value, Box<dyn Error>> {
	self.expression.evaluate(env)
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

    fn kind(&self) -> ExprType {
	ExprType::Literal
    }

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn evaluate(&self, _env: &mut Environment) -> Result<Value, Box<dyn Error>> {
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

    fn kind(&self) -> ExprType {
	ExprType::Unary
    }

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn evaluate(&self, env: &mut Environment) -> Result<Value, Box<dyn Error>> {
	let right = self.right.evaluate(env)?;

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

pub struct Assignment {
    name: Rc<String>,
    val: Box<dyn Expr>,
}

impl Assignment {
    pub fn new(name: String, val: Box<dyn Expr>) -> Self {
	Assignment {
	    name: Rc::new(name),
	    val: val,
	}
    }
}

impl Expr for Assignment {
    fn print(&self) -> String {
	format!("(= {} {})", self.name, self.val.print())
    }

    fn kind(&self) -> ExprType {
	ExprType::Assignment
    }

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn evaluate(&self, env: &mut Environment) -> Result<Value, Box<dyn Error>> {
	let r_value = self.val.evaluate(env)?;
	let l_value;
	l_value = env.get(&self.name)?;
	match (&l_value, &r_value) {
	    (Value::IntVal(_), Value::IntVal(_)) |
	    (Value::RealVal(_), Value::RealVal(_)) |
	    (Value::StrVal(_), Value::StrVal(_)) |
	    (Value::NilVal, Value::NilVal) => {
		env.assign(&self.name, &r_value)?;
	    },
	    _ => {
		println!("type mismatch in {:?} and {:?}", l_value, r_value);
		return Err(Box::new(EvalError {}));
	    },
	};
	Ok(r_value)
    }
}

pub struct Variable {
    pub name: Rc<String>,
}

impl Variable {
    pub fn new(name: String) -> Self {
	Variable {
	    name: Rc::new(name),
	}
    }
}

impl Expr for Variable {
    fn print(&self) -> String {
	format!("{}", self.name)
    }

    fn kind(&self) -> ExprType {
	ExprType::Variable
    }

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn evaluate(&self, env: &mut Environment) -> Result<Value, Box<dyn Error>> {
	env.get(&self.name)
    }
}
