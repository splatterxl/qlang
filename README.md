# Qlang

This is a simple syntax parser implementation I made in Rust. It's not functional atm, just parses the text and returns a top level object later to be consumed by LLVM.

```rs
fn a(x: int, b: str) -> :int {
  a(x);
  a + b;
}
```
