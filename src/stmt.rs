use std::error::Error;
use std::rc::Rc;
use crate::expr::Expr;

pub enum StmtType {
    Print(Box<dyn Expr>),
    Expression(Box<dyn Expr>),
    IntDecl(Rc<String>, Option<Box<dyn Expr>>),
    RealDecl(Rc<String>, Option<Box<dyn Expr>>),
    StrDecl(Rc<String>, Option<Box<dyn Expr>>),
    Block(Vec<Stmt>),
}

pub trait StmtVisitor {
    fn visit_print(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_expression(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_intdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_realdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_strdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_block(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
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

    pub fn print(&self) -> String {
	use StmtType::*;
	match &self.s_type {
	    Print(e) => {
		format!("(print {})", e.print())
	    },
	    Expression(e) => {
		format!("{}", e.print())
	    },
	    IntDecl(n, e) => {
		format!("(int {n}{}", match e {
		    Some(ex) => format!(" {})", ex.print()),
		    None => format!(")"),
		})
	    },
	    RealDecl(n, e) => {
		format!("(real {n}{}", match e {
		    Some(ex) => format!(" {})", ex.print()),
		    None => format!(")"),
		})
	    },
	    StrDecl(n, e) => {
		format!("(str {n}{}", match e {
		    Some(ex) => format!(" {})", ex.print()),
		    None => format!(")"),
		})
	    },
	    Block(s) => {
		let mut output = String::new();
		output.push_str("(block\n");
		for stmt in s {
		    output.push_str("  ");
		    output.push_str(&stmt.print());
		    output.push('\n');
		}
		output.push(')');
		output
	    },
	}
    }

    pub fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<(), Box<dyn Error>> {
	use StmtType::*;
	match &self.s_type {
	    Print(..) => {
		visitor.visit_print(&self.s_type)
	    },
	    Expression(..) => {
		visitor.visit_expression(&self.s_type)
	    },
	    IntDecl(..) => {
		visitor.visit_intdecl(&self.s_type)
	    },
	    RealDecl(..) => {
		visitor.visit_realdecl(&self.s_type)
	    },
	    StrDecl(..) => {
		visitor.visit_strdecl(&self.s_type)
	    },
	    Block(..) => {
		visitor.visit_block(&self.s_type)
	    },
	}
    }
}
