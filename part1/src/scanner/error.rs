use std::{fmt::Display, io};

/// Error type for the scanner. Includes information about the kind of error and its location.
///
/// This type is generic so that it can store both errors and warnings.
#[derive(Debug)]
pub struct Error<T: Display> {
    kind: T,
    line: String,
    line_num: usize,
    line_index: usize,
}

impl<T: Display> Error<T> {
    pub fn new(kind: T, line: String, line_num: usize, line_index: usize) -> Self {
        Self {
            kind,
            line,
            line_num,
            line_index,
        }
    }
}

impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // [ERROR] unclosed comment on line 12:
        //
        // 123 "hello" /*
        //              ^-- error happened here

        write!(
            f,
            "{} on line {}:\n\n{}\n{}^-- error happened here\n",
            self.kind,
            self.line_num,
            self.line,
            " ".repeat(self.line_index)
        )
    }
}

/// Types of errors that can happen during scanning.
#[derive(Debug)]
pub enum ErrorKind {
    /// EOF reached before '*/' sequence
    UnclosedComment,
    /// '\n' found in a character literal
    NewlineInCharLiteral,
    /// EOF reached before closing ' character
    UnclosedCharLiteral,
    /// '\n' found in a string literal
    NewlineInStringLiteral,
    /// EOF reached before closing " character
    UnclosedStringLiteral,
    /// State machine holds an invalid value
    CorruptState,
    /// Errors occuring because of I/O
    Io(io::Error),
}

impl From<io::Error> for ErrorKind {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ErrorKind::UnclosedComment => "unclosed comment",
            ErrorKind::NewlineInCharLiteral => "newline in character literal",
            ErrorKind::UnclosedCharLiteral => "unclosed character literal",
            ErrorKind::NewlineInStringLiteral => "newline in string literal",
            ErrorKind::UnclosedStringLiteral => "unclosed string literal",
            ErrorKind::CorruptState => "state machine was corrupted",
            ErrorKind::Io(e) => return write!(f, "i/o error occured ({:?})", e),
        };

        write!(f, "{}", str)
    }
}

/// Warnings that can be generated by the scanner. These are simply printed and ignored.
#[derive(Debug)]
pub enum WarningKind {
    IllegalCharacter,
}

impl Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            WarningKind::IllegalCharacter => "illegal character",
        };

        write!(f, "{}", str)
    }
}
