use std::process::exit;
use std::{fmt::Display, iter::Peekable};

use logos::{Logos, Span, SpannedIter};

use crate::{
    ast::{
        ImportMember,
        Node::{self, *},
        TopLevel,
    },
    debug,
    lexer::{Slice, Tokens},
};

macro_rules! die {
    () => {
        panic!("unexpected error");
    };

    ($where:expr) => {
        panic!("unexpected error at {}", $where);
    };

    ($where:expr, $msg:tt) => {
        panic!("{:?}: {}", $where, $msg)
    };

    ($where:expr, $msg:tt, $($arg:expr),+) => {
        panic!("{:?}: {}", $where, format!($msg, $($arg),*))
    };
}

pub trait Parse {
    fn parse(self) -> TopLevel;
}

impl Parse for std::string::String {
    fn parse(self) -> TopLevel {
        Parser::new(&self).parse()
    }
}

impl Parse for &str {
    fn parse(self) -> TopLevel {
        Parser::new(self).parse()
    }
}

pub struct Parser<'a> {
    raw: &'a str,
    lexer: Peekable<SpannedIter<'a, Tokens>>,
    current_token: (Tokens, Span),
    pos: (usize, usize),
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            raw,
            lexer: Tokens::lexer(raw).spanned().peekable(),
            current_token: (Tokens::Error, 0..0),
            pos: (0, 0),
        }
    }

    pub fn parse(mut self) -> TopLevel {
        let mut top_level = TopLevel {
            consts: Vec::new(),
            imports: Vec::new(),
        };

        while let Some(token) = self.next() {
            match token {
                Tokens::Semicolon => {}
                _ => match self.parse_statement() {
                    Import { path, members } => {
                        top_level.imports.push(Import { path, members });
                    }
                    ConstDeclaration { name, value } => {
                        top_level.consts.push(ConstDeclaration { name, value })
                    }
                    _ => panic!("invalid statement returned"),
                },
            }
        }

        top_level
    }

    pub fn error(&self, message: impl Display) -> ! {
        eprintln!(
            "Error at line {}, column {}: {}",
            self.pos.0,
            self.pos.1 - self.span().len(),
            message
        );
        exit(1)
    }

    // Lexer helpers

    #[inline]
    fn next(&mut self) -> Option<Tokens> {
        if let Some(next) = self.lexer.next() {
            self.pos.1 += next.1.len();

            match next.0 {
                Tokens::Whitespace => self.next(),
                Tokens::Newline => {
                    self.pos.0 += 1;
                    self.pos.1 = 0;
                    self.next()
                }
                Tokens::Error => {
                    self.error("unknown token");
                }
                _ => {
                    self.current_token = next;
                    Some(self.token())
                }
            }
        } else {
            None
        }
    }

    #[inline]
    fn next_force(&mut self) -> Tokens {
        self.next().expect("unexpected eof")
    }

    #[inline(always)]
    fn token(&self) -> Tokens {
        self.current_token.0.clone()
    }

    #[inline(always)]
    fn span(&self) -> Span {
        self.current_token.1.clone()
    }

    #[inline]
    fn peek(&mut self) -> Option<&(Tokens, Span)> {
        self.lexer.peek()
    }

    // Node parsers

    fn parse_statement(&mut self) -> Node {
        match self.current_token.0 {
            Tokens::Import => {
                let members = match self.next_force() {
                    Tokens::Identifier(name) => ImportMember::All(name),
                    Tokens::LParen => {
                        let mut members = Vec::new();

                        loop {
                            match self.next_force() {
                                Tokens::Identifier(slice) => {
                                    members.push(Identifier(slice));

                                    match self.next_force() {
                                        Tokens::Comma => {}
                                        Tokens::RParen => {
                                            break;
                                        }
                                        _ => self.error(
                                            "unexpected token after import member identifier",
                                        ),
                                    }
                                }
                                Tokens::RParen => {
                                    break;
                                }
                                _ => self.error("unexpected token in import member list"),
                            }
                        }

                        ImportMember::Named(members)
                    }
                    Tokens::Star => ImportMember::AllDestructured,
                    token => self.error(format!(
                        "unexpected token {:?} after {:?}",
                        token, self.current_token.0
                    )),
                };

                if let Tokens::From = self.next_force() {
                    Import {
                        path: self.resolve_import_path(),
                        members,
                    }
                } else {
                    self.error("eof after import member list")
                }
            }
            Tokens::Const => {
                let decl = ConstDeclaration {
                    name: self.resolve_const_name(),
                    value: Box::new(self.parse_value()),
                };
                self.ensure_semicolon();
                decl
            }
            _ => self.error("unexpected token"),
        }
    }

    pub fn parse_value(&mut self) -> Node {
        match self.next_force() {
            Tokens::String(slice) => Node::String(slice.trim()),
            Tokens::Char(c) => Char(c),
            Tokens::Integer(i) => Integer(i),
            Tokens::Float(f) => Float(f),
            Tokens::Identifier(id) => Identifier(id),
            Tokens::Atom(slice) => Atom(slice),

            _ => panic!("unknown value"),
        }
    }

    // Guards

    fn ensure_semicolon(&mut self) {
        if self.next_force() != Tokens::Semicolon {
            self.error("unexpected token")
        }
    }

    // Resolvers

    fn resolve_import_path(&mut self) -> Slice {
        if let Tokens::String(path) = self.next_force() {
            self.ensure_semicolon();
            path.trim()
        } else {
            self.error("unexpected token")
        }
    }

    fn resolve_const_name(&mut self) -> Slice {
        if let Tokens::Identifier(name) = self.next_force() {
            if self.next_force() != Tokens::Equals {
                self.error("unexpected token")
            }

            name
        } else {
            self.error("unexpected token")
        }
    }
}
