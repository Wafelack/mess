use crate::{Result, error, Error, value::Value, interpreter::Interpreter};

impl Interpreter {
    pub fn at(&mut self, args: Vec<Value>) -> Result<Value> {

        if args.len() != 2 {
            return error!("Procedure `@` takes 2 arguments, but {} arguments were supplied.", args.len());
        }

        let collection = args[0].clone();
        let idx = args[1].clone();

        if let Value::Number(idx) = idx {

            let idx = if idx < 0 {
                return Ok(Value::Unit);
            } else {
                idx as usize
            };

            if let Value::Array(array) = collection {
                Ok(array.into_iter().nth(idx).unwrap_or(Value::Unit) 
)            } else if let Value::String(string) = collection {
                Ok(string.chars().nth(idx).and_then(|c| Some(Value::String(format!("{}", c)))).unwrap_or(Value::Unit))
            } else {
                error!("Expected a String or an Array, found a {}.", collection.get_type())
            }
        } else {
            error!("Expected a Number, found a {}.", idx.get_type())
        }

    }
}
