use ansi_term::{
    Colour::{Blue, Cyan, Red, Yellow},
    Style,
};
use logos::Span;

use crate::error::codes::UNEXPECTED_END_OF_INPUT;

pub mod codes;
pub mod errors;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Syntax {
        message: String,
        span: Span,
        hints: Vec<String>,
        code: u32,
        lc: (u32, u32),
    },
}

#[allow(dead_code)]
impl Error {
    pub fn new(message: String, span: Span, hints: Vec<String>, code: u32, lc: (u32, u32)) -> Self {
        Error::Syntax {
            message,
            span,
            hints,
            code,
            lc,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Error::Syntax { message, .. } => message.to_string(),
        }
    }

    pub fn span(self) -> Span {
        match self {
            Error::Syntax { span, .. } => span,
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Error::Syntax { code, .. } => *code,
        }
    }

    pub fn line(&self) -> u32 {
        match self {
            Error::Syntax { lc, .. } => lc.0,
        }
    }

    pub fn column(&self) -> u32 {
        match self {
            Error::Syntax { lc, .. } => lc.1,
        }
    }

    pub fn hints(&self) -> Vec<String> {
        match self {
            Error::Syntax { hints, .. } => hints.clone(),
        }
    }
}

pub fn dim(s: &str) -> String {
    Style::new().dimmed().paint(s).to_string()
}

pub fn format_errs(raw: String, errors: Vec<Error>) {
    let lines = raw.lines().map(|s| s.to_string()).collect::<Vec<String>>();

    for err in errors {
        let message = err.message();
        let code = err.code();
        let line = err.line();
        let column = err.column();
        let hints = err.hints();
        let span = err.span();

        println!(
            "{}: {} {}{}{}\n\t{} {}\t{}\n\t\t{}{}\n{}",
            Blue.paint("SyntaxError"),
            message,
            dim("["),
            Yellow.paint(code.to_string()),
            dim("]"),
            dim((line + 1).to_string().as_str()),
            dim("|"),
            &lines[line as usize],
            " ".repeat({
                if code == UNEXPECTED_END_OF_INPUT {
                    lines[line as usize].len()
                } else if column == 0 {
                    column as usize
                } else {
                    (column - 1) as usize
                }
            }),
            Red.paint("^".repeat({
                if span.end - span.start == 0 {
                    1
                } else {
                    span.end - span.start
                }
            })),
            {
                if !hints.is_empty() {
                    hints
                        .iter()
                        .map(|hint| format!("{} {}\n", Cyan.paint("hint:"), hint))
                        .collect::<String>()
                } else {
                    String::new()
                }
            }
        );
    }
}
