use std::error::Error;
use std::rc::Rc;
use crate::expr::Expr;

pub enum StmtType {
    Print(Box<dyn Expr>),
    Expression(Box<dyn Expr>),
    IntDecl(Rc<String>, Option<Box<dyn Expr>>),
    RealDecl(Rc<String>, Option<Box<dyn Expr>>),
    StrDecl(Rc<String>, Option<Box<dyn Expr>>),
}

pub trait StmtVisitor {
    fn visit_print(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_expression(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_intdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_realdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
    fn visit_strdecl(&mut self, s: &StmtType) -> Result<(), Box<dyn Error>>;
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
	}
    }
}
