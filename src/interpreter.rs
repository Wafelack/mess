use std::collections::HashMap;
use crate::{Result, Error, error, parser::Expr};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(i32),
    Float(f32),
    String(String),
    Unit
}
impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Self::Number(_) => "Number",
            Self::Float(_) => "Float",
            Self::String(_) => "String",
            Self::Unit => "Unit"
        }.to_string()
    }
    pub fn get_lit(&self) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::Float(f) => format!("{}", f),
            Self::String(s) => format!("\"{}\"", s),
            Self::Unit => "()".to_string(),
        }
    }
}

pub struct Interpreter {
    input: Vec<Expr>,
    variables: HashMap<String, Value>,
    procedures: HashMap<String, (Vec<String>, Vec<Expr>)>,
}

impl Interpreter {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            variables: HashMap::new(),
            procedures: HashMap::new(),
        }
    }
    fn call(&mut self, func: String, argv: Vec<Expr>) -> Result<Value> {
        if !self.procedures.contains_key(&func) {
            error!("Unbound procecure: {}.", func)
        } else {
            let (args, body) = self.procedures[&func].clone();

            if args.len() != argv.len() {
                error!("Procedure `{}` takes {} arguments but {} arguments were supplied.", &func, args.len(), argv.len())
            } else {
                let mut to_replace = vec![];
                for (idx, arg) in argv.into_iter().enumerate() {
                    if self.variables.contains_key(&args[idx]) {
                        to_replace.push(Some(self.variables[&args[idx]].clone()));
                    } else {
                        to_replace.push(None);
                    }

                    self.assign(&args[idx], arg)?;
                }

                self.eval_exprs(body)
            }
        }

    }
    pub fn update_ast(&mut self, ast: Vec<Expr>) {
        self.input = ast;
    }
    fn eval_exprs(&mut self, exprs: Vec<Expr>) -> Result<Value> {
        let length = exprs.len();
        for (idx, expr) in exprs.into_iter().enumerate() {
            let evaluated = self.eval_expr(expr)?;
            if idx == length - 1{
                return Ok(evaluated); 
            } 
        }

        panic!("Bug: UNTRIGGERED_RETURN");
    }
    fn assign(&mut self, name: impl ToString, value: Expr) -> Result<Value> {
        let value = self.eval_expr(value)?;
        let name = name.to_string();
        if self.variables.contains_key(&name) {
            self.variables.remove(&name).unwrap();
        }
        self.variables.insert(name, value);

        Ok(Value::Unit)
    }
    fn procedure(&mut self, name: String, args: Vec<String>, body: Vec<Expr>) -> Result<Value> {

        if self.procedures.contains_key(&name) {
            *self.procedures.get_mut(&name).unwrap() = (args, body);
        } else {
            self.procedures.insert(name, (args, body));
        }

        Ok(Value::Unit)
    }
    fn eval_expr(&mut self, expr: Expr) -> Result<Value> {

        match expr {
            Expr::Var(s) => if self.variables.contains_key(&s) {
                Ok(self.variables[&s].clone())
            } else {
                error!("Unbound variable: {}.", s)
            }
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::Float(f) => Ok(Value::Float(f)),
            Expr::Unit => Ok(Value::Unit),
            Expr::Call(func, argv) => self.call(func, argv),
            Expr::Let(name, value) => self.assign(name, *value),
            Expr::Defun(name, args, body) => self.procedure(name, args, body),
        }
    }
    pub fn eval(&mut self) -> Result<Value> {
        let input = self.input.clone();
        self.eval_exprs(input)
    }
}

#[cfg(test)]
mod evaluation {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn r#let() -> Result<()> {
        let tokens = Lexer::new("(let foo 5)#foo").tokenize()?;
        let ast = Parser::new(tokens).parse()?;
        let out = Interpreter::new(ast).eval()?;
        assert_eq!(out, Value::Number(5));

        Ok(())
    }

    #[test]
    fn procedures() -> Result<()> {
        let tokens = Lexer::new("(defun (pi) 3.1415926535897932)(pi)").tokenize()?;
        let ast = Parser::new(tokens).parse()?;
        let out = Interpreter::new(ast).eval()?;
        assert_eq!(out, Value::Float(3.1415926535897932));

        Ok(())
    }

}
