use crate::tokens::{BasicToken, Keyword, Token};

#[derive(Debug)]
pub struct LexOutput {
    pub filename: String,
    pub original: String,
    pub lexed: Vec<Token>,
}

pub struct LexProvider {
    pub data: String,
    pub filename: String,
}

impl LexProvider {
    pub fn new(data: String, filename: String) -> Self {
        Self { data, filename }
    }

    pub fn lex(self) -> LexOutput {
        let mut out = Vec::new();

        let mut line_number = 0;

        for line in self.data.split(|c| c == '\n') {
            line_number += 1;

            let mut in_quote = false;

            let mut statement = line.split(|c| {
                if c == '"' || c == '\'' {
                    in_quote = !in_quote;
                    false
                } else if c == ' ' && !in_quote {
                    true
                } else {
                    false
                }
            });

            let keyword = String::from(statement.nth(0).unwrap_or(""));

            if keyword == "" {
                continue;
            }

            match keyword {
                name => {
                    out.push(Token::UnknownKeyword(Keyword {
                        name,
                        at: BasicToken {
                            line: line_number,
                            at_char: 1,
                        },
                    }));
                }
            }
        }

        LexOutput {
            filename: self.filename,
            lexed: out,
            original: self.data,
        }
    }
}
