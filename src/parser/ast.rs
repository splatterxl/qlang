#![allow(dead_code)]

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
