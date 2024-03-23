//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
    iter::Peekable,
    path::Path,
};

use crate::context::Context;

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
    ///
    /// # Errors
    ///
    /// Fails if the file cannot be opened or the first line cannot be read.
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

        // case 1: get a normal character along the line
        // case 2: insert newlines where applicable (these are stripped by Lines<T>)
        // case 3: buffer is empty

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
    ///
    /// # Errors
    ///
    /// Fails a line cannot be read.
    #[allow(clippy::missing_panics_doc)] // .unwrap() is unreachable
    pub fn advance(&mut self) -> Result<(), Context<Error>> {
        let Some(line) = &self.line else {
            return Ok(());
        };

        // either move along the line or refresh the line buffer

        if self.line_index < line.len() {
            if self.verbose {
                println!("[FILE_BUFFER] advancing line_index");
            }
            // note, this DOES allow line_index to be equal to line.len(),
            // in order to inject newline characters in the get_char() function.
            self.line_index += 1;
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
        }

        Ok(())
    }
}

impl Iterator for FileBuffer {
    type Item = Result<char, Context<Error>>;

    // combine get_char() and advance() into one call
    fn next(&mut self) -> Option<Self::Item> {
        // get next character
        let c = self.get_char()?;
        // advance, filling the Ok() value with the character
        // then transposing the Result<Option<T>> to an Option<Result<T>>
        self.advance().map(|_| Some(c)).transpose()
    }
}
