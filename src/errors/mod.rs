pub mod templates;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub line: usize,
    pub at: usize,
    pub symbol: Option<String>
}
