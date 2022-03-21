use crate::lexer::Slice;

#[derive(Debug)]
pub struct TopLevel {
    pub imports: Vec<Node>,
    pub consts: Vec<Node>,
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

    Identifier(Slice),
    Atom(Slice),

    Import { path: Slice, members: ImportMember },
    ConstDeclaration { name: Slice, value: Box<Node> },
}
