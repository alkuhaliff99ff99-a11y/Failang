#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnterminatedString {
        line: usize,
        column: usize,
    },
    UnterminatedComment {
        line: usize,
        column: usize,
    },
    InvalidCharacter {
        ch: char,
        line: usize,
        column: usize,
    },
}
