pub fn lex(code: String) -> Vec<Tokens> {
    println!("Lexing...");
    let mut lexer = Lexer::new(code);
    lexer.lex()
}

struct Lexer {
    raw: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { raw: code }
    }

    pub fn lex(&mut self) -> Vec<Tokens> {
        let mut i = 0;
        let mut tokens = Vec::new();
        let mut chars = String::new();
        let mut in_string = false;
        while i < self.raw.len() {
            let char = self.raw.chars().nth(i).unwrap();

            if in_string && char != '"' {
                chars.push(char);
                i += 1;
                continue;
            }

            match &char {
                '{' => tokens.push(Tokens::LCurly),
                '}' => tokens.push(Tokens::RCurly),
                '(' => tokens.push(Tokens::LParen),
                ')' => tokens.push(Tokens::RParen),
                '[' => tokens.push(Tokens::LSquare),
                ']' => tokens.push(Tokens::RSquare),
                '.' => tokens.push(Tokens::Dot),
                ',' => tokens.push(Tokens::Comma),
                '+' => tokens.push(Tokens::Plus),
                '-' => tokens.push(Tokens::Minus),
                '%' => tokens.push(Tokens::Percent),
                '=' => tokens.push(Tokens::Equals),
                '!' => tokens.push(Tokens::Bang),
                '?' => tokens.push(Tokens::Question),
                '<' => tokens.push(Tokens::LessThan),
                '>' => tokens.push(Tokens::GreaterThan),
                '/' => {
                    if (&tokens).ends_with(&[Tokens::ForwardSlash]) {
                        tokens.remove(tokens.len() - 1);
                        tokens.push(Tokens::Comment("".to_string()));
                    } else {
                        tokens.push(Tokens::ForwardSlash)
                    }
                }
                '|' => tokens.push(Tokens::BitOr),
                '&' => tokens.push(Tokens::BitAnd),
                ';' => tokens.push(Tokens::Semicolon),
                ':' => tokens.push(Tokens::Colon),
                '$' => tokens.push(Tokens::Dollar),
                '@' => tokens.push(Tokens::At),
                '#' => tokens.push(Tokens::Hash),
                '*' => tokens.push(Tokens::Star),
                '^' => tokens.push(Tokens::Caret),
                '"' => {
                    if !in_string {
                        in_string = true;
                    } else {
                        in_string = false;
                        tokens.push(Tokens::String(chars.clone()));
                        chars = String::new();
                    }
                }
                ' ' | '\t' | '\r' => {
                    if !chars.is_empty()
                        && !tokens.last().unwrap_or(&Tokens::Unknown).is_comment()
                        && !in_string
                    {
                        tokens.push(Tokens::Keyword(chars));
                        chars = String::new();
                    }
                }
                '\n' => {
                    if !chars.is_empty() && !in_string {
                        if tokens[tokens.len() - 1].is_comment() {
                            tokens.remove(tokens.len() - 1);
                            tokens.push(Tokens::Comment(chars.clone()));
                            chars = String::new();
                        } else {
                            tokens.push(Tokens::Keyword(chars.clone()));
                            chars = String::new();
                        }
                    }
                }
                char => {
                    if char.is_alphanumeric() {
                        chars.push(*char);
                    }
                }
            }

            i += 1;
        }

        tokens
    }
}

#[derive(Debug, PartialEq)]
pub enum Tokens {
    // Single char tokens
    Dot,
    Comma,
    RSquare,
    LSquare,
    RParen,
    LParen,
    RCurly,
    LCurly,
    Plus,
    Minus,
    Percent,
    Equals,
    Bang,
    Question,
    LessThan,
    GreaterThan,
    ForwardSlash,
    BitOr,
    BitAnd,
    Semicolon,
    Colon,
    Dollar,
    At,
    Hash,
    Star,
    Caret,

    // Two-char tokens
    Comment(String),

    // Multi-char tokens
    Keyword(String),
    String(String),

    // Others
    Unknown,
}

impl Tokens {
    pub fn is_comment(&self) -> bool {
        match self {
            Tokens::Comment(..) => true,
            _ => false,
        }
    }
}
