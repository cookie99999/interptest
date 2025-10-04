use std::error::Error;
use std::rc::Rc;
use crate::stmt::{Stmt, StmtType, StmtVisitor};
use crate::environment::Environment;
use crate::token::TokenType;
use crate::expr;
use crate::expr::{ExprVisitor, Value};

#[derive (Debug)]
struct EvalError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "evaluation error")
    }
}

impl Error for EvalError {}

pub struct Interpreter {
    env: Vec<Environment>,
    cur_env: usize,
}

impl Interpreter {
    pub fn new() -> Self {
	let mut i = Interpreter {
	    env: Vec::<Environment>::new(),
	    cur_env: 0,
	};
	i.env.push(Environment::new()); //global env
	i
    }

    pub fn interpret(&mut self, ast: Vec<Stmt>) -> Result<(), Box<dyn Error>> {
	for stmt in ast.iter() {
	    stmt.accept(self)?
	}
	Ok(())
    }
}

impl ExprVisitor for Interpreter {
    fn visit_binary(&mut self, e: &expr::Binary) -> Result<Value, Box<dyn Error>> {
	let left = e.left.accept(self)?;
	let right = e.right.accept(self)?;
	//todo: decide whether to saturate or wrap arithmetic
	match e.operator.t_type {
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
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
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(EvalError{}))
		    },
		}
	    },
	    _ => {
		//should be unreachable
		println!("binary operator '{}' supported in parser but not in evaluate", e.operator.lexeme);
		Err(Box::new(EvalError{}))
	    },
	}
    }

    fn visit_grouping(&mut self, e: &expr::Grouping) -> Result<Value, Box<dyn Error>> {
	e.expression.accept(self)
    }

    fn visit_literal(&mut self, e: &expr::Literal) -> Result<Value, Box<dyn Error>> {
	use expr::Literal;
	match e {
	    Literal::StrLit(s) => Ok(Value::StrVal(s.clone())),
	    Literal::RealLit(r) => Ok(Value::RealVal(*r)),
	    Literal::IntLit(i) => Ok(Value::IntVal(*i)),
	    Literal::BoolLit(b) => Ok(Value::BoolVal(*b)),
	    Literal::NilLit => Ok(Value::NilVal),
	    //no error possible unless the parsing is buggy
	}
    }

    fn visit_unary(&mut self, e: &expr::Unary) -> Result<Value, Box<dyn Error>> {
	let right = e.right.accept(self)?;

	match e.operator.t_type {
	    TokenType::Minus => match right {
		Value::RealVal(r) => Ok(Value::RealVal(-r)),
		_ => {
		    crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				  "type incompatible with operator");
		    Err(Box::new(EvalError{}))
		},
	    },
	    TokenType::Bang => match right {
		Value::BoolVal(b) => Ok(Value::BoolVal(!b)),
		_ => {
		    crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				  "type incompatible with operator");
		    Err(Box::new(EvalError{}))
		},
	    },
	    _ => {
		//should be unreachable due to parsing logic
		println!("crazy unreachable error in visit_unary");
		Err(Box::new(EvalError{}))
	    },
	}
    }

    fn visit_assignment(&mut self, e: &expr::Assignment) -> Result<Value, Box<dyn Error>> {
	let r_value = e.val.accept(self)?;
	let l_value;
	l_value = self.env[self.cur_env].get(&e.name)?;
	match (&l_value, &r_value) {
	    (Value::IntVal(_), Value::IntVal(_)) |
	    (Value::RealVal(_), Value::RealVal(_)) |
	    (Value::StrVal(_), Value::StrVal(_)) |
	    (Value::NilVal, Value::NilVal) => {
		self.env[self.cur_env].assign(&e.name, &r_value)?;
	    },
	    _ => {
		println!("type mismatch in {:?} and {:?}", l_value, r_value);
		return Err(Box::new(EvalError {}));
	    },
	};
	Ok(r_value)
    }

    fn visit_variable(&mut self, e: &expr::Variable) -> Result<Value, Box<dyn Error>> {
	self.env[self.cur_env].get(&e.name)
    }
}

impl StmtVisitor for Interpreter {
    //the match in Stmt::accept() makes the _ arms here
    //redundant. might be better to have just one visit
    //and do the matching in that, or else
    //different stmt structs like the exprs
    fn visit_print(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::Print(e) => {
		let val = e.accept(self)?;
		println!("{val}");
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		Err(Box::new(crate::RuntimeError {}))
	    },
	}
    }

    fn visit_expression(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::Expression(e) => {
		e.accept(self)?;
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		Err(Box::new(crate::RuntimeError {}))
	    },
	}
    }

    fn visit_intdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::IntDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.accept(self)?;
			match v {
			    Value::IntVal(_) => self.env[self.cur_env].define(&n, v),
			    _ => {
				println!("mismatched types {} and {:?}", n, v);
				return Err(Box::new(crate::RuntimeError {}))
			    },
			}
		    },
		    None => self.env[self.cur_env].define(&n, Value::IntVal(0)),
		};
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		Err(Box::new(crate::RuntimeError {}))
	    },
	}
    }

    fn visit_realdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::RealDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.accept(self)?;
			match v {
			    Value::RealVal(_) => self.env[self.cur_env].define(&n, v),
			    _ => {
				println!("mismatched types {} and {:?}", n, v);
				return Err(Box::new(crate::RuntimeError {}))
			    },
			}
		    },
		    None => self.env[self.cur_env].define(&n, Value::RealVal(0.0)),
		};
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		Err(Box::new(crate::RuntimeError {}))
	    },
	}
    }

    fn visit_strdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::StrDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.accept(self)?;
			match v {
			    Value::StrVal(_) => self.env[self.cur_env].define(&n, v),
			    _ => {
				println!("mismatched types {} and {:?}", n, v);
				return Err(Box::new(crate::RuntimeError {}))
			    },
			}
		    },
		    None => self.env[self.cur_env].define(&n,
							  expr::Value::StrVal(Rc::new(String::new()))),
		};
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		Err(Box::new(crate::RuntimeError {}))
	    },
	}
    }
}
