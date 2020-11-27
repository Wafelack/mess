use std::path::Path;
use clap::{Arg, App, SubCommand, ArgMatches};

fn get_lines(filename: &str) -> Vec<String> {
    let content = match std::fs::read(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open file");
            eprintln!("Debug info : {}", e);
            std::process::exit(-3);
        }
    };
    let stringed = match std::str::from_utf8(&content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("An error occured while converting file (probably cause it does not contain valid utf8)");
            eprintln!("Debug info : {}", e);
            std::process::exit(-2);
        }
    };
    stringed.to_string().split('\n').into_iter().map(|s| {s.to_string()}).collect::<Vec<String>>()
}  

fn is_specified(val: &str, matches: &ArgMatches) -> bool {
    if matches.occurrences_of(val) > 0 {
        return true;
    }
    false
}
fn main() {
    let matches = App::new("sgrep")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A grep implementation in rust")
        .arg(Arg::with_name("no-recursive")
            .short("nr")
            .long("no-recursive")
            .help("Disable the recursive flag")
            .takes_value(false))
        .arg(Arg::with_name("pattern")
            .help("The pattern to search")
            .required(true)
            .index(1))
        .arg(Arg::with_name("fname")
            .help("The filename to search in")
            .index(2))
        .get_matches();


    let mut recursive = is_specified("no-recursive", &matches);

    let pattern = &matches.value_of("pattern").unwrap();
    let fname = &matches.value_of("fname");

    if fname.is_some() {
        if !Path::new(fname.unwrap()).exists() {
            eprintln!("File not found");
            std::process::exit(-1);
        }
        let mut all_lines: Vec<(String, usize)> = vec![];
        let content = get_lines(fname.unwrap());
        let mut i = 0usize;
        for line in content  {
            i+=1;
            if line.contains(pattern) {
                all_lines.push((line, i));
            }            
       }
       for c in all_lines {
            println!("{} | {} | {}", &fname.unwrap(), c.1, c.0);
       }
       std::process::exit(0);
    }
}
