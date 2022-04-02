use logos::{Logos, Span};

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
    #[token("%")]
    Modulo,
    #[token("^")]
    Caret,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[token("!=")]
    NotEqual,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<=")]
    LessThanEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("/")]
    Slash,
    #[token("|>")]
    Pipe,
    #[token("->")]
    RArrow,

    // Multi-char tokens
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Slice::from(lex.span()))]
    Identifier(Slice),
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    Integer(i32),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f32>().unwrap())]
    Float(f32),
    #[regex(r"'[^']*'", |lex| lex.slice().parse::<char>().unwrap())]
    Char(char),
    #[regex("\"(?:\\.|[^\"])*\"", |lex| Slice::from(lex.span()))]
    String(Slice),

    // Keywords
    #[regex("true|false", |lex| lex.slice().parse::<bool>().unwrap())]
    Boolean(bool),
    #[token("null")]
    Null,
    #[token("fn")]
    Fn,

    // Position tally
    #[regex("(\r\n|\r|\n)")]
    Newline,
    #[regex(r"[ \t\f]+")]
    Whitespace,
    #[regex("(?:#|//).*")]
    Comment,

    // Others
    #[error]
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Slice {
    pub start: u32,
    pub end: u32,
}

impl Slice {
    pub fn trim(self) -> Self {
        Self {
            start: self.start + 1,
            end: self.end - 1,
        }
    }

    pub fn from(span: Span) -> Self {
        Self {
            start: span.start as _,
            end: span.end as _,
        }
    }
}
