use std::env::args_os;
use std::io::Read;
use std::process::exit;

mod lexer;
mod parser;

use parser::Parser;

fn main() {
    file(&args_os().nth(1).expect("file argument must be passed").to_string_lossy());
}

fn file(path: &str) {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    println!("{} {}", error::dim("read file"), error::dim(path));

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }

    let now = std::time::Instant::now();
    dbg!(Parser::parse(contents));
    let elapsed = now.elapsed().as_micros();
    println!("{elapsed} micros")
}
