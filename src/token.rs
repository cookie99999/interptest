use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LParen, RParen, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    LetEqual, Equal, Less, LessEqual, Greater, GreaterEqual, Bang, BangEqual,
    Ident, StringLit, FloatLit,
    Begin, End, Procedure, Function, Return, If, For, Repeat, Until, Else, Do, To, Program, True, False, And, Or, Nil, Print, Real, Str, Bool,
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
    pub str_literal: String,
    pub num_literal: f32,
    line: u32,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, str_literal: String,
	       num_literal: f32, line: u32) -> Self {
	Token {
	    t_type: t_type,
	    lexeme: lexeme,
	    str_literal: str_literal,
	    num_literal: num_literal,
	    line: line,
	}
    }
    
    pub fn to_string(&self) -> String {
	match self.t_type {
	    TokenType::FloatLit => format!("{}", self.num_literal),
	    TokenType::StringLit => format!("{}", self.str_literal),
	    _ => format!("{}", self.lexeme),
	}
    }
}
