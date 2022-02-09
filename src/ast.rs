use crate::lexer::Slice;

#[derive(Debug)]
pub struct TopLevel {
    pub imports: Vec<Expression>,
    pub consts: Vec<Expression>
}

#[derive(Debug)]
pub enum ImportMember {
    All(Slice),
    AllDestructured,
    Named(Vec<Value>)
}

#[derive(Debug)]
pub enum Value {
    String(Slice),
    Integer(i32),
    Float(f32),
    Char(char),

    Identifier(Slice),
    Atom(Slice),
}

impl Value {
    pub fn into_node(self) -> Node {
        Node::Value(self)
    }
}

#[derive(Debug)]
pub enum Expression {
  Import {
      path: Slice,
      members: ImportMember,
  },

  ConstDeclaration {
      name: Value,
      value: Box<Node>
  }
}

#[derive(Debug)]
pub enum Node {
    Expr(Expression),
    Value(Value),
    FunctionCall {
        func: Value,
        args: Vec<Box<Node>>,
    }
}
