use crate::{Result, error, Error, value::Value, interpreter::Interpreter};

impl Interpreter {
    pub fn r#if(&mut self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 3 {
            return error!("Procedure `if` takes 3 arguments, but {} arguments were supplied.", args.len());
        }

        let cond = &args[0];

        let b_cond = if let Value::Number(n) = cond {
            *n != 0
        } else if let Value::String(s) = cond {
            !s.is_empty()
        } else if let Value::Float(f) = cond {
            *f != 0.
        } else {
            false
        };

        if b_cond {
            self.unquote(vec![args[1].clone()])
        } else {
            self.unquote(vec![args[2].clone()])
        }
    }
}
