use std::io::{self, Write};

use inkwell::context::Context;
use qlang::llvm::codegen::Codegen;

#[inline]
fn prompt() -> String {
    print_flush("%> ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}

#[inline]
fn print_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn repl() {
    let context = Context::create();
    let module = context.create_module("qlang_repl");
    let codegen = Codegen::new(&context, module);

    loop { 
        match prompt().as_str() {
            ".q" => break,
            ".h" => {
                println!("\
                         .q: quit\n\
                         .h: help\n\
                         ");
            },
            code => {
                println!("{:?}", codegen.interpret(code));
            }
        }
    }
}
