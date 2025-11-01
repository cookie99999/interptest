use std::error::Error;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::stmt::{Stmt, StmtType, StmtVisitor};
use crate::expr;
use crate::expr::{ExprVisitor, Value};
use crate::token::{Token, TokenType};

#[derive (Copy, Clone, Debug)]
enum Type {
    Int,
    Real,
    Str,
    Bool,
    Nil,
}

#[derive (Debug)]
struct TypeError {}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "type error")
    }
}

impl Error for TypeError {}

struct TypeEnv {
    variables: HashMap<Rc<String>, Type>,
    parent: Option<Rc<RefCell<TypeEnv>>>,
}

impl TypeEnv {
    fn new(parent: Option<Rc<RefCell<TypeEnv>>>) -> Self {
	TypeEnv {
	    variables: HashMap::new(),
	    parent: parent,
	}
    }

    fn define(&mut self, name: &Token, value: Type) {
	self.variables.insert(name.lexeme.clone(), value);
    }

    fn get(&self, name: &Rc<String>) -> Result<Type, Box<dyn Error>> {
	match self.variables.get(name) {
	    Some(v) => Ok(*v),
	    None => {
		match &self.parent {
		    Some(p) => p.borrow().get(name),
		    None => {
			println!("undefined variable {}", name.clone());
			Err(Box::new(TypeError {}))
		    }
		}
	    },
	}
    }
}

pub struct TypeChecker {
    cur_env: Rc<RefCell<TypeEnv>>,
}

impl TypeChecker {
    pub fn new() -> Self {
	TypeChecker {
	    cur_env: Rc::new(RefCell::new(TypeEnv::new(None))),
	}
    }
    
    pub fn check(&mut self, ast: &Vec<Stmt>) -> Result<(), Box<dyn Error>> {
	for s in ast.iter() {
	    s.accept(self)?;
	}
	Ok(())
    }

    fn type_to_val(&self, t: &Type) -> Value {
	match t {
	    Type::Str => Value::StrVal(Rc::new(String::new())),
	    Type::Real => Value::RealVal(0.0),
	    Type::Int => Value::IntVal(0),
	    Type::Bool => Value::BoolVal(true),
	    Type::Nil => Value::NilVal,
	}
    }

    fn exec_block(&mut self, s: &Vec<Stmt>, e: Rc<RefCell<TypeEnv>>) -> Result<(), Box<dyn Error>> {
	let previous = self.cur_env.clone();
	self.cur_env = e;
	for stmt in s.iter() {
	    match stmt.accept(self) {
		Ok(_) => {},
		Err(e) => {
		    self.cur_env = previous;
		    return Err(e);
		},
	    }
	}
	self.cur_env = previous;
	Ok(())
    }
}

impl ExprVisitor for TypeChecker {
    fn visit_grouping(&mut self, e: &expr::Grouping) -> Result<Value, Box<dyn Error>> {
	e.expression.accept(self)
    }
    
    fn visit_binary(&mut self, e: &expr::Binary) -> Result<Value, Box<dyn Error>> {
	let left = e.left.accept(self)?;
	let right = e.right.accept(self)?;
	//todo: decide whether to saturate or wrap arithmetic
	match e.operator.t_type {
	    TokenType::EqualEqual => {
		match (left, right) {
		    (Value::RealVal(_), Value::RealVal(_)) |
		    (Value::IntVal(_), Value::IntVal(_)) |
		    (Value::BoolVal(_), Value::BoolVal(_)) |
		    (Value::NilVal, Value::NilVal) => Ok(Value::BoolVal(true)),
		    _ => {
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(TypeError{}))
		    },
		}
	    },
	    TokenType::BangEqual => {
		match (left, right) {
		    (Value::RealVal(_), Value::RealVal(_))|
		    (Value::IntVal(_), Value::IntVal(_)) |
		    (Value::BoolVal(_), Value::BoolVal(_)) => Ok(Value::BoolVal(true)),
		    _ => {
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				      "binary expression type mismatch");
			Err(Box::new(TypeError{}))
		    },
		}
	    },
	    _ => {
		match (left, right) {
		    (Value::IntVal(_), Value::IntVal(_)) => Ok(Value::IntVal(0)),
		    (Value::RealVal(_), Value::RealVal(_)) => Ok(Value::RealVal(0.0)),
		    _ => {
			crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				      "operand/operator type mismatch");
			Err(Box::new(TypeError{}))
		    },
		}
	    },
	}
    }

    fn visit_unary(&mut self, e: &expr::Unary) -> Result<Value, Box<dyn Error>> {
	let right = e.right.accept(self)?;

	match e.operator.t_type {
	    TokenType::Minus => match right {
		Value::RealVal(_) => Ok(Value::RealVal(0.0)),
		_ => {
		    crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				  "type incompatible with operator");
		    Err(Box::new(TypeError{}))
		},
	    },
	    TokenType::Bang => match right {
		Value::BoolVal(_) => Ok(Value::BoolVal(true)),
		_ => {
		    crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme),
				  "type incompatible with operator");
		    Err(Box::new(TypeError{}))
		},
	    },
	    _ => {
		//should be unreachable due to parsing logic
		println!("crazy unreachable error in TypeChecker::visit_unary");
		Err(Box::new(TypeError{}))
	    },
	}
    }

    fn visit_literal(&mut self, e: &expr::Literal) -> Result<Value, Box<dyn Error>> {
	use expr::Literal;
	match e {
	    Literal::StrLit(_) => Ok(Value::StrVal(Rc::new(String::new()))),
	    Literal::RealLit(_) => Ok(Value::RealVal(0.0)),
	    Literal::IntLit(_) => Ok(Value::IntVal(0)),
	    Literal::BoolLit(_) => Ok(Value::BoolVal(true)),
	    Literal::NilLit => Ok(Value::NilVal),
	    //no error possible unless the parsing is buggy
	}
    }

    fn visit_assignment(&mut self, e: &expr::Assignment) -> Result<Value, Box<dyn Error>> {
	let r_value = e.val.accept(self)?;
	let l_value = self.cur_env.borrow().get(&e.name.lexeme)?;
	let l_value = self.type_to_val(&l_value);
	
	match (l_value, &r_value) {
	    (Value::IntVal(_), Value::IntVal(_)) |
	    (Value::RealVal(_), Value::RealVal(_)) |
	    (Value::StrVal(_), Value::StrVal(_)) |
	    (Value::NilVal, Value::NilVal) => {
		Ok(r_value)
	    },
	    _ => {
		crate::report(e.name.line, &format!(" at '{}'", e.name.lexeme),
			      "type mismatch in assignment");
		Err(Box::new(TypeError {}))
	    },
	}
    }

    fn visit_variable(&mut self, e: &expr::Variable) -> Result<Value, Box<dyn Error>> {
	match (*self.cur_env).borrow().get(&e.name.lexeme) {
	    Ok(v) => Ok(self.type_to_val(&v)),
	    Err(_) => {
		crate::report(e.name.line, &format!(" at '{}'", e.name.lexeme.clone()),
			      "undeclared variable");
		Err(Box::new(TypeError {}))
	    }
	}
    }

    fn visit_logical(&mut self, e: &expr::Logical) -> Result<Value, Box<dyn Error>> {
	match (e.left.accept(self)?, e.right.accept(self)?) {
	    (Value::BoolVal(_), Value::BoolVal(_)) => Ok(Value::BoolVal(true)),
	    _ => {
		crate::report(e.operator.line, &format!(" at '{}'", e.operator.lexeme.clone()),
			      "type mismatch in logical expr (only booleans allowed)");
		Err(Box::new(TypeError {}))
	    },
	}
    }
}

impl StmtVisitor for TypeChecker {
    fn visit_intdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::IntDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.accept(self)?;
			match v {
			    Value::IntVal(_) => (*self.cur_env).borrow_mut().define(n, Type::Int),
			    _ => {
				crate::report(n.line, &format!(" at '{}'", n.lexeme.clone()),
					      "type mismatch in declaration");
				return Err(Box::new(TypeError {}))
			    },
			}
		    },
		    None => (*self.cur_env).borrow_mut().define(n, Type::Int),
		};
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in TypeChecker::StmtVisitor");
		Err(Box::new(TypeError {}))
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
			    Value::RealVal(_) => (*self.cur_env).borrow_mut().define(n, Type::Real),
			    _ => {
				crate::report(n.line, &format!(" at '{}'", n.lexeme.clone()),
					      "type mismatch in declaration");
				return Err(Box::new(TypeError {}))
			    },
			}
		    },
		    None => (*self.cur_env).borrow_mut().define(n, Type::Real),
		};
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in TypeChecker::StmtVisitor");
		Err(Box::new(TypeError {}))
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
			    Value::StrVal(_) => (*self.cur_env).borrow_mut().define(n, Type::Str),
			    _ => {
				crate::report(n.line, &format!(" at '{}'", n.lexeme.clone()),
					      "type mismatch in declaration");
				return Err(Box::new(TypeError {}))
			    },
			}
		    },
		    None => (*self.cur_env).borrow_mut().define(n, Type::Str),
		};
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in TypeChecker::StmtVisitor");
		Err(Box::new(TypeError {}))
	    },
	}
    }

    fn visit_print(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::Print(e) => e.accept(self)?,
	    _ => {
		println!("theoretically impossible error in TypeChecker::StmtVisitor");
		return Err(Box::new(TypeError {}));
	    },
	};
	Ok(())
    }

    fn visit_expression(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::Expression(e) => {
		e.accept(self)?;
		Ok(())
	    },
	    _ => {
		println!("theoretically impossible error in TypeChecker::StmtVisitor");
		Err(Box::new(TypeError {}))
	    },
	}
    }

    fn visit_block(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::Block(s) => {
		self.exec_block(s, Rc::new(RefCell::new(TypeEnv::new(Some(self.cur_env.clone())))))
	    },
	    _ => {
		println!("theoretically impossible error in TypeChecker::StmtVisitor");
		Err(Box::new(TypeError {}))
	    },
	}
    }

    fn visit_if(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::If(c, t, e) => {
		match c.accept(self)? {
		    Value::BoolVal(_) => {
			t.accept(self)?;
			match e {
			    Some(el) => el.accept(self)?,
			    None => {},
			};
			Ok(())
		    },
		    _ => {
			println!("conditional expression must be boolean");
			Err(Box::new(TypeError {}))
		    },
		}
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		Err(Box::new(TypeError {}))
	    },
	}
    }

    fn visit_while(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>> {
	match s {
	    StmtType::While(c, s) => {
		match c.accept(self)? {
		    Value::BoolVal(_) => {},
		    _ => {
			println!("conditional expression must be boolean");
			return Err(Box::new(crate::RuntimeError {}));
		    },
		};
		s.accept(self)?;
	    },
	    _ => {
		println!("theoretically impossible error in StmtVisitor");
		return Err(Box::new(crate::RuntimeError {}));
	    },
	}
	Ok(())
    }
}
