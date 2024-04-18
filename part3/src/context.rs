//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Contextual errors. Created using [crate::file_buffer::FileBuffer].
//! [MaybeContext] allows mixing these errors with others.

use std::fmt::Display;

use colored::Colorize;

/// Gives locational context to the inner error/warning type
#[derive(Debug)]
pub struct Context<T: Display> {
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

impl<T: Display> Context<T> {
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

    /// Allows the conversion from one error type to another while keeping the context the same.
    pub fn map_kind<F: FnOnce(T) -> U, U: Display>(self, f: F) -> Context<U> {
        let Self {
            kind,
            line,
            line_num,
            line_index,
            file_name,
        } = self;

        let kind = f(kind);

        Context {
            kind,
            line,
            line_num,
            line_index,
            file_name,
        }
    }
}

impl<T: Display> Display for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // [ERROR] unclosed comment in test.c:12:34:
        // 123 "hello" /*
        //              ^~~ error happened here

        write!(
            f,
            "{} in {}:{}:{}:\n{}\n{}{}\n",
            self.kind,
            self.file_name.purple(),
            (self.line_num + 1).to_string().purple(),
            (self.line_index + 1).to_string().purple(),
            self.line,
            " ".repeat(self.line_index),
            "^~~ happened here".blue()
        )
    }
}

/// An error type that may or may not have locational context
pub enum MaybeContext<T: Display> {
    /// Variant that happens when there is locational context
    Context(Context<T>),
    /// Variant that happens when there is no locational context
    NoContext(T),
}

impl<T: Display> MaybeContext<T> {
    /// Allows the conversion from one error type to another while keeping the context the same.
    pub fn map_kind<F: FnOnce(T) -> U, U: Display>(self, f: F) -> MaybeContext<U> {
        match self {
            MaybeContext::Context(e) => MaybeContext::Context(e.map_kind(f)),
            MaybeContext::NoContext(e) => MaybeContext::NoContext(f(e)),
        }
    }
}

impl<T: Display> From<Context<T>> for MaybeContext<T> {
    fn from(value: Context<T>) -> Self {
        Self::Context(value)
    }
}

impl<T: Display> From<T> for MaybeContext<T> {
    fn from(value: T) -> Self {
        Self::NoContext(value)
    }
}

impl<T: Display> Display for MaybeContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeContext::Context(c) => c.fmt(f),
            MaybeContext::NoContext(n) => n.fmt(f),
        }
    }
}
