use crate::{Result, error, Error, interpreter::{Value, Interpreter}, parser::{Expr}};

impl Interpreter {
    pub fn r#if(&mut self, args: Vec<Expr>) -> Result<Value> {
        if args.len() != 3 {
            return error!("Procedure `if` takes 3 arguments, but {} arguments were supplied.", args.len());
        }

        let cond = self.eval_expr(args[0].clone())?;

        let b_cond = if let Value::Number(n) = cond {
            n != 0
        } else if let Value::String(s) = cond {
            !s.is_empty()
        } else if let Value::Float(f) = cond {
            f != 0.
        } else {
            false
        };

        if b_cond {
            self.eval_expr(args[1].clone()) 
        } else {
            self.eval_expr(args[2].clone())
        }
    }
}
