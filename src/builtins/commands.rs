use crate::{Result, error, Error, value::Value, interpreter::Interpreter};
use std::{fs, path::Path};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

impl Interpreter {
    pub fn ls(&mut self, args: Vec<Value>) -> Result<Value> {
        let mut hidden = false;
        let mut mode = false;
        let mut fname = None;

        for arg in args {
            if let Value::String(s) = arg {
                match s.as_str() {
                    "-s" | "--show-hiden" => hidden = true,
                    "-m" | "--mode" => mode = true,
                    _ => fname = Some(s)
                } 
            } else {
                return error!("Expected a String, found a {}.", arg.get_type());
            }
        }
        let fname = fname.unwrap_or(".".to_string());
        let pathed = Path::new(&fname);

        let mut keys = vec!["name".to_string(), "type".to_string(), "size".to_string()];

        if mode  {
            keys.push("mode".to_string());
        }

        if !pathed.exists() {
            error!("ls: `{}`: not found.", fname)
        } else if pathed.is_file() {
           error!("ls: `{}`: not a directory", fname)
        } else {
            let content = match fs::read_dir(&fname) {
                Ok(r) => r,
                Err(e) => return error!("ls: cannot read `{}`: {}", fname, e),
            };

            let mut values = vec![];

            for entry in content {
                let mut sub = vec![];

                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => return error!("ls: failed to read entry: {}.", e),
                };
                let pathed = entry.path();

                if !hidden && pathed.file_name().unwrap().to_str().unwrap().starts_with(".") {
                    continue;
                } 
                sub.push(Value::String(pathed.file_name().unwrap().to_str().unwrap().to_string()));
                sub.push(Value::String(if pathed.is_dir() { "directory" } else { "file" }.to_string()));
                let metadata = match pathed.metadata() {
                    Ok(m) => m,
                    Err(_) => return error!("ls: `{}`: failed to get file metadata.", &fname),
                };
                let size = metadata.len();
                sub.push(Value::String(if size > 1_000_000_000 {
                    format!("{:.1}GB", size as f32 / 1_000_000_000.)
                } else if size > 1_000_000 {
                    format!("{:.1}MB", size as f32 / 1_000_000.)
                } else if size > 1_000 {
                    format!("{:.1}kB", size as f32 / 1_000.)
                } else {
                    format!("{}B", size)
                }));

                if mode && cfg!(unix) {
                    sub.push(Value::String(format!("{:o}", metadata.permissions().mode())))
                }
                values.push(sub)
            }
        
            Ok(Value::Table(keys, values))
        }
    }
}
