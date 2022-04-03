use std::iter::Peekable;
use std::process::exit;

use logos::{Logos, Span, SpannedIter};

use crate::parser::{
    ast::{
        ast::{
            Node::{self, *},
            TopLevel,
        },
        lexer::Tokens,
    },
    error::{CompileError, CompileErrorBuilder, ErrorCodes},
};

use super::ast::{Function, NodeType, Op};

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
        let mut top_level = TopLevel { fns: Vec::new() };

        while let Some(token) = self.next() {
            match token {
                Tokens::Semicolon => {}
                _ => {
                    let stmt = self.parse_statement();

                    match stmt {
                        Fn { .. } => {
                            top_level.fns.push(stmt);
                        }
                        _ => panic!("parse_statement failed"),
                    }
                }
            }
        }

        top_level
    }

    pub fn error(&self, message: &str) -> ! {
        let m = message.to_string();
        let mut e = CompileErrorBuilder::new().code(0).message(m).build();

        self.emit_error(&mut e);
    }

    fn emit_error(&self, err: &mut CompileError) -> ! {
        let err = err.with_raw(self.raw.to_string()).set_pos(self.pos);

        eprintln!("{}", err);
        exit(1);
    }

    fn unknown_token(&self, where_: &str) -> ! {
        let mut e = CompileErrorBuilder::new()
            .from(ErrorCodes::UnexpectedToken)
            .note(format!("error occurred at {}", where_).as_str())
            .build();

        self.emit_error(&mut e);
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
                    self.unknown_token("lexer");
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
        match self.next() {
            Some(t) => t,
            None => {
                let mut e = CompileErrorBuilder::new()
                    .from(ErrorCodes::UnexpectedEOF)
                    .build();

                self.emit_error(&mut e);
            }
        }
    }

    #[inline(always)]
    fn token(&self) -> Tokens {
        self.current_token.0.clone()
    }

    #[inline(always)]
    fn span(&self) -> Span {
        self.current_token.1.clone()
    }

    #[inline(always)]
    fn slice(&self) -> std::string::String {
        let span = self.span();

        self.raw[span.start..span.end].to_string()
    }

    #[inline]
    fn peek(&mut self) -> Option<&(Tokens, Span)> {
        self.lexer.peek()
    }

    // Node parsers

    fn parse_statement(&mut self) -> Node {
        match self.current_token.0 {
            Tokens::Fn => self.parse_fn(),
            _ => {
                let mut e = CompileErrorBuilder::new()
                    .from(ErrorCodes::UnexpectedEOF)
                    .note("expected `fn` at top level")
                    .build();

                self.emit_error(&mut e)
            }
        }
    }

    fn parse_fn(&mut self) -> Node {
        let name = self.resolve_ident();
        let args = self.parse_fn_args();
        let ret = self.parse_fn_ret();
        let body = self.parse_block();

        Node::Fn(Box::new(Function {
            name,
            args,
            body,
            ret,
        }))
    }

    fn parse_fn_args(&mut self) -> Vec<(std::string::String, NodeType)> {
        let mut args = Vec::new();

        self.ensure(Tokens::LParen);

        loop {
            match self.next_force() {
                Tokens::RParen => break,
                Tokens::Identifier(_) => {
                    args.push((self.slice(), self.resolve_type()));
                }
                _ => self.unknown_token("function arguments"),
            }

            match self.next_force() {
                Tokens::Comma => {}
                Tokens::RParen => break,
                _ => self.unknown_token("function arguments"),
            }
        }

        args
    }

    fn parse_fn_ret(&mut self) -> NodeType {
        self.ensure(Tokens::RArrow);

        self.resolve_type()
    }

    fn parse_call_args(&mut self) -> Vec<Node> {
        let mut args = Vec::new();

        loop {
            match self.next_force() {
                Tokens::RParen => break,
                _ => {
                    args.push(self.parse_value());
                }
            }

            match self.next_force() {
                Tokens::Comma => {}
                Tokens::RParen => break,
                _ => self.unknown_token("call arguments"),
            }
        }

        args
    }

    fn parse_block(&mut self) -> Node {
        let mut body = Vec::new();

        self.ensure(Tokens::LBrace);

        loop {
            match self.next_force() {
                Tokens::RBrace => break,
                Tokens::Semicolon => {}
                _ => {
                    body.push(self.parse_stmt_or_expr());
                }
            }
        }

        Block(body)
    }

    fn parse_stmt_or_expr(&mut self) -> Node {
        match self.token() {
            Tokens::Fn => self.parse_fn(),
            Tokens::LBrace => self.parse_block(),
            Tokens::Identifier(_)
            | Tokens::Integer(_)
            | Tokens::Boolean(_)
            | Tokens::Float(_)
            | Tokens::Null => self.parse_expr(),
            _ => self.unknown_token("statement or expression"),
        }
    }

    fn parse_expr(&mut self) -> Node {
        let lhs = self.parse_value();
        match self.next_force() {
            Tokens::Semicolon => lhs,
            a => {
                dbg!(a);
                let op = self.resolve_op();
                let rhs = match self.peek().unwrap().0 {
                    Tokens::Semicolon => None,
                    _ => {
                        self.next_force();
                        Some(Box::new(self.parse_value()))
                    }
                };

                Node::Expr {
                    lhs: Box::new(lhs),
                    op,
                    rhs,
                }
            }
        }
    }

    fn parse_value(&mut self) -> Node {
        match self.token() {
            Tokens::Identifier(val) => {
                let id = self.slice();

                match self.peek().unwrap().0 {
                    Tokens::LParen => {
                        self.next_force();
                        let args = self.parse_call_args();
                        Call { name: id, args }
                    }
                    _ => Identifier(val),
                }
            }
            _ => unimplemented!(),
        }
    }

    // Guards

    fn ensure(&mut self, token: Tokens) -> Tokens {
        if self.next_force() != token {
            let mut err = CompileErrorBuilder::new()
                .from(ErrorCodes::UnexpectedToken)
                .note(format!("expected `{:?}`", token).as_str())
                .build();

            self.emit_error(&mut err);
        }

        self.token()
    }

    // Resolvers

    fn resolve_ident(&mut self) -> std::string::String {
        match self.next_force() {
            Tokens::Identifier(_) => self.slice(),
            _ => {
                let mut err = CompileErrorBuilder::new()
                    .from(ErrorCodes::UnexpectedToken)
                    .note("expected identifier after function name")
                    .build();

                self.emit_error(&mut err);
            }
        }
    }

    fn resolve_type(&mut self) -> NodeType {
        self.ensure(Tokens::Colon);

        match self.next_force() {
            Tokens::Identifier(_) => {
                let id = self.slice();

                match id.as_str() {
                    "int" => NodeType::Integer,
                    "float" => NodeType::Float,
                    "char" => NodeType::Char,
                    "str" => NodeType::String,
                    _ => {
                        let mut err = CompileErrorBuilder::new()
                            .from(ErrorCodes::UnexpectedToken)
                            .note("expected type")
                            .build();

                        self.emit_error(&mut err);
                    }
                }
            }
            _ => {
                let mut err = CompileErrorBuilder::new()
                    .from(ErrorCodes::UnexpectedToken)
                    .note("expected type")
                    .build();

                self.emit_error(&mut err);
            }
        }
    }

    fn resolve_op(&mut self) -> Op {
        match self.token() {
            Tokens::Plus => Op::Add,
            Tokens::Minus => Op::Sub,
            Tokens::Star => Op::Mul,
            Tokens::Slash => Op::Div,
            Tokens::Modulo => Op::Mod,
            Tokens::Equals => Op::Eq,
            Tokens::NotEqual => Op::Neq,
            Tokens::LessThan => Op::Lt,
            Tokens::GreaterThan => Op::Gt,
            Tokens::LessThanEqual => Op::Le,
            Tokens::GreaterThanEqual => Op::Ge,
            _ => {
                let mut err = CompileErrorBuilder::new()
                    .from(ErrorCodes::UnexpectedToken)
                    .note("expected operator")
                    .build();

                self.emit_error(&mut err);
            }
        }
    }
}
