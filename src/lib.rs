use std::env::args;
use std::io::{stdout, Read};

mod ast;
mod lexer;
pub mod parser;
mod error;

#[macro_export]
macro_rules! debug {
    () => {
        eprintln!("debug: [{}:{}]", file!(), line!())
    }; 

    ($msg: expr) => {
        let msg = $msg;
        eprintln!("debug [{}:{}]: {:#?}", file!(), line!(), msg)
    };

    ($msg: tt, $($args:expr),*) => {
        eprintln!("debug [{}:{}]: {}", file!(), line!(), format!($msg, $($args),*))
    };
}


use crate::parser::Parse;

pub fn file(path: &str) {
    debug!("open file {}", &path);

    let mut file = std::fs::File::open(path).expect("unexpected error while opening file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error while reading file");

    let now = std::time::Instant::now();
    let parsed = contents.parse();
    debug!(&parsed);
    debug!("done in {}ms", now.elapsed().as_millis());
}
