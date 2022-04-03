use std::str::FromStr;

use self::ast::parser::Parser;

pub mod ast;
pub mod error;

pub use ast::ast::TopLevel;

impl FromStr for ast::ast::TopLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = Parser::new(s);

        Ok(parser.parse())
    }
}
