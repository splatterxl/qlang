/// Which type of token is currently in focus? 🤔
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    // unknown keyword at beginning of line, this is handled by the parser
    UnknownKeyword(Keyword),

    // functions
    FunctionReference(Function),

    // arguments
    ArgumentBlock,
    Argument,
}

impl Token {
    pub fn is_unknown(&self) -> bool {
        matches!(*self, Token::UnknownKeyword(_))
    }
}

/// A function declaration
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
    pub name: String,
    pub body: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BasicToken {
    pub line: usize,
    /// this *would* just be `char`, but Rust has that as a type already
    pub at_char: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Keyword {
    pub at: BasicToken,
    pub name: String,
}
