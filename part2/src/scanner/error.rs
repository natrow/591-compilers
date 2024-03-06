//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use std::{fmt::Display, io};

/// Types of errors that can happen during scanning.
#[derive(Debug)]
pub enum Error {
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

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Error::UnclosedComment => "unclosed comment",
            Error::NewlineInCharLiteral => "newline in character literal",
            Error::UnclosedCharLiteral => "unclosed character literal",
            Error::NewlineInStringLiteral => "newline in string literal",
            Error::UnclosedStringLiteral => "unclosed string literal",
            Error::CorruptState => "state machine was corrupted",
            Error::Io(e) => return write!(f, "i/o error occurred ({:?})", e),
        };

        write!(f, "{}", str)
    }
}

/// Warnings that can be generated by the scanner. These are simply printed and ignored.
#[derive(Debug)]
pub enum Warning {
    /// Input contains an unknown character.
    ///
    /// Comments, character literals, and string literals will not generate this warning.
    IllegalCharacter,
}

impl Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Warning::IllegalCharacter => "ignoring illegal character",
        };

        write!(f, "{}", str)
    }
}
