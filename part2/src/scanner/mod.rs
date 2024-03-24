//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan
//!
//! The scanner, as implemented in part 1 of the project.

use std::{fmt::Display, path::Path};

use colored::Colorize;

use crate::{context::Context, file_buffer::FileBuffer};

pub mod error;
mod fsm;
pub mod token; // see this file for DFA scanner implementation

use error::{Error, Warning};
use fsm::Fsm;
use token::Token;

/// Scanner implemented as an iterator. This combines both the FSM and the [FileBuffer]
/// (also implemented as an iterator) and handles all the call-site logic and invariance
/// for the FSM.
///
/// Note: Rust iterators are lazily evaluated, so the file is only read and tokens
/// are only scanned as needed. Since the parser is implemented as an LL(1) recursive
/// descent parser, there is a minimal amount of memory overhead.
pub struct Scanner {
    /// Finite state machine that does actual scanning
    ///
    /// This is an [Option] because after an error or EOF it is set to [None]
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
    ///
    /// # Errors
    ///
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

    /// Attempts to make an EOF token, returning [Some(Ok(Token::Eof))] on the first
    /// call and [None] on subsequent calls.
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
    #[allow(clippy::missing_panics_doc)] // constructor guarantees this won't panic
    pub fn context<T: Display>(&self, t: T) -> Context<T> {
        self.file_buffer.context(t).unwrap()
    }

    /// Prints warnings with context
    ///
    /// This is not a method function because in the context of the loop, the borrow check fails.
    fn print_warning(f: &FileBuffer, w: Warning) {
        eprintln!("{} {}", "[WARNING]".yellow(), f.context(w).unwrap());
    }

    /// Prints tokens in debug mode
    fn debug_print_token(&self, t: &Token) {
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
    /// - [None] indicates that the scanner has completed scanning the file and the
    ///   iterator may be discarded. It is crucial that this is not returned early.
    ///
    /// Every call must return one and only one value.
    fn next(&mut self) -> Option<Self::Item> {
        // 1: Check if FSM is in a valid state
        let Some(fsm) = &mut self.fsm else {
            return self.make_eof_token().map(Ok);
        };

        // 2: Get next character in the buffer
        // (repeat until a token is made or there are no more characters)
        while let Some(c) = self.file_buffer.get_char() {
            if self.verbose {
                println!("[SCANNER] Running state machine against char {}", c);
            }

            // 3: Attempt to run state machine
            match fsm.step(c) {
                Ok((t, w)) => {
                    if let Some(w) = w {
                        Self::print_warning(&self.file_buffer, w);
                    }
                    if let Some(t) = t {
                        self.token_count += 1;
                        self.debug_print_token(&t);
                        return Some(Ok(t));
                    }

                    // if no token was returned, advance the buffer

                    if self.verbose {
                        println!("[SCANNER] Advancing...");
                    }
                    if let Err(e) = self.file_buffer.advance() {
                        return Some(Err(e.map_kind(Error::Io)));
                    }
                }
                Err(e) => return Some(Err(self.context(e))),
            }
        }

        // 4. Finish the state machine
        match self.finish_fsm() {
            Ok((t, w)) => {
                if let Some(w) = w {
                    Self::print_warning(&self.file_buffer, w);
                }
                if let Some(t) = t {
                    self.token_count += 1;
                    self.debug_print_token(&t);
                    Some(Ok(t))
                } else {
                    self.make_eof_token().map(Ok)
                }
            }
            Err(e) => Some(Err(self.context(e))),
        }
    }
}
