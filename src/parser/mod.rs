use crate::{
    error::{errors::Errors, format_errs, Error},
    lexer::{Lexer, Token, Tokens},
};

mod ast;

use ast::{Node, Operator};

pub struct Parser {
    lexed: Vec<Token>,
    ast: Vec<Node>,
}

impl Parser {
    pub fn categorise_value(val: Token) -> Node {
        match val.data {
            Tokens::Integer(value) => Node::Integer { value },
            Tokens::Float(value) => Node::Float { value },
            Tokens::String(value) => Node::String { value },
            Tokens::Identifier(value) => Node::Identifier { value },
            Tokens::Boolean(value) => Node::Boolean { value },
            Tokens::Char(value) => Node::Character {
                value: value.chars().next().unwrap(),
            },
            Tokens::Null => Node::Null,
            Tokens::Undefined => Node::Undefined,
            _ => panic!("Unrecognised value: {:?}", val.data),
        }
    }

    pub fn operator(token: Tokens) -> Operator {
        match token {
            Tokens::Plus => Operator::Add,
            Tokens::Minus => Operator::Sub,
            Tokens::Star => Operator::Mul,
            _ => panic!("invalid operator {:?}", token),
        }
    }

    pub fn parse(raw: String) -> Vec<Node> {
        let lexed = Lexer::new(raw.clone()).vec();

        dbg!(&lexed);

        let mut parser = Parser { lexed, ast: vec![] };

        let syntax_errors = parser.parse_program();

        if syntax_errors.is_empty() {
            parser.ast
        } else {
            dbg!(parser.ast);
            format_errs(raw, syntax_errors);
            std::process::exit(1);
        }
    }

    pub fn parse_program(&mut self) -> Vec<Error> {
        let mut i = 0;

        let tokens = &self.lexed;
        let mut ast = Vec::new();

        let mut syntax_errors = Vec::new();
        // TODO: syntax errors

        while i < tokens.len() {
            let token = tokens[i].clone();

            match &token.data {
                Tokens::From => {
                    if i == tokens.len() - 1 {
                        println!("error encountered ({})", line!());
                        syntax_errors
                            .push(Errors::unexpected_end_of_input(token.line, token.column));
                    } else {
                        i += 1;
                        let next = tokens[i].clone();

                        let library = match next.data {
                            Tokens::String(library) => library,
                            _ => {
                                println!("error encountered ({})", line!());
                                syntax_errors.push(Errors::import_no_lib(token.line, token.column));
                                break;
                            }
                        };

                        i += 1;
                        let next = tokens[i].clone();

                        match next.data {
                            Tokens::Import => {}
                            Tokens::EOF => {
                                println!("error encountered ({})", line!());
                                syntax_errors.push(Errors::unexpected_end_of_input(
                                    token.line,
                                    token.column,
                                ));
                                break;
                            }
                            _ => {
                                println!("error encountered ({})", line!());
                                syntax_errors.push(Errors::unexpected_token(
                                    "'import'",
                                    next.data,
                                    next.line,
                                    next.column,
                                ));
                                break;
                            }
                        }

                        i += 1;
                        let next = tokens[i].clone();

                        match next.data {
                            Tokens::RParen | Tokens::RBrace => {
                                println!("error encountered ({})", line!());
                                syntax_errors
                                    .push(Errors::curly_bracket_import(next.line, next.column));
                                let errs = syntax_errors.len();

                                let mut properties = Vec::new();
                                loop {
                                    i += 1;
                                    let next = tokens[i].clone();

                                    match next.data {
                                        Tokens::LParen | Tokens::LBrace => break,
                                        Tokens::EOF => {
                                            println!("error encountered ({})", line!());
                                            syntax_errors.push(Errors::unexpected_end_of_input(
                                                token.line,
                                                token.column,
                                            ));
                                            break;
                                        }
                                        Tokens::Identifier(property) => {
                                            properties.push(Node::Identifier { value: property });
                                        }
                                        Tokens::Comma => {}
                                        _ => {
                                            println!("error encountered ({})", line!());
                                            syntax_errors.push(Errors::unexpected_token(
                                                "')' or an identifier",
                                                next.data,
                                                next.line,
                                                next.column,
                                            ));
                                            break;
                                        }
                                    }
                                }

                                if syntax_errors.len() > errs {
                                    break;
                                }

                                ast.push(Node::Import {
                                    from: library,
                                    properties,
                                });
                            }
                            Tokens::EOF => {
                                println!("error encountered ({})", line!());
                                syntax_errors.push(Errors::unexpected_end_of_input(
                                    token.line,
                                    token.column,
                                ));
                                break;
                            }
                            tok => {
                                println!("error encountered ({})", line!());
                                syntax_errors.push(Errors::unexpected_token(
                                    "'('",
                                    tok,
                                    next.line,
                                    next.column,
                                ));
                                break;
                            }
                        };
                    }
                }

                Tokens::Import => {
                    println!("error encountered ({})", line!());
                    syntax_errors.push(Errors::import_out_of_statement(token.line, 0));
                    break;
                }

                Tokens::RBrace | Tokens::LBrace | Tokens::RSquare | Tokens::LSquare => {
                    println!("error encountered ({})", line!());
                    syntax_errors.push(Errors::unimplemented_feature(token.line, token.column));
                    break;
                }

                Tokens::Semicolon => {}
                Tokens::EOF => break,

                Tokens::InvalidNumberAlpha => {
                    println!("error encountered ({})", line!());
                    syntax_errors.push(Errors::invalid_number(token.line, token.column, token.end));
                }

                val => {
                    let is_value = val.is_value();

                    i += 1;
                    let next = tokens[i].clone();

                    match next.data {
                        Tokens::Semicolon => {
                            if is_value {
                                ast.push(Parser::categorise_value(token));
                            } else {
                                println!("error encountered ({})", line!());
                                syntax_errors.push(Errors::unexpected_token(
                                    "value",
                                    token.data,
                                    token.line,
                                    token.column,
                                ));
                                continue;
                            }
                        }
                        Tokens::EOF => {
                            println!("error encountered ({})", line!());
                            syntax_errors
                                .push(Errors::unexpected_end_of_input(token.line, token.column));
                            break;
                        }
                        Tokens::Plus | Tokens::Minus | Tokens::Star => {
                            if is_value {
                                i += 1;
                                let rhs = tokens[i].clone();

                                let right = match rhs.data {
                                    Tokens::Integer(value) => Node::Integer { value },
                                    Tokens::Float(value) => Node::Float { value },
                                    Tokens::String(value) => Node::String { value },
                                    Tokens::Identifier(value) => Node::Identifier { value },
                                    Tokens::Boolean(value) => Node::Boolean { value },
                                    Tokens::Char(value) => Node::Character {
                                        value: value.chars().next().unwrap(),
                                    },
                                    Tokens::Null => Node::Null,
                                    Tokens::Undefined => Node::Undefined,
                                    tok => {
                                        println!("error encountered ({})", line!());
                                        syntax_errors.push(Errors::unexpected_token(
                                            "value",
                                            tok,
                                            token.line,
                                            token.column,
                                        ));
                                        break;
                                    }
                                };

                                ast.push(Node::Expression {
                                    left: Box::new(Parser::categorise_value(token)),
                                    right: Box::new(right),
                                    operator: Parser::operator(next.data),
                                });
                            }
                        }
                        tok => {
                            println!("error encountered ({})", line!());
                            syntax_errors.push(Errors::expected_semicolon(
                                tok,
                                next.line,
                                next.column,
                            ));
                            break;
                        }
                    }
                }
            }

            i += 1;
        }

        self.ast = ast;
        syntax_errors
    }
}
