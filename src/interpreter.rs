use std::{collections::HashMap, str, process::Command};
use crate::{apply_escape, Result, Error, error, parser::Expr};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(i32),
    Float(f32),
    String(String),
    Array(Vec<Value>),
    Table(HashMap<String, Value>),
    Unit
}
impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Self::Number(_) => "Number",
            Self::Array(_) => "Array",
            Self::Float(_) => "Float",
            Self::String(_) => "String",
            Self::Unit => "Unit",
            Self::Table(_) => "Table",
        }.to_string()
    }
    pub fn get(&self, inner: bool) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::Float(f) => format!("{}", f),
            Self::String(s) => format!("{}", s),
            Self::Table(table) => {
                if inner {
                    format!("#<table {} rows>", table.len())
                } else {
                    let lengths_stringy = table.iter().map(|(k, v)| {
                        let stringy = v.get(true);
                        (if k.len() > stringy.len() {
                            k.len() + 2
                        } else {
                            stringy.len() + 2
                        }, stringy)
                    }).collect::<Vec<_>>();

                    let lengths = lengths_stringy.iter().rev().map(|(l, _)| {
                        l
                    }).collect::<Vec<_>>();

                    let mut toret = String::new();

                    table.keys().collect::<Vec<_>>().iter().rev().enumerate().for_each(|(idx, _)| {
                        toret.push_str(&format!("{:─^idx_len$}", "", idx_len=lengths[idx]));
                        if idx != table.len() - 1 {
                            toret.push('┬');
                        } else {
                            toret.push('\n');
                        }
                    });

                    table.keys().collect::<Vec<_>>().iter().rev().enumerate().for_each(|(idx, k)| {
                        toret.push_str(&format!("\x1b[0;32m{:^idx_len$}\x1b[0m", k, idx_len=lengths[idx]));
                        if idx != table.len() - 1 {
                            toret.push('│');
                        } else {
                            toret.push('\n')
                        }
                    });

                    table.keys().collect::<Vec<_>>().iter().rev().enumerate().for_each(|(idx, _)| {
                        toret.push_str(&format!("{:─^idx_len$}", "", idx_len=lengths[idx]));
                        if idx != table.len() - 1 {
                            toret.push('┼');
                        } else {
                            toret.push('\n');
                        }
                    });


                    lengths_stringy.iter().rev().enumerate().for_each(|(idx, (_, v))| {
                        toret.push_str(&format!("{:^idx_len$}", v, idx_len=lengths[idx]));
                        if idx != table.len() - 1 {
                            toret.push('|');
                        } else {
                            toret.push('\n');
                        }
                    });

                    toret
                }
            }
            Self::Array(vals) => {
                if inner {
                    format!("#<array {} rows>", vals.len())
                } else {
                    let mut stringified = vec![];
                    let mut longest = 0;

                    vals.into_iter().for_each(|v| {
                        let stringy = v.get(true);
                        if longest < stringy.len() {
                            longest = stringy.len();
                        }
                        stringified.push(stringy);
                    });

                    let idx_len = format!("{}", stringified.len() - 1).len() + 2;
                    longest += 2;
                
                    let mut toret = format!("{:─^idx_len$}┬{:─^longest$}\n", "", "", idx_len=idx_len, longest=longest);

                    for (idx, val) in stringified.into_iter().enumerate() {
                        toret.push_str(&format!("\x1b[1;32m{:^idx_len$}\x1b[0m│{:>longest$}\n", idx, val, idx_len=idx_len, longest=longest));                         
                    }

                    toret.push_str(&format!("{:─^idx_len$}┴{:─^longest$}\n", "", "", idx_len=idx_len, longest=longest));

                    toret

                }
            }
            Self::Unit => "()".to_string(),
        }
    }
}

pub struct Interpreter {
    input: Vec<Expr>,
    variables: HashMap<String, Value>,
    procedures: HashMap<String, (Vec<String>, Vec<Expr>)>,
    builtins: HashMap<String, fn(&mut Interpreter, Vec<Expr>) -> Result<Value>>,
}

impl Interpreter {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            variables: HashMap::new(),
            procedures: HashMap::new(),
            builtins: HashMap::new(),
        }
    }
    fn call(&mut self, func: String, argv: Vec<Expr>) -> Result<Value> {
        if self.builtins.contains_key(&func) {
            let callback = self.builtins[&func];
            callback(self, argv)
        } else if self.procedures.contains_key(&func) {
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
        } else {

            let mut args = vec![];

            for arg in argv {
                args.push(self.eval_expr(arg)?);
            }

            let out = match Command::new(func)
                .args(args.iter().map(|v| v.get(false)).collect::<Vec<_>>())
                .output() {
                    Ok(o) => o,
                    Err(e) => return error!("Failed to run command: {}.", e)
                };


            let stdout = match str::from_utf8(&out.stdout) {
                Ok(s) => s,
                Err(e) => return error!("Failed to get command output: {}.", e),
            };


            Ok(Value::String(apply_escape(stdout.trim())))
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
    pub fn assign(&mut self, name: impl ToString, value: Expr) -> Result<Value> {
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
    pub fn eval_expr(&mut self, expr: Expr) -> Result<Value> {

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
            Expr::Table(t) => {
                let mut toret = HashMap::new();

                for (k, v) in t {
                    toret.insert(k, self.eval_expr(v)?);
                }

                Ok(Value::Table(toret))
            }
            Expr::Array(content) => {
                let mut values = vec![];

                for elem in content {
                    values.push(self.eval_expr(elem)?);
                }

                Ok(Value::Array(values))
            }
        }
    }
    fn register_builtin(&mut self, builtin: impl ToString, associated: fn(&mut Interpreter, Vec<Expr>) -> Result<Value>) {
        self.builtins.insert(builtin.to_string(), associated);
    }
    fn register_builtins(&mut self) {
        self.register_builtin("+", Self::add);
        self.register_builtin("-", Self::sub);
        self.register_builtin("*", Self::mul);
        self.register_builtin("/", Self::div);
        self.register_builtin("cd", Self::cd);
        self.register_builtin("if", Self::r#if);
        self.register_builtin("@", Self::at);
    }
    pub fn eval(&mut self) -> Result<Value> {
        self.register_builtins();
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
