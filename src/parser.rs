use std::mem::discriminant;
use crate::{Result, error, Error, lexer::Token};

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Var(String),
    String(String),
    Number(i32),
    Float(f32),
    Let(String, Box<Expr>),
    Defun(String, Vec<String>, Vec<Expr>),
    Call(String, Vec<Expr>),
    Unit
}
impl Expr {
    pub fn get_type(&self) -> String {
        match self {
            Self::Var(_) => "Variable",
            Self::String(_) => "String",
            Self::Number(_) => "Number",
            Self::Float(_) => "Float",
            Self::Call(_, _) => "Function Call",
            Self::Let(_, _) => "Variable Definition",
            Self::Defun(_, _, _) => "Function Definition",
            Self::Unit => "Unit",
        }.to_string()
    }
}

pub struct Parser {
    input: Vec<Token>,
    output: Vec<Expr>,
    current: usize
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            output: vec![],
            current: 0,
        }
    }
    fn advance(&mut self, expected: Token) -> Result<Token> {
        let popped = self.pop()?;

        if discriminant(&popped) != discriminant(&expected) {
            error!(
                "Expected {}, found {}.",
                expected.get_type(),
                popped.get_type(),
                )
        } else {
            Ok(popped)
        }
    }
    fn peek(&self) -> Option<Token> {
        self.input
            .iter()
            .nth(self.current)
            .and_then(|t| Some(t.clone()))
    }
    fn is_at_end(&self) -> bool {
        self.input.len() != 1 && self.current >= self.input.len()
    }
    fn pop(&mut self) -> Result<Token> {
        if self.is_at_end() {
            error!("Unfinished expression.")

        } else {
            if self.input.len() != 1 {
                self.current += 1;
            }
            Ok(self.input[self.current - if self.input.len() == 1 { 0 } else { 1  }].clone())

        }       
    }
    pub fn parse(&mut self) -> Result<Vec<Expr>> {
        while !self.is_at_end() {
            let to_push = self.parse_expr()?;
            self.output.push(to_push);

            if self.input.len() == 1 {
                break;

            }

        }

        Ok(self.output.clone())

    }
    fn parse_expr(&mut self) -> Result<Expr> {
        let token = self.pop()?;

        Ok(match token {
            Token::String(s) => Expr::String(s),
            Token::Identifier(s) => Expr::String(s),
            Token::Number(n) => Expr::Number(n),
            Token::Float(f) => Expr::Float(f),
            Token::Sharp => {
                let var = self.advance(Token::Identifier("".to_string()))?;
                if let Token::Identifier(var) = var {
                    Expr::Var(var)
                } else {
                    panic!("Bug: UNEXPECTED_NON_IDENTIFIER");
                }
            }
            Token::RParen => return error!("Unexpected Closing Parenthese."),
            Token::LParen => {
                let sub = self.pop()?;

                match sub {
                    Token::Identifier(function) => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap() != Token::RParen {
                            args.push(self.parse_expr()?);

                        }

                        if !self.is_at_end() {
                            self.advance(Token::RParen)?;
                        }

                        Expr::Call(function, args)
                    }
                    Token::RParen => Expr::Unit,
                    Token::Let => {
                        let name = self.advance(Token::Identifier("".to_string()))?;
                        let value = self.parse_expr()?;

                        if !self.is_at_end() {
                            self.advance(Token::RParen)?;
                        }

                        let name = if let Token::Identifier(name) = name {
                            name
                        } else {
                            panic!("Bug: UNEXPECTED_NON_IDENTIFIER");
                        };

                        Expr::Let(name, Box::new(value))
                    }
                    Token::Defun => {
                        self.advance(Token::LParen)?;

                        let name = self.advance(Token::Identifier("".to_string()))?;

                        let name = if let Token::Identifier(name) = name {
                            name
                        } else {
                            panic!("Bug: UNEXPECTED_NON_IDENTIFIER");
                        };

                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap() != Token::RParen {
                            let arg = self.advance(Token::Identifier("".to_string()))?;

                            args.push(if let Token::Identifier(arg) = arg { arg } else {
                                panic!("Bug: UNEXPECTED_NON_IDENTIFIER");
                            });
                        }

                        self.advance(Token::RParen)?;

                        let mut body = vec![];

                        while !self.is_at_end() && self.peek().unwrap() != Token::RParen {
                            body.push(self.parse_expr()?);
                        }

                        if !self.is_at_end() {
                            self.advance(Token::RParen)?;
                        }

                        Expr::Defun(name, args, body)
                    }
                    x => return error!("Unexpected {}.", x.get_type()),
                }
            }
            _ => return error!("Unexpected Keyword."),
        }
        )   
    }
}

#[cfg(test)]
mod parsing {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn string() -> Result<()> {
        let tokens = Lexer::new("'foo' bar").tokenize()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(ast, vec![Expr::String("foo".to_string()), Expr::String("bar".to_string())]);

        Ok(())
    }

    #[test]
    fn variable() -> Result<()> {
        let tokens = Lexer::new("#foo").tokenize()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(ast, vec![Expr::Var("foo".to_string())]);

        Ok(())
    }

}
