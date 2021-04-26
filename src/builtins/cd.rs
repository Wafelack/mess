use crate::{Result, error, Error, value::Value, interpreter::Interpreter, parser::Expr};
use std::env::{self, set_current_dir, current_dir};

impl Interpreter {
    pub fn cd(&mut self, args: Vec<Value>) -> Result<Value> {
        if !(0..=1).contains(&args.len()) {
            return error!("Function `cd` takes 0 or 1 arguments, but {} arguments were supplied.", args.len());
        }

        let current = match current_dir() {
            Ok(p) => p.to_str().unwrap().to_string(),
            Err(e) => return error!("Failed to get current directory: {}.", e),
        };

        let home = match env::var("HOME") {
            Ok(s) => s,
            Err(e) => return error!("Failed to read $HOME: {}.", e),
        };



        if args.len() == 0 {

            self.assign("previous-dir", Expr::String(current))?;

            match set_current_dir(&home) {
                Ok(_) => Ok(Value::Unit),
                Err(e) => error!("Failed to change directory to {}: {}.", &home, e),
            }
        } else {
            let path = if let Value::String(s) = &args[0] {
                s.replace("~", &home)
            } else {
                return error!("Expected a String, found a {}.", args[0].get_type());
            };

            self.assign("previous-dir", Expr::String(current))?;

            match set_current_dir(&path) {
                Ok(_) => Ok(Value::Unit),
                Err(e) => error!("Failed to change directory to {}: {}.", &path, e),
            }            
        }

    }
}
