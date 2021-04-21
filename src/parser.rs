use crate::{Result, error, Error, lexer::Token};

pub enum Expr {
    Var(String),
}
impl Expr {
    pub fn get_type(&self) -> String {
        match self {
            Self::Var(_) => "Variable"
        }.to_string()
    }
    pub fn get_lit(&self) -> String {
        match self {
            Self::Var(name) => name.to_string()
        }
    }
}

pub struct Parser {
    input: Vec<Token>,
    output: Vec<Expr>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            output: vec![],
        }
    }
}
