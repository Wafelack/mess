use std::collections::HashMap;
use crate::parser::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(i32),
    Float(f32),
    String(String),
    Array(Vec<Value>),
    Table(Vec<String>, Vec<Vec<Value>>),
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
            Self::Table(_, _) => "Table",
            Self::Quote(_) => "Quote",
        }.to_string()
    }
    pub fn get(&self, inner: bool) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::Float(f) => format!("{}", f),
            Self::String(s) => format!("{}", s),
            Self::Quote(expr) => format!("<#quote{{{}}}>", expr.get_type()),
            Self::Table(keys, values) => {
                let mut length = vec![];

                for (idx, key) in keys.iter().enumerate() {
                    let mut len = key.len();

                    for val in values.iter() {
                        let got = val[idx].get(true);
                        if got.len() > len {
                            len = got.len();
                        }
                    }
                    length.push(len + 2);
                }

                let idx_len = format!("{}", values.len()).len() + 2;

                let mut first_row= format!("{:─^idx_len$}", "", idx_len=idx_len);
                let mut second_row=  format!("\x1b[1;32m{:^idx_len$}\x1b[0m", "@", idx_len=idx_len);

                for (idx, key) in keys.iter().rev().enumerate() {
                    first_row.push_str(&format!("┬{:─^length$}", "", length=length[idx]));
                    second_row.push_str(&format!("│\x1b[1;32m{:^length$}\x1b[0m", key, length=length[idx]));
                }

                let mut toret = format!("{}\n{}\n{}\n", first_row, second_row, first_row.replace("┬", "┼"));

                for (idx, val) in values.into_iter().enumerate() {
                    toret.push_str(&format!("\x1b[1;32m{:^idx_len$}\x1b[0m", idx, idx_len=idx_len));

                    for (i, elem)  in val.iter().enumerate() {
                        toret.push_str(&format!("│{:^length$}", elem.get(inner), length=length[i]));
                    }
                    toret.push('\n')
                }
                toret.push_str(&format!("{:─^idx_len$}", "", idx_len=idx_len)); 
                for (idx, _)  in keys.into_iter().enumerate() {
                    toret.push_str(&format!("┴{:─^length$}", "", length=length[idx]));
                }

                toret
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
