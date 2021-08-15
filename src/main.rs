use std::process::exit;

use clap::{App, Arg};
use qlang::File;

fn main() {
    let app = App::new("Qlang")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Splatterxl <splatterxl@outlook.ie>")
        .about("Qlang is a minimal, keyword-based, simple programming language.")
        .arg(Arg::with_name("file").index(1).required(true));

    let params = app.get_matches();

    file(params.value_of("file").unwrap());
}

fn file(_path: &str) {

    let file_data = String::from("echo (\"Hello, World!\");");

    let mut file = File {
        data: file_data,
        filename: String::from("aaaaa.q"),
        parsed: None
    };

    match file.parse() {
        Ok(_) => {},
        Err(_) => {
            println!("\nAborting due to errors.");
            exit(1);
        }
    }

    dbg!(&file);

}
