use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LParen, RParen, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    LetEqual, Equal, Less, LessEqual, Greater, GreaterEqual, Bang, BangEqual,
    Ident, StrLit, RealLit(f32), IntLit(u32),
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
    pub lexeme: String,
    pub line: u32,
    pub strval: String,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, line: u32, strval: String) -> Self {
	Token {
	    t_type: t_type,
	    lexeme: lexeme,
	    line: line,
	    /* this is a bad solution, should just be in the StrLit variant
	     * but i can't get the borrow checker happy when i do that.
	     * so it will be subpar for now
	     */
	    strval: strval,
	}
    }
    
    pub fn to_string(&self) -> String {
	match self.t_type {
	    TokenType::RealLit(r) => format!("{r}"),
	    TokenType::StrLit => format!("{}", self.strval),
	    _ => format!("{}", self.lexeme),
	}
    }
}
