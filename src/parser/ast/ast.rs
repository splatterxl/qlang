use crate::parser::ast::lexer::Slice;

#[derive(Debug)]
pub struct TopLevel {
    pub fns: Vec<Node>
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

    Fn {
        name: String,
        args: Vec<(String, NodeType)>,
        /// Node::Block, to access use body.0
        body: Box<Node>,
        ret: NodeType,
    },
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
