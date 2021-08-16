use ansi_term::{
    Color::{Red, Yellow},
    Style,
};

pub fn error(s: String) -> String {
    Style::new().bold().fg(Red).paint(s).to_string()
}

pub fn number(num: usize) -> String {
    Style::new().fg(Yellow).paint(num.to_string()).to_string()
}
