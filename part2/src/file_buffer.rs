//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan

use colored::Colorize;

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
    iter::Peekable,
    path::Path,
};

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

/// An iterator over the characters in a file.
/// Internally buffers by line.
pub struct FileBuffer {
    /// Inner file buffer
    inner: Peekable<Lines<BufReader<File>>>,
    /// Current line being read
    line: Option<String>,
    /// Current position along line
    line_index: usize,
    /// Current line number
    line_num: usize,
    /// Current file name
    file_name: String,
    /// Whether or not to display verbose debugging information
    verbose: bool,
}

impl FileBuffer {
    /// Constructor for FileBuffer
    pub fn new(path: &Path, verbose: bool) -> Result<Self, Error> {
        let file_name = path.to_string_lossy().to_string();
        let mut inner = BufReader::new(File::open(path)?).lines().peekable();
        let line = inner.next().transpose()?;

        Ok(Self {
            inner,
            line,
            line_index: 0,
            line_num: 0,
            file_name,
            verbose,
        })
    }

    /// Get context for a warning or error
    pub fn context<T: Display>(&self, t: T) -> Option<Context<T>> {
        Some(Context::new(
            t,
            self.line.clone()?,
            self.line_num,
            self.line_index,
            self.file_name.clone(),
        ))
    }

    /// Gets the current character
    pub fn get_char(&mut self) -> Option<char> {
        let line = self.line.as_ref()?;
        if let Some(c) = line.chars().nth(self.line_index) {
            if self.verbose {
                println!("[FILE_BUFFER] Got character from line: {}", c);
            }
            Some(c)
        } else if self.line_index == line.len() && self.inner.peek().is_some() {
            if self.verbose {
                println!("[FILE_BUFFER] Inserting newline...");
            }
            Some('\n')
        } else {
            if self.verbose {
                println!("[FILE_BUFFER] Couldn't get char")
            }
            None
        }
    }

    /// Moves to the next character in the buffer
    pub fn advance(&mut self) -> Result<(), Context<Error>> {
        if let Some(line) = &self.line {
            // advance to the next character in the line or fetch the next one
            if self.line_index < line.len() {
                self.line_index += 1;
                if self.verbose {
                    println!("[FILE_BUFFER] advancing line_index");
                }
                Ok(())
            } else {
                if self.verbose {
                    println!("[FILE_BUFFER] advancing line_num");
                }
                self.line_index = 0;
                self.line_num += 1;
                self.line = self
                    .inner
                    .next()
                    .transpose()
                    .map_err(|e| self.context(e).unwrap())?;
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl Iterator for FileBuffer {
    type Item = Result<char, Context<Error>>;

    // combine get_char() and advance() into one call
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.get_char()?;
        self.advance().map(|_| Some(c)).transpose()
    }
}
