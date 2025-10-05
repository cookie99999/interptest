use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use crate::expr;

pub struct Environment {
    values: HashMap<Rc<String>, expr::Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
	Environment {
	    values: HashMap::new(),
	    parent: parent,
	}
    }

    pub fn define(&mut self, name: &Rc<String>, value: expr::Value) {
	self.values.insert(name.clone(), value);
    }

    pub fn get(&self, name: &Rc<String>) -> Result<expr::Value, Box<dyn Error>> {
	match self.values.get(name) {
	    Some(v) => Ok(v.clone()),
	    None => {
		match &self.parent {
		    Some(p) => p.borrow().get(name),
		    None => {
			println!("undefined variable {}", name.clone());
			Err(Box::new(crate::RuntimeError {}))
		    }
		}
	    },
	}
    }

    pub fn assign(&mut self, name: &Rc<String>, value: &expr::Value) -> Result<(), Box<dyn Error>> {
	if self.values.contains_key(name) {
	    self.values.insert(name.clone(), value.clone());
	    Ok(())
	} else {
	    match &self.parent {
		Some(p) => p.borrow_mut().assign(name, value),
		None => {
		    println!("undefined variable {}", name);
		    Err(Box::new(crate::RuntimeError {}))
		},
	    }
	}
    }
}
