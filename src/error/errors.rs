use logos::Span;

use crate::lexer::Tokens;

use super::{
    codes::{
        EXPECTED_SEMICOLON, IMPORT_NO_LIB, IMPORT_KEYWORD, INVALID_NUMBER,
        UNEXPECTED_END_OF_INPUT, UNEXPECTED_TOKEN, UNIMPLEMENTED_FEATURE,
    },
    Error,
};

pub struct Errors {}
impl Errors {
    pub fn import_no_lib(line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Expected string literal after import keyword".to_string())
            .code(IMPORT_NO_LIB)
            .lc((line, column))
            .hints(vec!["did you forget to add a library?".to_string()])
            .build()
    }
    pub fn unexpected_token(expected: &str, got: Tokens, line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Unexpected token".to_string())
            .code(UNEXPECTED_TOKEN)
            .lc((line, column))
            .hints(vec![format!("expected {}, got {:?}", expected, got)])
            .build()
    }
    pub fn expected_semicolon(got: Tokens, line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Expected semicolon".to_string())
            .code(EXPECTED_SEMICOLON)
            .lc((line, column))
            .hints(vec![format!("expected semicolon, got {:?}", got)])
            .build()
    }
    pub fn unexpected_end_of_input(line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Unexpected end of input".to_string())
            .code(UNEXPECTED_END_OF_INPUT)
            .lc((line, column))
            .build()
    }
    pub fn import_keyword(line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Unknown keyword".to_string())
            .code(IMPORT_KEYWORD)
            .lc((line, column))
            .hints(vec![
                "imports in Qlang work strangely, see the documentation for more info".to_string(),
            ])
            .build()
    }
    // TODO: implement displaying start to end of data
    pub fn invalid_number(line: u32, column: u32, _end: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Invalid number".to_string())
            .code(INVALID_NUMBER)
            .lc((line, column))
            .hints(vec![
                "number literals can't have alphabetical characters in them!".to_string(),
            ])
            .build()
    }
    pub fn unimplemented_feature(line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Unimplemented feature".to_string())
            .code(UNIMPLEMENTED_FEATURE)
            .lc((line, column))
            .hints(vec!["this feature is not yet implemented in Qlang, it may be added or removed in a future version".to_string()])
            .build()
    }
    pub fn curly_bracket_import(line: u32, column: u32) -> Error {
        SyntaxErrorBuilder::new()
            .message("Unexpected token".to_string())
            .code(UNEXPECTED_TOKEN)
            .lc((line, column))
            .hints(vec!["import statements use parentheses".to_string()])
            .build()
    }
}

trait ErrorBuilder {
    fn new() -> Self;
    fn message(self, message: String) -> Self;
    fn code(self, code: u32) -> Self;
    fn hints(self, hints: Vec<String>) -> Self;
    fn build(self) -> Error;
}

struct SyntaxErrorBuilder {
    message: Option<String>,
    code: Option<u32>,
    hints: Option<Vec<String>>,
    span: Option<Span>,
    lc: Option<(u32, u32)>,
}

impl SyntaxErrorBuilder {
    fn lc(mut self, lc: (u32, u32)) -> Self {
        self.lc = Some(lc);
        self
    }
}

impl ErrorBuilder for SyntaxErrorBuilder {
    fn new() -> Self {
        SyntaxErrorBuilder {
            message: None,
            code: None,
            hints: None,
            span: None,
            lc: None,
        }
    }

    fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    fn code(mut self, code: u32) -> Self {
        self.code = Some(code);
        self
    }

    fn hints(mut self, hints: Vec<String>) -> Self {
        self.hints = Some(hints);
        self
    }

    fn build(self) -> Error {
        Error::Syntax {
            message: self.message.unwrap(),
            span: self.span.unwrap_or(0..0),
            hints: self.hints.unwrap_or(Vec::new()),
            code: self.code.unwrap(),
            lc: self.lc.unwrap_or((0, 0)),
        }
    }
}
