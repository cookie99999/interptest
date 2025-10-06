use std::error::Error;
use std::any::Any;
use std::rc::Rc;
use std::fmt;
use crate::token::Token;

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

//so far only need this in the assignment parsing,
//feels like it could be refactored out
pub enum ExprType {
    Binary,
    Unary,
    Grouping,
    Literal,
    Assignment,
    Variable,
    Logical,
}

pub trait Expr {
    fn print(&self) -> String;
    fn kind(&self) -> ExprType;
    //another hack only needed for assignment
    fn as_any(&self) -> &dyn Any;
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>>;
}

pub trait ExprVisitor {
    fn visit_binary(&mut self, e: &Binary) -> Result<Value, Box<dyn Error>>;
    fn visit_unary(&mut self, e: &Unary) -> Result<Value, Box<dyn Error>>;
    fn visit_grouping(&mut self, e: &Grouping) -> Result<Value, Box<dyn Error>>;
    fn visit_literal(&mut self, e: &Literal) -> Result<Value, Box<dyn Error>>;
    fn visit_assignment(&mut self, e: &Assignment) -> Result<Value, Box<dyn Error>>;
    fn visit_variable(&mut self, e: &Variable) -> Result<Value, Box<dyn Error>>;
    fn visit_logical(&mut self, e: &Logical) -> Result<Value, Box<dyn Error>>;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
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

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_binary(&self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
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

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_grouping(&self)
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

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_literal(&self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
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

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_unary(&self)
    }
}

pub struct Assignment {
    pub name: Rc<String>,
    pub val: Box<dyn Expr>,
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

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_assignment(&self)
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

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_variable(&self)
    }
}

pub struct Logical {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Logical {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
	Logical {
	    left: left,
	    operator: operator,
	    right: right,
	}
    }
}

impl Expr for Logical {
    fn print(&self) -> String {
	format!("({} {} {})", self.operator.lexeme, self.left.print(), self.right.print())
    }

    fn kind(&self) -> ExprType {
	ExprType::Logical
    }

    fn as_any(&self) -> &dyn Any {
	self
    }

    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value, Box<dyn Error>> {
	visitor.visit_logical(&self)
    }
}
