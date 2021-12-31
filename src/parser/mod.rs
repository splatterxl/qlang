use crate::{
    errors::{format_errs, Error},
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
        match val.token {
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
            _ => panic!("Unrecognised value: {:?}", val.token),
        }
    }

    pub fn operator(token: Token) -> Operator {
        match token.token {
            Tokens::Plus => Operator::Add,
            Tokens::Minus => Operator::Sub,
            Tokens::Star => Operator::Mul,
            _ => panic!("invalid operator {:?}", token.token),
        }
    }

    pub fn parse(raw: String) -> Vec<Node> {
        let lexed = Lexer::new(raw.clone()).vec();

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

            match &token.token {
                Tokens::From => {
                    if i == tokens.len() - 1 {
                        syntax_errors.push(Error::Syntax {
                            lc: token.lc,
                            code: line!(),
                            hints: vec![],
                            message: "Unexpected end of input".to_string(),
                            span: token.span,
                        });
                    } else {
                        i += 1;
                        let next = tokens[i].clone();

                        let library = match next.token {
                            Tokens::String(library) => library,
                            _ => {
                                syntax_errors.push(Error::Syntax {
                                    lc: next.lc,
                                    code: line!(),
                                    hints: vec![
                                        "import statements must be followed by a string literal"
                                            .to_string(),
                                        "did you forget to add the library name?".to_string(),
                                    ],
                                    message: "Expected string literal".to_string(),
                                    span: next.span,
                                });
                                break;
                            }
                        };

                        i += 1;
                        let next = tokens[i].clone();

                        if next.token != Tokens::Import {
                            syntax_errors.push(Error::Syntax {
                                lc: next.lc,
                                code: line!(),
                                hints: vec![],
                                message: "Expected import keyword".to_string(),
                                span: next.span,
                            });
                            break;
                        }

                        i += 1;
                        let next = tokens[i].clone();

                        match next.token {
                            Tokens::RParen => {
                                let mut properties = Vec::new();
                                loop {
                                    i += 1;
                                    let next = tokens[i].clone();

                                    match next.token {
                                        Tokens::LParen => break,
                                        Tokens::EOF => {
                                            syntax_errors.push(Error::Syntax {
                                                lc: next.lc,
                                                code: line!(),
                                                hints: vec![],
                                                message: "Unexpected end of file".to_string(),
                                                span: next.span,
                                            });
                                            break;
                                        }
                                        Tokens::Identifier(property) => {
                                            properties.push(Node::Identifier { value: property });
                                        }
                                        Tokens::Comma => {}
                                        _ => {
                                            syntax_errors.push(Error::Syntax {
                                                lc: next.lc,
                                                code: line!(),
                                                hints: vec![],
                                                message: "Expected Identifier, Comma or LParen"
                                                    .to_string(),
                                                span: next.span,
                                            });
                                            break;
                                        }
                                    }
                                }

                                ast.push(Node::Import {
                                    from: library,
                                    properties,
                                });
                            }
                            _ => {
                                syntax_errors.push(Error::Syntax {
                                    lc: next.lc,
                                    code: line!(),
                                    hints: vec![],
                                    message: "Expected RParen".to_string(),
                                    span: next.span,
                                });
                                break;
                            }
                        };
                    }
                }
                Tokens::Semicolon => {}
                Tokens::EOF => break,

                Tokens::InvalidNumberAlpha => {
                    syntax_errors.push(Error::Syntax {
                        lc: token.lc,
                        code: line!(),
                        hints: vec![
                            "number literals can't have alphabetical characters in them!"
                                .to_string(),
                        ],
                        message: "Invalid number".to_string(),
                        span: token.span,
                    });
                }

                val => {
                    let is_value = val.is_value();

                    i += 1;
                    let next = tokens[i].clone();

                    match &next.token {
                        Tokens::Semicolon => {
                            if is_value {
                                ast.push(Parser::categorise_value(token));
                            } else {
                                syntax_errors.push(Error::Syntax {
                                    lc: token.lc,
                                    code: line!(),
                                    hints: vec![],
                                    message: format!(
                                        "{:?} is not valid in this context",
                                        token.token
                                    ),
                                    span: token.span,
                                });
                                continue;
                            }
                        }
                        Tokens::Plus | Tokens::Minus | Tokens::Star => {
                            if is_value {
                                i += 1;
                                let rhs = tokens[i].clone();

                                let right = match rhs.token {
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
                                    _ => {
                                        syntax_errors.push(Error::Syntax {
                                            lc: rhs.lc,
                                            code: line!(),
                                            hints: vec![],
                                            message: "Expected value".to_string(),
                                            span: rhs.span,
                                        });
                                        break;
                                    }
                                };

                                ast.push(Node::Expression {
                                    left: Box::new(Parser::categorise_value(token)),
                                    right: Box::new(right),
                                    operator: Parser::operator(next),
                                });
                            }
                        }
                        _ => {
                            syntax_errors.push(Error::Syntax {
                                lc: token.lc,
                                code: line!(),
                                hints: vec![],
                                message: "Expected semicolon".to_string(),
                                span: token.span,
                            });
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
