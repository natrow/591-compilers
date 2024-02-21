//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan

use std::{fmt::Display, path::Path};

pub mod token;
use colored::Colorize;
use token::*;
pub mod error;
use error::*;
mod fsm; // see this file for DFA scanner implementation
use fsm::*;

use crate::file_buffer::*;

/// Scanner implemented as an iterator. This can be thought of as a wrapping FSM
/// which keeps track of the input buffer and its position. Calls to this iterator
/// are guaranteed to return either the next token or an error until the end of the
/// input buffer.
///
/// Note: iterators in Rust are evaluated lazily. This means this iterator holds a file
/// handle and only reads lines as needed. If the parser is implemented as another iterator,
/// it would call `.next()` on this one as needed.
///
/// The `Lines<T>` buffer will normally buffer line-by-line which has better performance
/// than individual calls to `File::read()`.
pub struct Scanner {
    /// Finite state machine that does actual scanning
    ///
    /// This is an `Option<T>` because after an error or EOF it is set to `None`
    fsm: Option<Fsm>,
    /// Whether or not to print debug information
    debug: bool,
    /// Whether or not to print verbose information
    verbose: bool,
    /// Whether the EOF token has been inserted to the stream
    eof: bool,
    /// Internal count of the number of tokens returned
    token_count: usize,
    /// File buffer
    file_buffer: FileBuffer,
}

impl Scanner {
    /// Constructs the scanner, attempting to open the file path for reading.
    /// Fails if file cannot be opened or first line cannot be read.
    pub fn new(path: &Path, debug: bool, verbose: bool) -> Result<Self, Error> {
        let file_buffer = FileBuffer::new(path, verbose)?;

        Ok(Self {
            fsm: Some(Default::default()),
            debug,
            verbose,
            file_buffer,
            eof: false,
            token_count: 0,
        })
    }

    /// Attempts to finish FSM
    fn finish_fsm(&mut self) -> Result<(Option<Token>, Option<Warning>), Error> {
        self.fsm.take().unwrap().finish()
    }

    /// Attempts to make an EOF token, returning `Some(Ok(Token::Eof))` on the first
    /// call and `None` on subsequent calls.
    fn make_eof_token(&mut self) -> Option<Token> {
        if !self.eof {
            self.eof = true;
            self.token_count += 1;
            if self.debug {
                println!("[SCANNER] {}", Token::Eof);
                println!("[SCANNER] Total tokens: {}", self.token_count);
            }
            Some(Token::Eof)
        } else {
            None
        }
    }

    /// Add context to a given error
    fn context<T: Display>(&self, t: T) -> Context<T> {
        self.file_buffer.context(t).unwrap()
    }

    /// Prints warnings with context
    ///
    /// This is not a method function because in the context of the loop, the borrow check fails.
    fn print_warning(f: &FileBuffer, w: Warning) {
        eprintln!("{} {}", "[WARNING]".yellow(), f.context(w).unwrap());
    }

    /// Prints tokens in debug mode
    fn print_token(&self, t: &Token) {
        if self.debug {
            println!("[SCANNER] {}", t);
        }
    }
}

impl Iterator for Scanner {
    type Item = Result<Token, Context<Error>>;

    /// Implementation of iterator. Points worth noting in this API:
    /// - `Some(Ok(T))` indicates that the scanning happened with no errors
    /// - `Some(Error(T))` indicates that the scanner returned an error, and the
    ///    caller may either ignore this error or abort scanning. (Warnings are printed)
    /// - `None` indicates that the scanner has completed scanning the file and the
    ///   iterator may be discarded. It is crucial that this is not returned early.
    ///
    /// Every call must return one and only one value.
    fn next(&mut self) -> Option<Self::Item> {
        // 1: Check if FSM is in a valid state
        if let Some(fsm) = &mut self.fsm {
            // 2: Get next character in the buffer
            // (repeat until a token is made or there are no more characters)
            while let Some(c) = self.file_buffer.get_char() {
                if self.verbose {
                    println!("[SCANNER] Running state machine against char {}", c);
                }
                // 3: Attempt to run the state machine
                match fsm.step(c) {
                    Ok((t, w, r)) => {
                        if let Some(w) = w {
                            Self::print_warning(&self.file_buffer, w);
                        }
                        if !r {
                            if self.verbose {
                                println!("[SCANNER] Advancing...");
                            }
                            if let Err(e) = self.file_buffer.advance() {
                                return Some(Err(e.map_kind(Error::Io)));
                            }
                        }
                        if let Some(t) = t {
                            self.token_count += 1;
                            self.print_token(&t);
                            return Some(Ok(t));
                        }
                    }
                    Err(e) => return Some(Err(self.context(e))),
                }
            }
            // 4: Finish the state machine
            match self.finish_fsm() {
                Ok((t, w)) => {
                    if let Some(w) = w {
                        Self::print_warning(&self.file_buffer, w);
                    }
                    if let Some(t) = t {
                        self.token_count += 1;
                        self.print_token(&t);
                        Some(Ok(t))
                    } else {
                        self.make_eof_token().map(Ok)
                    }
                }
                Err(e) => Some(Err(self.context(e))),
            }
        } else {
            self.make_eof_token().map(Ok)
        }
    }
}
