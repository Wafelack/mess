use crate::{Result, Error, error};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(String),
    Identifier(String),
    Number(i32),
    Float(f32),
    LParen,
    RParen,
}

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    current: usize,
    start: usize,
}

impl Lexer {
    pub fn new(input: impl ToString) -> Self {
        Self {
            input: input.to_string(),
            output: vec![],
            current: 0,
            start: 0
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn peek(&self) -> char {
        self.input.chars().nth(self.current).unwrap()
    }
    fn add_token(&mut self, token: Token) {
        self.output.push(token)
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.tokenize_one()?;
            self.start = self.current;
        }

        Ok(self.output.clone())
    }
    fn tokenize_one(&mut self) -> Result<()> {
        let c = self.advance();

        match c {
            '(' => self.add_token(Token::LParen),
            ')' => self.add_token(Token::RParen),
            '"' | '\'' => self.string(c)?,
            ';' => while !self.is_at_end() && self.peek() != '\n' {
                self.advance();
            }
            ' ' |  '\n' | '\t' | '\r' => {}
            _ => if c.is_digit(10) {
                self.number();
            } else {
                self.identifier();
            }
        }

        Ok(())
    }
    fn identifier(&mut self) {
        let end_chars = vec!['(', ')', ' ', '\n', '\t', '\r'];

        while !self.is_at_end() && !end_chars.contains(&self.peek()) {
            self.advance();
        }

        self.add_token(Token::Identifier(self.input[self.start..self.current].to_string()));
    }
    fn number(&mut self) {
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            self.advance();
        }

        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        let raw = self.input[self.start..self.current].to_string();

        match (&raw).parse::<i32>() {
            Ok(z) => self.add_token(Token::Number(z)),
            Err(_) => match raw.parse::<f32>() {
                Ok(f) => self.add_token(Token::Float(f)),
                _ => panic!("Bug: INVALID_NUMBER_SHOULD_BE_VALID."),
            }
        }
    }
    fn string(&mut self, delimiter: char) -> Result<()> {
        while !self.is_at_end() && self.peek() != delimiter {
            self.advance();
        }

        if self.is_at_end() {
            return error!("Unterminated String: {}.", self.input[self.start..].to_string());
        }

        self.advance();

        self.add_token(Token::String(self.input[self.start + 1..self.current - 1].to_string()));

        Ok(())
    }

}

#[cfg(test)]
mod lexing{
    use super::*;

    #[test]
    fn parentheses() -> Result<()> {
        let tokens = Lexer::new("()").tokenize()?;

        assert_eq!(tokens, vec![Token::LParen, Token::RParen]);
        Ok(())
    }

    #[test]
    fn numbers() -> Result<()> {
        let tokens = Lexer::new("42 3.1415").tokenize()?;

        assert_eq!(tokens, vec![Token::Number(42), Token::Float(3.1415)]);
        Ok(())
    }

    #[test]
    fn string() -> Result<()> {
        let tokens = Lexer::new(r#""foobar" 'moobar'"#).tokenize()?;

        assert_eq!(tokens, vec![Token::String("foobar".to_string()), Token::String("moobar".to_string())]);
        Ok(())
    }

    #[test]
    fn identifier() -> Result<()> {
        let tokens = Lexer::new("moow").tokenize()?;

        assert_eq!(tokens, vec![Token::Identifier("moow".to_string())]);
        Ok(())
    }
}
