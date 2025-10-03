use std::error::Error;
use std::rc::Rc;
use crate::expr::Expr;
use crate::expr::Value;
use crate::environment::Environment;

pub enum StmtType {
    Print(Box<dyn Expr>), Expression(Box<dyn Expr>),
    IntDecl(Rc<String>, Option<Box<dyn Expr>>),
    RealDecl(Rc<String>, Option<Box<dyn Expr>>),
    StrDecl(Rc<String>, Option<Box<dyn Expr>>),
}

pub struct Stmt {
    s_type: StmtType,
}

impl Stmt {
    pub fn new(s_type: StmtType) -> Self {
	Stmt {
	    s_type: s_type,
	}
    }

    pub fn execute(&self, env: &mut Environment) -> Result<(), Box<dyn Error>> {
	use StmtType::*;
	match &self.s_type {
	    Print(e) => {
		let val = e.evaluate(env)?;
		println!("{val}");
		Ok(())
	    },
	    Expression(e) => {
		e.evaluate(env)?;
		Ok(())
	    },
	    IntDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.evaluate(env)?;
			match v {
			    Value::IntVal(_) => env.define(n, v),
			    _ => {
				println!("mismatched types {} and {:?}", n, v);
				return Err(Box::new(crate::RuntimeError {}))
			    },
			}
		    },
		    None => env.define(n, Value::IntVal(0)),
		};
		Ok(())
	    },
	    RealDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.evaluate(env)?;
			match v {
			    Value::RealVal(_) => env.define(n, v),
			    _ => {
				println!("mismatched types {} and {:?}", n, v);
				return Err(Box::new(crate::RuntimeError {}))
			    },
			}
		    },
		    None => env.define(n, Value::RealVal(0.0)),
		};
		Ok(())
	    },
	    StrDecl(n, e) => {
		match e {
		    Some(ex) => {
			let v = ex.evaluate(env)?;
			match v {
			    Value::StrVal(_) => env.define(n, v),
			    _ => {
				println!("mismatched types {} and {:?}", n, v);
				return Err(Box::new(crate::RuntimeError {}))
			    },
			}
		    },
		    None => env.define(n, Value::StrVal(Rc::new(String::new()))),
		};
		Ok(())
	    },
	}
    }
}
