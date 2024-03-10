//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use std::fmt::Display;

use crate::{
    file_buffer::Context,
    scanner::{error::Error as ScannerError, token::Token},
};

/// Create a comma separated list of `T::to_string()`
fn list_to_string<T: Display>(list: &[T]) -> String {
    let mut s = String::new();
    for (i, e) in list.iter().enumerate() {
        s += &e.to_string();
        if i < list.len() - 1 {
            s += ", ";
        }
    }
    s
}

/// Types of errors that can happen during parsing.
#[derive(Debug)]
pub enum Error {
    /// Syntax errors
    SyntaxError {
        /// The token that was actually scanned
        got: Token,
        /// The token which was expected
        expected: Vec<&'static str>,
    },
    /// An error returned from the scanner
    ScannerError(ScannerError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::SyntaxError { got, expected } => {
                format!(
                    "got: {}, expected{}: {}",
                    got,
                    if expected.len() == 1 { "" } else { " one of" },
                    list_to_string(expected)
                )
            }
            Self::ScannerError(e) => e.to_string(),
        };

        write!(f, "{}", str)
    }
}

impl From<ScannerError> for Error {
    fn from(value: ScannerError) -> Self {
        Self::ScannerError(value)
    }
}

impl From<Context<ScannerError>> for Context<Error> {
    fn from(value: Context<ScannerError>) -> Self {
        value.map_kind(Error::from)
    }
}
