pub mod lexer;
pub mod tokens;
pub mod parser;
pub mod errors;

#[derive(Debug)]
pub struct File {
    pub filename: String,
    pub data: String,
    pub parsed: Option<parser::ParseOutput>
}

impl File {
    pub fn parse(&mut self) {
        let mut lexer = lexer::Lexer::new(self.filename.clone());

        let output = lexer.lex(self.data.to_owned());


        let parsed = parser::Parser::parse(output);

        self.parsed = Some(parsed);
    }

    pub fn run() {
        todo!();
    }
}
