use std::{
    error::Error,
    fs::File,
    io::{stdin as io_stdin, Read},
    time::Instant,
};

use crate::parser::TopLevel;

pub mod llvm;
pub mod parser;
pub use inkwell;

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

pub fn stdin() -> Result<TopLevel, Box<dyn Error>> {
    let mut input = String::new();

    let mut buf = io_stdin();

    buf.read_to_string(&mut input)?;

    let root = parse_file(input);

    Ok(root)
}

/// Parse a file into TopLevel data.
pub fn file(path: &str) -> Result<TopLevel, Box<dyn Error>> {
    let mut content = String::new();

    File::open(&path)?.read_to_string(&mut content)?;

    let root = parse_file(content);

    Ok(root)
}

fn parse_file(raw: String) -> TopLevel {
    let now = Instant::now();
    let parsed = raw.parse::<TopLevel>().unwrap();
    debug!(&parsed);
    debug!("done in {}ms", now.elapsed().as_millis());

    parsed
}
