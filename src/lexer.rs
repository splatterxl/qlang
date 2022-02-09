use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub line: u32,
    pub column: u32,
    pub end: u32,
    pub data: T,
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
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[regex("\\s*", |lex| lex.slice().to_string())]
    Whitespace(String),

    // Multi-char tokens
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Slice(lex.span()))]
    Identifier(Slice),
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    Integer(i32),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f32>().unwrap())]
    Float(f32),
    #[regex(r"'[^']*'", |lex| lex.slice().parse::<char>().unwrap())]
    Char(char),
    #[regex("\"(?:\\.|[^\"])*\"", |lex| Slice(lex.span()))]
    String(Slice),
    #[regex(r":[a-zA-Z_]+", |lex| Slice(lex.span()))]
    Atom(Slice),

    // Keywords
    #[token("import")]
    Import,
    #[token("from")]
    From,
    #[token("choose")]
    Choose,
    #[regex("true|false", |lex| lex.slice().parse::<bool>().unwrap())]
    Boolean(bool),
    #[token("null")]
    Null,
    #[token("undefined")]
    Undefined,
    #[token("const")]
    Const,

    #[regex("function", |lex| lex.slice().to_string())]
    ReservedKeyword(String),

    // Others
    #[error]
    #[regex("//.*", logos::skip)]
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Slice(pub Span);

impl Slice {
    pub fn trim(self) -> Self {
        Self(Span {
            start: self.0.start + 1,
            end: self.0.end - 1,
        })
    }
}
