use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use colored::Colorize;

pub mod token;
use token::*;
pub mod error;
use error::*;

/// Scanner implemented as a finite state machine. This is a private member to insure
/// correct usage of the 'step' and 'finish' functions.
///
/// Note: This FSM keeps no track of its location in a file, and therefore does NOT
/// return a complete Error<T>, only the kind of error (or warning) that occurred.
#[derive(Default, Clone)]
struct ScannerFsm {
    /// Current state, represented as an 8-bit unsigned integer (max value: 34)
    state: u8,
    /// Current token being scanned, used to fill attribute fields
    _token: String,
    /// Current number of nested comment tags
    _comment_level: usize,
}

impl ScannerFsm {
    /// Implementation of the DFA transitions.
    ///
    /// Can return an error, or a pair of an optional token and optional warning.
    ///
    /// Note: accepting states only return a value on *the next edge*.
    fn step(&mut self, c: char) -> Result<(Option<Token>, Option<WarningKind>), ErrorKind> {
        // FSM implementation
        match self.state {
            0 => {
                if c.is_ascii_whitespace() {
                    Ok((None, None))
                } else {
                    match c {
                        '/' => {
                            self.state = 1;
                            Ok((None, None))
                        }
                        _ => Ok((None, Some(WarningKind::IllegalCharacter))),
                    }
                }
            }
            1 => match c {
                '/' => todo!(),
                '*' => todo!(),
                _ => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Div)), None))
                }
            },
            _ => Err(ErrorKind::CorruptState),
        }
    }

    /// Consumes the DFA and evaluates the validity of the final state.
    fn finish(self) -> Result<(Option<Token>, Option<WarningKind>), ErrorKind> {
        match self.state {
            0 => Ok((None, None)),
            _ => Err(ErrorKind::CorruptState),
        }
    }
}

/// Scanner implemented as an iterator. This can be thought of as a wrapping FSM
/// which keeps track of the input buffer and its position. Calls to this iterator
/// are garaunteed to return either the next token or an error until the end of the
/// input buffer.
///
/// Note: iterators in Rust are evaluated lazily. This means this iterator holds a file
/// handle and only reads lines as needed. If the parser is implemented as another iterator,
/// it would call `.next()` on this one as needed.
///
/// The `Lines<T>` buffer will normally buffer line-by-line which has better performance
/// than individual calls to `File::read()`.
pub struct Scanner {
    /// File read buffer
    buffer: Lines<BufReader<File>>,
    /// Current line being scanned
    line: String,
    /// Number of line being scanned
    line_num: usize,
    /// Position along the line being scanned
    line_index: usize,
    /// Finite state machine that does actual scanning
    ///
    /// This is an `Option<T>` because after an error or EOF it is set to `None`
    fsm: Option<ScannerFsm>,
    /// Whether or not to print debug information
    debug: bool,
    /// Name of the file currently open
    file_name: String,
    /// Whether the EOF token has been inserted to the stream
    eof: bool,
    /// Internal count of the number of tokens returned
    token_count: usize,
}

impl Scanner {
    /// Constructs the scanner, attempting to open the file path for reading.
    /// Fails if file cannot be opened or first line cannot be read.
    pub fn new(path: &Path, debug: bool) -> Result<Self, io::Error> {
        let file_name = path.to_string_lossy().to_string();
        let mut buffer = BufReader::new(File::open(path)?).lines();

        // initialize line buffer
        let line = buffer.next().unwrap_or_else(|| Ok(String::new()))?;

        Ok(Self {
            buffer,
            line,
            line_num: 0,
            line_index: 0,
            fsm: Some(Default::default()),
            debug,
            file_name,
            eof: false,
            token_count: 0,
        })
    }

    /// Construct an error value using location information from self
    fn error<E: Display>(&self, kind: E) -> Error<E> {
        Error::new(
            kind,
            self.line.clone(),
            self.line_num,
            self.line_index,
            self.file_name.clone(),
        )
    }

    /// Construct an I/O type error using information from self
    fn handle_io_error(&self, e: io::Error) -> Error<ErrorKind> {
        self.error(ErrorKind::Io(e))
    }

    /// Attempts to make an EOF token, returning `Some(Ok(Token::Eof))`` on the first
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
}

impl Iterator for Scanner {
    type Item = Result<Token, Error<ErrorKind>>;

    /// Implementation of iterator. Points worth noting in this API:
    /// - `Some(Ok(T))` indicates that the scanning happened with no errors
    /// - `Some(Error(T))` indicates that the scanner returned an error, and the
    ///    caller may either ignore this error or abort scanning. (Warnings are printed)
    /// - `None` indicates that the scanner has completed scanning the file and the
    ///   iterator may be discarded. It is crucial that this is not returned early.
    ///
    /// Every call must return one and only one value.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fsm) = &mut self.fsm {
            loop {
                // 1: attempt to read from current line
                for c in self.line[self.line_index..].chars() {
                    match fsm.step(c) {
                        Ok((t, w)) => {
                            if let Some(w) = w {
                                // can't use self.error(w) here because of borrow checker semantics
                                eprintln!(
                                    "{} {}",
                                    "[WARNING]".yellow(),
                                    Error::new(
                                        w,
                                        self.line.clone(),
                                        self.line_num,
                                        self.line_index,
                                        self.file_name.clone(),
                                    )
                                );
                            }

                            if let Some(t) = t {
                                if self.debug {
                                    println!("[SCANNER] {}", &t)
                                }
                                self.token_count += 1;
                                return Some(Ok(t));
                            }
                        }
                        Err(e) => {
                            self.fsm.take().unwrap(); // invalidate the FSM, abort parsing
                            return Some(Err(self.error(e)));
                        }
                    }
                    self.line_index += 1;
                }
                // 2: get next line
                match self.buffer.next() {
                    Some(Ok(line)) => self.line = line,
                    Some(Err(e)) => return Some(Err(self.handle_io_error(e))),

                    // 3: repeat until all lines read
                    None => break,
                }
                self.line_index = 0;
                self.line_num += 1;
            }

            // 4: attempt to finish the FSM
            match self.fsm.take().unwrap().finish() {
                Ok((t, w)) => {
                    if let Some(w) = w {
                        eprintln!("{} {}", "[WARNING]".yellow(), self.error(w));
                    }

                    if let Some(t) = t {
                        if self.debug {
                            self.token_count += 1;
                            println!("[SCANNER] {}", &t)
                        }
                        Some(Ok(t))
                    } else {
                        self.make_eof_token().map(Ok)
                    }
                }
                Err(e) => Some(Err(self.error(e))),
            }
        } else {
            self.make_eof_token().map(Ok)
        }
    }
}
