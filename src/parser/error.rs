use std::fmt::Display;

use ansi_term::{Colour, Style};

#[macro_export]
macro_rules! parser_error {
    () => {};
    ($code:tt, $message:tt, $at:tt) => {
        let err = CompileErrorBuilder::new()
            .code($code)
            .message($message)
            .at($at)
            .build();

        eprintln!("{}", err);
    };
}

fn minus_one(i: usize) -> usize {
    if i == 0 {
        0
    } else {
        i - 1
    }
}

#[derive(Default, Debug)]
pub struct CompileError {
    pub code: u32,
    pub message: String,
    pub at: (usize, usize),
    pub hints: Vec<String>,
    pub notes: Vec<String>,
    raw: String,
}

impl CompileError {
    pub fn with_raw(&mut self, raw: String) -> &mut Self {
        self.raw = raw;
        self
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) -> &mut Self {
        self.at = pos;
        self
    }

    fn line(&self) -> usize {
        self.at.0
    }

    fn column(&self) -> usize {
        self.at.1
    }

    fn fmt_at(&self, pos: Option<(usize, usize)>) -> String {
        let (line, column) = pos.unwrap_or(self.at);

        format!("{}:{}", line, column)
    }

    fn prepare_line(&self) -> (String, (usize, usize)) {
        let lines = self.raw.lines().collect::<Vec<_>>();
        let mut idx = self.line();
        let mut col = self.column();
        let mut line = lines.get(idx);

        // check for unexpected EOF & adjust line/col accordingly
        if self.code == 1002 && line.is_none() {
            let mut tmp = lines.get(idx - 1).unwrap_or(&"");
            idx -= 1;

            // newline at end of file
            if tmp.trim().is_empty() {
                // we can ignore a newline in this, because it's not the last line
                tmp = lines.get(idx - 1).unwrap_or(&"");
                idx -= 1;
            }

            col = tmp.len();
            line = Some(tmp);
        }

        (line.unwrap_or(&"<unknown>").to_string(), (idx, col))
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (l, pos) = self.prepare_line();
        let p = self.fmt_at(Some(pos));
        let s = Style::new();

        f.write_str(&format!(
            "{} [E{}] {} at {}\n\t{} {}\n\t   {} {}",
            &s.fg(Colour::Red).paint("Error"),
            self.code,
            self.message,
            p,
            &s.dimmed().paint("at:"),
            l,
            " ".repeat(minus_one(pos.1)),
            &s.fg(Colour::Red).paint("^")
        ))
        .ok();

        if !self.notes.is_empty() {
            for note in &self.notes {
                f.write_str(&format!(
                    "\n{} {}",
                    &s.fg(Colour::Green).paint("note:"),
                    note
                ))
                .ok();
            }
        }

        if !self.hints.is_empty() {
            for hint in &self.hints {
                f.write_str(&format!(
                    "\n{} {}",
                    &s.fg(Colour::Blue).paint("hint:"),
                    hint
                ))
                .ok();
            }
        }

        Ok(())
    }
}

pub enum Error {
    Compile(CompileError),
}

pub struct CompileErrorBuilder {
    pub code: u32,
    pub message: String,
    pub at: (usize, usize),
    pub hints: Vec<String>,
    pub notes: Vec<String>,
}

impl CompileErrorBuilder {
    pub fn new() -> Self {
        CompileErrorBuilder {
            code: 0,
            message: String::new(),
            at: (0, 0),
            hints: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn init(template: ErrorCodes) -> Self {
        CompileErrorBuilder {
            code: template.code(),
            message: template.message().to_string(),
            at: (0, 0),
            hints: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn at(mut self, at: (usize, usize)) -> Self {
        self.at = at;
        self
    }

    pub fn hint(mut self, hint: &str) -> Self {
        self.hints.push(hint.to_string());
        self
    }

    pub fn note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }

    pub fn from(mut self, template: ErrorCodes) -> Self {
        self.code = template.code();
        self.message = template.message();
        self
    }

    pub fn build(self) -> CompileError {
        CompileError {
            code: self.code,
            message: self.message,
            at: self.at,
            hints: self.hints,
            notes: self.notes,
            raw: String::new(),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ErrorCodes {
    UnknownError = 1000,
    UnexpectedToken,
    UnexpectedEOF,
}

impl ErrorCodes {
    pub fn code(&self) -> u32 {
        *self as _
    }

    pub fn message(&self) -> String {
        match self {
            ErrorCodes::UnknownError => "Unknown error".to_string(),
            ErrorCodes::UnexpectedToken => "Unexpected token".to_string(),
            ErrorCodes::UnexpectedEOF => "Unexpected end of file".to_string(),
        }
    }

    pub fn resolve(self) -> (u32, String) {
        (self.code(), self.message())
    }
}
