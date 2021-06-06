use std::io::{self, Write};

// The code is disgusting but I made it just to try implementing the Shunting Yard

#[derive(Debug, PartialEq)]
enum Part {
    Operator(char),
    Number(f64),
}
#[derive(Debug)]
struct Error(pub String);

fn separate(code: String) -> Vec<String> {
    let mut i = 0;
    let mut to_ret = vec![];
    let chars = code.chars().collect::<Vec<char>>();
    while i < code.len() {
        match chars[i] {
            ' ' | '\n' | '\t' | '\r' => {},
            x if x.is_digit(10) => {
                let start = i;
                i += 1;
                while i < code.len() && chars[i].is_digit(10) {
                    i += 1;
                }
                if i < code.len() && chars[i] == '.' {
                    i += 1;
                }
                while i < code.len() && chars[i].is_digit(10) {
                    i += 1;
                }
                to_ret.push(code[start..i].to_string());
                continue;
            }
            c => to_ret.push(c.to_string()),
        }
        i += 1;
    }
    to_ret
}
fn digit_str(str_: impl ToString) -> bool {
    str_.to_string().chars().fold(true, |acc, c| acc && (c.is_digit(10) || c == '.'))
}

fn get_precedence(operator: char, precedence: &[Vec<char>]) -> Result<usize, Error> {
    precedence.iter().enumerate().map(|(idx, t)| {
        if t.contains(&operator) {
            Some(idx)
        } else {
            None
        }
    }).filter(|f| f.is_some()).last().map_or(Err(Error(format!("Unrecognized operator: `{}`.", operator))), |o| Ok(o.unwrap()))
}

fn compile(code: impl ToString) -> Result<Vec<Part>, Error> {
    let split = separate(code.to_string());
    let mut queue = vec![];
    let mut stack = vec![];
    let precedence = vec![vec!['+', '-'], vec!['*', '/']];

    split.into_iter().map(|s| {
        if digit_str(&s) {
            queue.push(Part::Number(s.parse::<f64>().unwrap()));
        } else {
            let c = s.chars().last().unwrap();
            if c == ')' {
                let mut found = false;
                while let Some(c) = stack.pop() {
                    if c == '(' {
                        found = true;
                        break;
                    }
                    queue.push(Part::Operator(c))
                }
                if !found {
                    return Err(Error("Mismatched parenthesis.".to_string()));
                }
            } else if c == '(' {
                stack.push(c);
            } else {
                match stack.last() {
                    Some(o) => if *o != '(' && get_precedence(*o, &precedence)? > get_precedence(c, &precedence)? {
                        queue.push(Part::Operator(stack.pop().unwrap()));
                        stack.push(c);
                    } else {
                        stack.push(c);
                    }
                    None => stack.push(c),
                }
            }
        }
        Ok(())
    }).collect::<Result<(), Error>>()?;
    stack.into_iter().rev().for_each(|c| queue.push(Part::Operator(c)));
    Ok(queue)
}
use std::collections::HashMap;
fn run(code: &[Part]) -> Result<f64, Error> {
    let mut stack = Vec::with_capacity(256);
    let mut functions: HashMap<char, fn(&mut Vec<f64>)> = HashMap::new();
    functions.insert('+', |stack: &mut Vec<f64>| {
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        stack.push(lhs + rhs);
    });
    functions.insert('-', |stack: &mut Vec<f64>| {
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        stack.push(lhs - rhs);
    });
    functions.insert('*', |stack: &mut Vec<f64>| {
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        stack.push(lhs * rhs);
    });
    functions.insert('/', |stack: &mut Vec<f64>| {
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        stack.push(lhs / rhs);
    });

    code.into_iter().map(|p| {
        match p {
            Part::Number(x) => {
                stack.push(*x)
            }
            Part::Operator(op) => if functions.contains_key(op) {
                functions[op](&mut stack);
            } else {
                return Err(Error(format!("Unrecognised operator: `{}`.", op)));
            },
        }
        Ok(())
    }).collect::<Result<(), Error>>()?;
    Ok(stack.last().and_then(|f| Some(*f)).unwrap_or(0.))
}

fn main() {
    loop {
        let mut buffer = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let compiled = match compile(buffer.trim()) {
            Ok(c) => c,
            Err(e) => {
                println!("\x1b[0;31mError: \x1b[0m {}", e.0);
                continue;
            }
        };
        match run(&compiled) {
            Ok(f) => println!("{}", f),
            Err(e) => {
                println!("\x1b[0;31mError: \x1b[0m {}", e.0);
                continue;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sep() {
        let sep = separate("5 + 23 + 3.14 + (12 + 3)".to_string());
        assert_eq!(sep, ["5", "+", "23", "+", "3.14", "+", "(", "12", "+", "3", ")"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
    }
    #[test]
    fn comp() -> Result<(), Error> {
        let comp = compile("5 * (10 + 2) - 2")?;
        assert_eq!(comp, vec![Part::Number(5.), Part::Number(10.), Part::Number(2.), Part::Operator('+'), Part::Operator('*'), Part::Number(2.), Part::Operator('-')]);
        Ok(())
    }
}
