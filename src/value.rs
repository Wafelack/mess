use std::collections::HashMap;
use crate::parser::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(i32),
    Float(f32),
    String(String),
    Array(Vec<Value>),
    Table(HashMap<String, Value>),
    Quote(Expr),
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
            Self::Quote(_) => "Quote",
        }.to_string()
    }
    pub fn get(&self, inner: bool) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::Float(f) => format!("{}", f),
            Self::String(s) => format!("{}", s),
            Self::Quote(expr) => format!("<#quote{{{}}}>", expr.get_type()),
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
