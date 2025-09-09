use crate::token::Token;

pub trait Expr {
    fn print(&self) -> String;
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
}

#[derive (Debug)]
pub enum Literal {
    BoolLit(bool),
    StrLit(String),
    RealLit(f32),
    NilLit,
}

/*pub struct Literal {
    bool_value: bool,
    str_value: String,
    real_value: f32,
    is_nil: bool,
}

impl Literal {
    pub fn new(bool_value: bool, str_value: String,
    real_value: f32, is_nil: bool) -> Self {
	Literal {
	    bool_value: bool_value,
	    str_value: str_value,
	    real_value: real_value,
	    is_nil: is_nil,
	}
    }
}*/

impl Expr for Literal {
    fn print(&self) -> String {
	match self {
	    Self::NilLit => format!("nil"),
	    _ => format!("{:?}", self),
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
}
