pub mod errors;
pub mod lexer;
pub mod parser;
pub mod tokens;

macro_rules! format_err {
    ($error: tt, $filename: tt) => {
        println!(
            "error at {}[{},{} ({})]: {}",
            $filename,
            $error.line,
            $error.at,
            {
                match &$error.symbol {
                    None => String::from("root"),
                    Some(symbol) => symbol.to_owned(),
                }
            },
            $error.message
        )
    };
}

#[derive(Debug)]
pub struct File {
    pub filename: String,
    pub data: String,
    pub parsed: Option<parser::ParseOutput>,
}

impl File {
    pub fn parse(&mut self) -> Result<(), ()> {
        let filename = &self.filename;
        let mut lexer = lexer::Lexer::new(self.filename.clone());

        let output = lexer.lex(self.data.to_owned());

        let parsed = parser::Parser::parse(output);
        self.parsed = Some(parsed);

        let data = self.parsed.as_ref().unwrap();

        if data.errors.len() != 0 {
            for error in &data.errors {
                format_err!(error, filename)
            }
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn run() {
        todo!();
    }
}
