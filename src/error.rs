use std::fmt::Display;

use ansi_term::Style;

macro_rules! compile_error {
    () => {};
    ($code:tt, $message:tt, $at:tt) => {};
}

pub struct CompileError {
    pub code: u32,
    pub message: String,
    pub at: String,
    pub hints: Vec<String>,
    pub notes: Vec<String>,
}

impl Default for CompileError {
    fn default() -> Self {
        CompileError {
            code: 0,
            message: String::new(),
            at: String::new(),
            hints: Vec::new(),
            notes: Vec::new(),
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} [E{}] {} at {}",
            Style::new().fg(ansi_term::Color::Red).paint("Error"),
            self.code,
            Style::new().bold().paint(self.message.clone()),
            self.at
        )
    }
}
