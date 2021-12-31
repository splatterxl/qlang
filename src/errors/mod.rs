use ansi_term::{
    Colour::{Blue, Cyan, Red, Yellow},
    Style,
};
use logos::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Syntax {
        message: String,
        span: Span,
        hints: Vec<String>,
        code: u32,
        lc: (usize, usize),
    },
}

#[allow(dead_code)]
impl Error {
    pub fn new(
        message: String,
        span: Span,
        hints: Vec<String>,
        code: u32,
        lc: (usize, usize),
    ) -> Self {
        Error::Syntax {
            message,
            span,
            hints,
            code,
            lc: lc,
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

    pub fn line(&self) -> usize {
        match self {
            Error::Syntax { lc, .. } => lc.0,
        }
    }

    pub fn column(&self) -> usize {
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
            &lines[line],
            " ".repeat({
                if column == 0 {
                    column
                } else {
                    column - 1
                }
            }),
            Red.paint("^".repeat(span.end - span.start)),
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
