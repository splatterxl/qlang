# How Qlang Works

It's actually quite simple. 🤓☝

## Step-by-step

- [Lexing](#lexing)
- [Parsing](#parsing)
- [Compiling](#compiling)

### Lexing

*See [lexer.rs](../src/parser/ast/lexer.rs)*

Qlang uses a process called [Lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis) to turn the input (a file to compile) into *tokens*. This is done with a library called [logos](https://crates.io/crates/logos).

These tokens are all the components of the file: the identifiers, the values and all of the special characters that combine to create statements and expressions.

### Parsing

*See [parser/mod.rs](../src/parser/mod.rs)*

After turning the text into tokens, the tokens are digested by the parser to be converted into an [abstract syntax tree](https://wikipedia.org/wiki/Abstract_syntax_tree). This syntax tree contains all the syntax in the input, simplified so that the computer can handle it by turning it into LLVM bytecode.

### Compiling
*See [llvm/codegen.rs](../src/llvm/codegen.rs)*

After all of that, the AST is consumed by the LLVM Code Generation (codegen) module.
