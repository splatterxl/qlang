pub mod core;

/// A lexing provider.
pub struct Lexer {
    pub filename: String,
}

impl Lexer {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }

    /// Lex the provided `file`'s data.
    pub fn lex(&mut self, file: String) -> core::LexOutput {
        let provider = core::LexProvider::new(file, self.filename.to_owned());

        provider.lex()
    }
}
