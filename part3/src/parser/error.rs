//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Implementation of an error type for the parser.

use std::fmt::Display;

use crate::{
    context::Context,
    scanner::{error::Error as ScannerError, token::Token},
};

/// Create a comma separated list using [ToString::to_string]
fn list_to_string<I: IntoIterator<Item = T>, T: Display>(list: I) -> String {
    let mut s = String::new();
    let mut iter = list.into_iter().peekable();
    while let Some(e) = iter.next() {
        s += &e.to_string();
        if iter.peek().is_some() {
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
        expected: Vec<Token>,
    },
    /// An error returned from the scanner
    ScannerError(ScannerError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::SyntaxError { got, expected } => {
                format!(
                    "invalid syntax, got: {}, expected{}: {}",
                    got.as_str(),
                    if expected.len() == 1 { "" } else { " one of" },
                    list_to_string(expected.iter().map(|e| e.as_str()))
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
