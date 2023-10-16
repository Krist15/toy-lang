use std::fmt::Display;

// use strum_macros::Display;

use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Object>, line: u32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    // pub fn to_string(&self) -> String {
    //     format!(
    //         "{:?} {} {} ",
    //         self.token_type.clone(),
    //         self.lexeme.clone(),
    //         self.literal.clone()
    //     )
    // }
}

#[derive(Clone, Debug)]
pub enum Object {
    Nil,
    String(String),
    Number(f64),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "NULL"),
            Object::String(s) => write!(f, "{s}"),
            Object::Number(n) => write!(f, "{n}"),
        }
    }
}
