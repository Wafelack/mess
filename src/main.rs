mod lexer;
mod parser;
mod interpreter;
mod builtins;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

use std::io::{self, Write};

fn main() -> Result<()> {
    let mut i = 0;
    let mut interpreter = Interpreter::new(vec![]);
    loop {
        i += 1;
        print!("(REPL:{:03}) ", i);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let tokens = Lexer::new(buffer.trim()).tokenize()?;
        let ast = Parser::new(tokens).parse()?;
        interpreter.update_ast(ast);
        let output = interpreter.eval()?;
        println!("{}", output.get());
    }
}

#[derive(Debug)]
pub struct Error(pub String);
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Err(Error(format_args!($($arg)*).to_string()))
    }
}

pub fn apply_escape(input: &str) -> String {
    input
        .replace("\\x1b", "\x1b")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\0", "\0")
        .replace("\\\\", "\\")
        .to_string()
}

