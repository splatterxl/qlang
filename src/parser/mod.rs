use crate::{
    errors::{templates::Errors, Error},
    lexer::core::LexOutput,
    tokens::Token,
};

pub struct Parser {}

impl Parser {
    pub fn parse(lexed: LexOutput) -> ParseOutput {
        let mut errors = Vec::new();

        for token in lexed.lexed {
            match token {
                Token::UnknownKeyword(k) => {
                    errors.push(Errors::unknown_keyword(k.at.line, k.at.at_char, k.name))
                }
                _ => todo!(),
            }
        }

        ParseOutput { errors }
    }
}

#[derive(Debug)]
pub struct ParseOutput {
    pub errors: Vec<Error>,
}
