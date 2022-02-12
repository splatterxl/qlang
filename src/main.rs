use std::env::args_os;
use std::io::{stdout, Read};
use std::process::exit;

mod ast;
mod lexer;
mod parser;

use crate::parser::Parse;

fn main() {
    file(
        &args_os()
            .nth(1)
            .expect("file argument must be passed")
            .to_string_lossy(),
    );
}

fn file(path: &str) {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    println!("debug: read file {}", path);

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            println!("debug: working...")
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }

    let now = std::time::Instant::now();
    let parsed = contents.parse();
    dbg!(parsed);
    let elapsed = now.elapsed().as_micros();
    println!("debug: {elapsed}ms")
}
