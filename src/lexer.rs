use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub line: u32,
    pub column: u32,
    pub end: u32,
    pub data: T,
}

pub struct Lexer {
    raw: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { raw: code }
    }

    pub fn vec(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();
        let mut lexer = Tokens::lexer(&mut self.raw);
        let mut line = 0;
        let mut column = 0;

        let _ = column;

        while let Some(token) = lexer.next() {
            column += lexer.slice().len() as u32;

            if token.is_whitespace() {
                let mut newlines = 0;

                for char in token.get_whitespace().chars() {
                    if char == '\n' {
                        newlines += 1;
                    }
                }

                if newlines > 0 {
                    line += newlines;
                    column = 0;
                }
                continue;
            } else {
            }

            vec.push(Token {
                line,
                data: token,
                column: column as u32,
                end: column as u32 + lexer.slice().len() as u32,
            });
        }

        vec.push(Token {
            data: Tokens::EOF,
            line,
            column: column as u32,
            end: column as u32 + lexer.slice().len() as u32,
        });

        vec
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Tokens {
    // Single-char tokens
    #[token(".")]
    Dot,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    ForwardSlash,
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
    #[token("(")]
    RParen,
    #[token(")")]
    LParen,
    #[token("{")]
    RBrace,
    #[token("}")]
    LBrace,
    #[token("[")]
    RSquare,
    #[token("]")]
    LSquare,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[regex("\\s*", |lex| lex.slice().to_string())]
    Whitespace(String),

    // Multi-char tokens
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    Integer(i32),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f32>().unwrap())]
    Float(f32),
    #[regex(r"'[^']*'", |lex| {
        let s = lex.slice();
        s[1..s.len() - 1].to_string()
    })]
    Char(String),
    #[regex("\"(?:\\.|[^\"])*\"", |lex| {
        let s = lex.slice();
        s[1..s.len() - 1].to_string()
    })]
    String(String),

    // Keywords
    #[token("import")]
    Import,
    #[token("from")]
    From,
    #[regex("true|false", |lex| lex.slice().parse::<bool>().unwrap())]
    Boolean(bool),
    #[token("null")]
    Null,
    #[token("undefined")]
    Undefined,

    #[regex("function", |lex| lex.slice().to_string())]
    ReservedKeyword(String),

    // Others
    #[error]
    #[regex("//.*", logos::skip)]
    Error,

    #[regex("[0-9]+[a-zA-Z_]")]
    InvalidNumberAlpha,

    EOF,
}

impl Tokens {
    pub fn is_value(&self) -> bool {
        match self {
            Tokens::Integer(_)
            | Tokens::Float(_)
            | Tokens::String(_)
            | Tokens::Identifier(_)
            | Tokens::Boolean(_)
            | Tokens::Char(_)
            | Tokens::Null
            | Tokens::Undefined => true,
            _ => false,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            Tokens::Whitespace(..) => true,
            _ => false,
        }
    }

    pub fn get_whitespace(&self) -> &str {
        match self {
            Tokens::Whitespace(s) => s,
            _ => panic!("Not whitespace"),
        }
    }
}

pub type Token = Spanned<Tokens>;
