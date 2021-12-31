use logos::{Logos, Span};

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
        let mut line = 0usize;
        let mut column = 0;
        let mut spanned = 0;

        let _ = column;

        while let Some(token) = lexer.next() {
            spanned += lexer.span().end - spanned;

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
                } else {
                    column += spanned - lexer.span().start;
                }

                continue;
            } else {
                column = spanned - lexer.span().start;
            }

            vec.push(Token {
                token,
                span: lexer.span(),
                lc: (line, column),
            });
        }

        vec.push(Token {
            token: Tokens::EOF,
            span: 0..0,
            lc: (0, 0),
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: Tokens,
    pub span: Span,
    pub lc: (usize, usize),
}
