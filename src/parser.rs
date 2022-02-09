use logos::{Lexer, Logos};

use crate::{ast::{Expression, ImportMember, TopLevel, Value, Node}, lexer::Tokens};

pub struct Parser;

impl Parser {
    pub fn parse(raw: String) -> TopLevel {
        let mut lexer = Tokens::lexer(&raw);
        let mut top_level = TopLevel {
            imports: Vec::new(),
            consts: Vec::new(),
        };

        while let Some(token) = lexer.next() {
            match token {
                Tokens::Semicolon => {}
                token => {
                    match Parser::parse_expression(token, &mut lexer) {
                        Expression::Import { path, members } => {
                            top_level.imports.push(Expression::Import { path, members });
                        }
                        Expression::ConstDeclaration { name, value } => {
                            top_level.consts.push(Expression::ConstDeclaration { name, value })
                        }
                        _ => panic!("invalid expression returned")
                    }
                }
            }
        };

        top_level
    }

    pub fn parse_expression(start_token: Tokens, lexer: &mut Lexer<Tokens>) -> Expression {
        let expr = match start_token {
            Tokens::Import => {
                let members = match lexer.next().expect("import followed by eof") {
                    Tokens::Identifier(name) => {
                        ImportMember::All(name)
                    }
                    Tokens::LParen => {
                        let mut members = Vec::new();

                        while let Some(next) = lexer.next() {
                            match next {
                                Tokens::Identifier(slice) => {
                                    members.push(Value::Identifier(slice));

                                    match lexer.next().expect("unexpected eof") {
                                        Tokens::Comma => {}
                                        Tokens::RParen => {
                                            break;
                                        }
                                        _ => panic!("unexpected token after import member identifier")
                                    }
                                }
                                Tokens::RParen => {
                                    break;
                                }
                                token => panic!("unexpected token in import member list: {:?}", token),
                            }
                        };

                        ImportMember::Named(members)
                    }
                    Tokens::Star => {
                        ImportMember::AllDestructured
                    }
                    token => panic!("unexpected token {:?} after {:?}", token, start_token),
                };

                if let Some(Tokens::From) = lexer.next() {
                    Expression::Import {
                        path: if let Some(Tokens::String(slice)) = lexer.next() {
                                slice.trim()
                        } else { 
                            panic!("unexpected token in import statement after From")
                        },
                        members
                    }
                } else {
                    panic!("eof after import member list")
                }
            }
            Tokens::Const => {
                match lexer.next().expect("unexpected eof") {
                    Tokens::Identifier(name) => {
                        if lexer.next().expect("unexpected eof") != Tokens::Equals {
                            panic!("unexpected identifier after const identifier");
                        }

                        Expression::ConstDeclaration {
                            name: Value::Identifier(name),
                            value: Box::new(Parser::parse_value_or_func_call(lexer.next().expect("unexpected eof"), lexer))
                        }
                    }
                    _ => panic!("unexpected identifier after const declaration")
                }
            }
            start_token => {
                panic!("Unexpected token: {:?}", start_token)
            }
        };


        dbg!(&expr);
        if dbg!(lexer.peekable().peek().expect("unexpected eof")) != &Tokens::Semicolon {
            panic!("Expected semicolon")
        }

        expr
    }

    pub fn parse_value_or_func_call(start_token: Tokens, lexer: &mut Lexer<Tokens>) -> Node {
        match start_token {
            Tokens::Identifier(_) => { 
                match lexer.peekable().peek().expect("unexpected eof") {
                    Tokens::LParen => {
                        Parser::parse_function(start_token, true, lexer)
                    }
                    _ => {
                        Parser::parse_value(start_token, lexer).into_node()
                    }
                }
            }
            _ => {
                Parser::parse_value(start_token, lexer).into_node()
            }
        }
    }

    pub fn parse_value(start_token: Tokens, lexer: &mut Lexer<Tokens>) -> Value {
        match start_token {
            Tokens::String(slice) => Value::String(slice),
            Tokens::Char(c) => Value::Char(c),
            Tokens::Integer(i) => Value::Integer(i),
            Tokens::Float(f) => Value::Float(f),
            Tokens::Identifier(id) => Value::Identifier(id),
            Tokens::Atom(slice) => Value::Atom(slice),

            _ => panic!("unknown value")
        }
    }

    pub fn parse_function(start_token: Tokens, paren_done: bool, lexer: &mut Lexer<Tokens>) -> Node {
        unimplemented!()
    }
}
