use crate::{Result, error, Error, interpreter::{Value, Interpreter}, parser::{Expr}};

impl Interpreter {
    pub fn add(&mut self, args: Vec<Expr>) -> Result<Value> {

        if args.len() < 2  {
            return error!("Procedure `+` takes 2 or more arguments, but {} arguments were supplied.", args.len());
        }

        if let Expr::Number(mut n) = args[0] {
            for arg in args.iter().skip(1) {
                if let Expr::Number(n0) = arg {
                    n += *n0;
                } else {
                    return error!("Expected a Number, found a {}.", args[0].get_type());
                }
            }

            Ok(Value::Number(n))

        } else if let Expr::Float(mut f) = args[0] {

            for arg in args.iter().skip(1) {
                if let Expr::Float(f0) = arg {
                    f += *f0;
                } else {
                    return error!("Expected a Float, found a {}.", args[0].get_type());
                }
            }

            Ok(Value::Float(f))
        } else {
            error!("Expected a Number or a Float, found a {}.", args[0].get_type())
        }

    }

    pub fn sub(&mut self, args: Vec<Expr>) -> Result<Value> {

        if args.len() < 2  {
            return error!("Procedure `-` takes 2 or more arguments, but {} arguments were supplied.", args.len());
        }

        if let Expr::Number(mut n) = args[0] {
            for arg in args.iter().skip(1) {
                if let Expr::Number(n0) = arg {
                    n -= *n0;
                } else {
                    return error!("Expected a Number, found a {}.", args[0].get_type());
                }
            }

            Ok(Value::Number(n))

        } else if let Expr::Float(mut f) = args[0] {

            for arg in args.iter().skip(1) {
                if let Expr::Float(f0) = arg {
                    f -= *f0;
                } else {
                    return error!("Expected a Float, found a {}.", args[0].get_type());
                }
            }

            Ok(Value::Float(f))
        } else {
            error!("Expected a Number or a Float, found a {}.", args[0].get_type())
        }

    }

    pub fn mul(&mut self, args: Vec<Expr>) -> Result<Value> {

        if args.len() < 2  {
            return error!("Procedure `*` takes 2 or more arguments, but {} arguments were supplied.", args.len());
        }

        if let Expr::Number(mut n) = args[0] {
            for arg in args.iter().skip(1) {
                if let Expr::Number(n0) = arg {
                    n *= *n0;
                } else {
                    return error!("Expected a Number, found a {}.", args[0].get_type());
                }
            }

            Ok(Value::Number(n))

        } else if let Expr::Float(mut f) = args[0] {

            for arg in args.iter().skip(1) {
                if let Expr::Float(f0) = arg {
                    f *= *f0;
                } else {
                    return error!("Expected a Float, found a {}.", args[0].get_type());
                }
            }

            Ok(Value::Float(f))
        } else {
            error!("Expected a Number or a Float, found a {}.", args[0].get_type())
        }

    }

    pub fn div(&mut self, args: Vec<Expr>) -> Result<Value> {

        if args.len() != 2  {
            return error!("Procedure `/` takes 2 arguments, but {} arguments were supplied.", args.len());
        }

        match args[0] {
            Expr::Number(lhs) => match args[1] {
                Expr::Number(rhs) => Ok(Value::Number(lhs / rhs)),
                _ => error!("Expected a Number, found a {}.", args[1].get_type()),
            }
            Expr::Float(lhs) => match args[1] {
                Expr::Float(rhs) => Ok(Value::Float(lhs / rhs)),
                _ => error!("Expected a Float, found a {}.", args[1].get_type()),
            }
            _ => error!("Expected a Number or a Float, found a {}.", args[0].get_type())
        } 
    }



}
