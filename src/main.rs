use lines_from_file::lines_from_file;

fn main() {
    let args: Vec<String> = std::env::args().into_iter().skip(1).collect();

    if args.len() >= 2 {
        if !std::path::Path::new(&args[1]).exists() {
            eprintln!("File `{}` not found", &args[1]);
            std::process::exit(-5);
        }
        let lines = lines_from_file(&args[1]);
        let mut counter: usize = 0;
        for line in lines {
            counter += 1;
            if line.contains(&args[0]) {
                println!("{} | {}", counter, line);
            }
        }
    } else if args.len() >= 2 {
    }
}
