//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan

use std::{fmt::Display, io};

use colored::Colorize;

/// Error type for the scanner. Includes information about the kind of error and its location.
///
/// This type is generic so that it can store both errors and warnings.
#[derive(Debug)]
pub struct Error<T: Display> {
    /// The type of error that occurred
    kind: T,
    /// The contents of the line on which the error occurred
    line: String,
    /// The line number on which the error occurred
    line_num: usize,
    /// The place along the line on which the error occurred
    line_index: usize,
    /// The name of the file in which the error occurred
    file_name: String,
}

impl<T: Display> Error<T> {
    /// Construct a new `Error<T>`
    pub fn new(
        kind: T,
        line: String,
        line_num: usize,
        line_index: usize,
        file_name: String,
    ) -> Self {
        Self {
            kind,
            line,
            line_num,
            line_index,
            file_name,
        }
    }
}

impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // [ERROR] unclosed comment in test.c:12:34:
        // 123 "hello" /*
        //              ^~~ error happened here

        write!(
            f,
            "{} in {}:{}:{}:\n{}\n{}{}\n",
            self.kind,
            self.file_name.purple(),
            self.line_num.to_string().purple(),
            self.line_index.to_string().purple(),
            self.line,
            " ".repeat(self.line_index),
            "^~~ happened here".blue()
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
    /// Errors occurring because of I/O
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
            ErrorKind::Io(e) => return write!(f, "i/o error occurred ({:?})", e),
        };

        write!(f, "{}", str)
    }
}

/// Warnings that can be generated by the scanner. These are simply printed and ignored.
#[derive(Debug)]
pub enum WarningKind {
    /// Input contains an unknown character.
    ///
    /// Comments, character literals, and string literals will not generate this warning.
    IllegalCharacter,
}

impl Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            WarningKind::IllegalCharacter => "ignoring illegal character",
        };

        write!(f, "{}", str)
    }
}
