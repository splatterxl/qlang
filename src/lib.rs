use ansi_term::Style;

pub(crate) mod colors;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod tokens;

/// Format an [errors::Error] into a readable string.
macro_rules! format_err {
    ($error: tt, $filename: tt, $data: tt, $line: tt) => {
        println!(
            "{} at {}[{},{}]: {}\n{}\t{}\n  \t{}{}",
            colors::error(String::from("error")),
            $filename,
            colors::number($error.line),
            colors::number($error.at),
            $error.message,
            Style::new().dimmed().paint("at:"),
            $data.split(|c| c == '\n').nth($line).unwrap(),
            " ".repeat($error.at - 1),
            colors::error(String::from("^"))
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
        let filedata = &self.data;
        let mut lexer = lexer::Lexer::new(self.filename.clone());

        let output = lexer.lex(self.data.to_owned());

        let parsed = parser::Parser::parse(output);
        self.parsed = Some(parsed);

        let data = self.parsed.as_ref().unwrap();

        if data.errors.len() != 0 {
            for error in &data.errors {
                let line = error.line - 1;
                format_err!(error, filename, filedata, line)
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
