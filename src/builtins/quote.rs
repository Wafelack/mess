use crate::{Result, error, Error, interpreter::Interpreter, value::Value};

impl Interpreter {
    pub fn unquote(&mut self, args: Vec<Value>) -> Result<Value> {
        if let Value::Quote(e) = &args[0] {
            self.eval_expr(e.clone())
        } else {
            error!("Expected a Quote, found a {}.", args[0].get_type())
        }
    }
}
