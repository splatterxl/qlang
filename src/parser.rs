use crate::lexer::{Lexer, Token, Tokens};

use logos::Span;

#[derive(Debug, PartialEq)]
pub enum Node {
    Expression {
        left: Box<Node>,
        right: Box<Node>,
        operator: Operator,
    },
    Integer {
        value: i32,
    },
    Float {
        value: f32,
    },
    Character {
        value: char,
    },
    String {
        value: String,
    },
    Identifier {
        value: String,
    },
    Boolean {
        value: bool,
    },
    Null,
    Undefined,
    Array {
        elements: Vec<Node>,
    },
    Function {
        params: Vec<Node>,
        body: Box<Node>,
    },
    Return(Box<Node>),
    Call {
        callee: Box<Node>,
        args: Vec<Node>,
    },
    Block {
        statements: Vec<Node>,
    },
    Break,
    Continue,
    VariableDeclaration {
        kind: VariableDeclarationKind,
        declarations: Vec<Node>,
    },
    Import {
        from: String,
        properties: Vec<Node>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationKind {
    Let,
    Const,
}

pub struct Parser {
    lexed: Vec<Token>,
}

impl Parser {
    pub fn categorise_value(val: Token, iter: usize) -> Node {
        match val.token {
            Tokens::Integer(value) => Node::Integer { value },
            Tokens::Float(value) => Node::Float { value },
            Tokens::String(value) => Node::String { value },
            Tokens::Identifier(value) => Node::Identifier { value },
            Tokens::Boolean(value) => Node::Boolean { value },
            Tokens::Char(value) => {
                if value.len() > 1 {
                    panic!("Invalid character literal: {}", value);
                } else {
                    Node::Character {
                        value: value.chars().next().unwrap(),
                    }
                }
            }
            Tokens::Null => Node::Null,
            Tokens::Undefined => Node::Undefined,
            _ => panic!("Unrecognised token: {:?}, iteration {}", val.token, iter),
        }
    }

    pub fn parse(raw: String) -> Vec<Node> {
        let lexed = Lexer::new(raw.clone()).vec();

        let mut parser = Parser { lexed };

        parser.parse_program()
    }

    pub fn parse_program(&mut self) -> Vec<Node> {
        let tokens = &self.lexed;
        let mut ast = Vec::new();

        let mut i = 0;

        while i < tokens.len() {
            let token = tokens[i].clone();

            match &token.token {
                Tokens::From => {
                    if i == tokens.len() - 1 {
                        panic!("Unexpected end of file");
                    } else {
                        i += 1;
                        let next = tokens[i].clone();

                        let library = match next.token {
                            Tokens::String(library) => library,
                            _ => panic!("Expected string literal"),
                        };

                        i += 1;
                        let next = tokens[i].clone();

                        if next.token != Tokens::Import {
                            panic!("Expected keyword 'import'");
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
                                        Tokens::EOF => panic!("Unexpected end of file"),
                                        Tokens::Identifier(property) => {
                                            properties.push(Node::Identifier { value: property });
                                        }
                                        Tokens::Comma => {}
                                        _ => {
                                            panic!("Expected identifier or LParen");
                                        }
                                    }
                                }

                                ast.push(Node::Import {
                                    from: library,
                                    properties,
                                });
                            }
                            _ => panic!("Expected '{:?}'", Tokens::RParen),
                        };
                    }
                }
                Tokens::Semicolon => {}
                Tokens::EOF => break,
                val => {
                    let is_value = val.is_value();

                    i += 1;
                    let next = tokens[i].clone();

                    match next.token {
                        Tokens::Semicolon => {
                            if is_value {
                                ast.push(Parser::categorise_value(token, i));
                            } else {
                                panic!("{:?} is not valid in this context", val);
                            }
                        }
                        Tokens::Plus => {
                            if is_value {
                                i += 1;
                                let next = tokens[i].clone();

                                let right = match next.token {
                                    Tokens::Integer(value) => Node::Integer { value },
                                    Tokens::Float(value) => Node::Float { value },
                                    Tokens::String(value) => Node::String { value },
                                    Tokens::Identifier(value) => Node::Identifier { value },
                                    Tokens::Boolean(value) => Node::Boolean { value },
                                    Tokens::Char(value) => {
                                        if value.len() > 1 {
                                            panic!("Invalid character literal: {}", value);
                                        } else {
                                            Node::Character {
                                                value: value.chars().next().unwrap(),
                                            }
                                        }
                                    }
                                    Tokens::Null => Node::Null,
                                    Tokens::Undefined => Node::Undefined,
                                    _ => panic!("Expected value"),
                                };

                                ast.push(Node::Expression {
                                    left: Box::new(Parser::categorise_value(token, i)),
                                    right: Box::new(right),
                                    operator: Operator::Add,
                                });
                            }
                        }
                        Tokens::Minus => {
                            if is_value {
                                i += 1;
                                let next = tokens[i].clone();

                                let right = match next.token {
                                    Tokens::Integer(value) => Node::Integer { value },
                                    Tokens::Float(value) => Node::Float { value },
                                    Tokens::Identifier(value) => Node::Identifier { value },
                                    Tokens::Null => Node::Null,
                                    Tokens::Undefined => Node::Undefined,
                                    _ => panic!("Expected value"),
                                };

                                ast.push(Node::Expression {
                                    left: Box::new(Parser::categorise_value(token, i)),
                                    right: Box::new(right),
                                    operator: Operator::Sub,
                                });
                            }
                        }
                        Tokens::Star => {
                            if is_value {
                                i += 1;
                                let next = tokens[i].clone();

                                let right = match next.token {
                                    Tokens::Integer(value) => Node::Integer { value },
                                    Tokens::Float(value) => Node::Float { value },
                                    Tokens::Identifier(value) => Node::Identifier { value },
                                    Tokens::Null => Node::Null,
                                    Tokens::Undefined => Node::Undefined,
                                    _ => panic!("Expected value"),
                                };

                                ast.push(Node::Expression {
                                    left: Box::new(Parser::categorise_value(token, i)),
                                    right: Box::new(right),
                                    operator: Operator::Mul,
                                });
                            }
                        }

                        _ => {
                            panic!("Expected {:?}", Tokens::Semicolon);
                        }
                    }
                }
            }

            i += 1;
        }

        ast
    }
}
