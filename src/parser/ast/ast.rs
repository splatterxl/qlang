use crate::parser::ast::lexer::Slice;

#[derive(Debug)]
pub struct TopLevel {
    pub fns: Vec<Node>,
}

#[derive(Debug)]
pub enum ImportMember {
    All(Slice),
    AllDestructured,
    Named(Vec<Node>),
}

#[derive(Debug)]
pub enum Node {
    String(Slice),
    Integer(i32),
    Float(f32),
    Char(char),
    Bool(bool),

    Identifier(Slice),

    Block(Vec<Node>),
    Expr {
        lhs: Box<Node>,
        op: Op,
        rhs: Option<Box<Node>>,
    },
    Stmt(Box<Node>),

    Fn(Box<Function>),
    Call {
        name: String,
        args: Vec<Node>,
    },
}

#[derive(Debug)]
pub enum Op {
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Assign,
}

#[derive(Debug)]
pub enum NodeType {
    String,
    Integer,
    Float,
    Char,
    Bool,
    Void,
    Fn {
        args: Vec<NodeType>,
        ret: Box<NodeType>,
    },
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, NodeType)>,
    pub body: Node,
    pub ret: NodeType,
}
