use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LParen, RParen, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    LetEqual, Equal, Less, LessEqual, Greater, GreaterEqual, Bang, BangEqual,
    Ident, StrLit(Rc<String>), RealLit(f32), IntLit(u32),
    Begin, End, Procedure, Function, Return, If, For, Repeat, Until, Else, Do, To, Program, True, False, And, Or, Nil, Print, Real, Int, Str, Bool,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: Rc<String>,
    pub line: u32,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, line: u32) -> Self {
	Token {
	    t_type: t_type,
	    lexeme: Rc::new(lexeme),
	    line: line,
	}
    }
    
    pub fn to_string(&self) -> String {
	match &self.t_type {
	    TokenType::RealLit(r) => format!("{r}"),
	    TokenType::StrLit(s) => format!("{}", s),
	    _ => format!("{}", self.lexeme),
	}
    }
}
