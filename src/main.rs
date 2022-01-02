use std::io::Read;
use std::process::exit;

use clap::{App, Arg};

mod error;
mod lexer;
mod parser;

use parser::Parser;

fn main() {
    let app = App::new("Qlang")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Splatterxl <splatterxl@outlook.ie>")
        .arg(Arg::with_name("file").index(1).required(true));

    let params = app.get_matches();

    file(params.value_of("file").unwrap());
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

    if !contents.ends_with("\n") {
        contents.push('\n');
    }

    dbg!(Parser::parse(contents));
}
