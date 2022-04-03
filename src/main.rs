use std::{
    env,
    process::{exit, Command},
};

mod args;
use args::parse as args;
use repl::repl;

mod repl;

fn main() {
    header();
    let (flags, args, _) = args(false);

    if flags.contains(&"help".to_string()) {
        help(false);
        exit(0);
    } else if flags.contains(&"version".to_string()) {
        version();
        exit(0);
    } else if args.is_empty() {
        repl();
    } else {
        match args.get(0).unwrap().as_str() {
            "help" => help(true),
            "run" => run(args),
            "repl" => repl(),
            cmd => {
                eprintln!("Unknown subcommand: {}", cmd);
                exit(1);
            }
        }
    }
}

// Projects

fn run(args: Vec<String>) {
    let file = args.get(1).unwrap();

    qlang::file(file).ok();
}

// Metadata

pub const HELP: &str = "\
If no arguments are given, the REPL is started.

Subcommands:
    run [file] [-- args...]
        Runs the specified file or the current project.
    repl
        Starts the interactive Qlang REPL.

Options:
    -h, --help
        Prints this message.
    -v, --version
        Prints the version of this program.
    -V, --verbose
        Prints additional debug information.
    -vv, --very-verbose
        Prints all debug information. \
";

pub fn help(spawn: bool) {
    if spawn {
        Command::new("man")
            .args(Vec::from(["1".to_string(), "qlang".to_string()]))
            .spawn()
            .unwrap_or_else(|_| {
                println!("{}", HELP);
                exit(0);
            })
            .wait()
            .unwrap_or_else(|_| {
                println!("{}", HELP);
                exit(0);
            });
    } else {
        println!("{}", HELP);
    }
}

fn header() {
    eprintln!("Copyright (C) 2021, 2022 Splatterxl \n")
}

fn version() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}
