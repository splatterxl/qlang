use std::iter::Peekable;

use logos::{Logos, Span, SpannedIter};

use crate::{
    ast::{Expression, ImportMember, Node, TopLevel, Value},
    lexer::Tokens,
};

pub trait Parse {
    fn parse(self) -> TopLevel;
}

impl Parse for String {
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
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            raw,
            lexer: Tokens::lexer(raw).spanned().peekable(),
            current_token: (Tokens::Error, 0..0),
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
                _ => match self.parse_expression() {
                    Expression::Import { path, members } => {
                        top_level.imports.push(Expression::Import { path, members });
                    }
                    Expression::ConstDeclaration { name, value } => top_level
                        .consts
                        .push(Expression::ConstDeclaration { name, value }),
                },
            }
        }

        top_level
    }

    fn next(&mut self) -> Option<Tokens> {
        let next = self.lexer.next();

        match next {
            Some(token) => {
                self.current_token = token;

                Some(self.current_token.0.clone())
            }
            None => None,
        }
    }

    fn next_force(&mut self) -> Tokens {
        self.next().expect("unexpected eof")
    }

    fn token(&self) -> Tokens {
        self.current_token.0.clone()
    }

    fn span(&self) -> Span {
        self.current_token.1.clone()
    }

    fn slice(&self) -> &str {
        &self.raw[self.span()]
    }

    pub fn parse_expression(&mut self) -> Expression {
        match self.current_token.0 {
            Tokens::Import => {
                let members = match self.next_force() {
                    Tokens::Identifier(name) => ImportMember::All(name),
                    Tokens::LParen => {
                        let mut members = Vec::new();

                        loop {
                            match self.next_force() {
                                Tokens::Identifier(slice) => {
                                    members.push(Value::Identifier(slice));

                                    match self.next_force() {
                                        Tokens::Comma => {}
                                        Tokens::RParen => {
                                            break;
                                        }
                                        _ => panic!(
                                            "unexpected token after import member identifier"
                                        ),
                                    }
                                }
                                Tokens::RParen => {
                                    break;
                                }
                                token => {
                                    panic!("unexpected token in import member list: {:?}", token)
                                }
                            }
                        }

                        ImportMember::Named(members)
                    }
                    Tokens::Star => ImportMember::AllDestructured,
                    token => panic!(
                        "unexpected token {:?} after {:?}",
                        token, self.current_token.0
                    ),
                };

                if let Tokens::From = self.next_force() {
                    Expression::Import {
                        path: if let Tokens::String(slice) = self.next_force() {
                            if let Tokens::Semicolon = self.next_force() {
                                slice.trim()
                            } else {
                                panic!("unexpected token after import expr")
                            }
                        } else {
                            panic!("unexpected token in import statement after From")
                        },
                        members,
                    }
                } else {
                    panic!("eof after import member list")
                }
            }
            Tokens::Const => match self.next_force() {
                Tokens::Identifier(name) => {
                    if self.next_force() != Tokens::Equals {
                        panic!("unexpected identifier after const identifier");
                    }

                    let expr = Expression::ConstDeclaration {
                        name: Value::Identifier(name),
                        value: Box::new(self.parse_value()),
                    };

                    if self.next_force() != Tokens::Semicolon {
                        panic!("unexpected identifier")
                    } else {
                        expr
                    }
                }
                _ => panic!("unexpected identifier after const declaration"),
            },
            _ => {
                panic!("unexpected token")
            }
        }
    }

    pub fn parse_value(&mut self) -> Node {
        match self.next_force() {
            Tokens::String(slice) => Value::String(slice).into_node(),
            Tokens::Char(c) => Value::Char(c).into_node(),
            Tokens::Integer(i) => Value::Integer(i).into_node(),
            Tokens::Float(f) => Value::Float(f).into_node(),
            Tokens::Identifier(id) => Value::Identifier(id).into_node(),
            Tokens::Atom(slice) => Value::Atom(slice).into_node(),

            _ => panic!("unknown value"),
        }
    }
}
