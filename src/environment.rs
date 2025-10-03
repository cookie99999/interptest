use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use crate::expr;

pub struct Environment {
    values: HashMap<Rc<String>, expr::Value>,
}

impl Environment {
    pub fn new() -> Self {
	Environment {
	    values: HashMap::new(),
	}
    }

    pub fn define(&mut self, name: &Rc<String>, value: expr::Value) {
	self.values.insert(name.clone(), value);
    }

    pub fn get(&self, name: &Rc<String>) -> Result<expr::Value, Box<dyn Error>> {
	match self.values.get(name) {
	    Some(v) => Ok(v.clone()),
	    None => {
		println!("undefined variable {}", name.clone());
		Err(Box::new(crate::RuntimeError {}))
	    },
	}
    }

    pub fn assign(&mut self, name: &Rc<String>, value: &expr::Value) -> Result<(), Box<dyn Error>> {
	if self.values.contains_key(name) {
	    self.values.insert(name.clone(), value.clone());
	    Ok(())
	} else {
	    println!("undefined variable {}", name);
	    Err(Box::new(crate::RuntimeError {}))
	}
    }
}
