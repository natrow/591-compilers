//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan

use std::{
    f32::consts::E,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    option,
    path::{Path, Prefix},
    ptr::slice_from_raw_parts,
    string,
    thread::sleep,
    time::Duration,
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
        //sleep(Duration::from_secs(2));
        println!("Outside the match: At state {} c: {}", self.state, c);
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
                        'A'..='Z' | 'a'..='z' => {
                            println!("At state {} c: {}", self.state, c);
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 5;
                            Ok((None, None))
                        }

                        '0'..='9' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 6;
                            Ok((None, None))
                        }

                        '\'' => {
                            self.state = 12;
                            self._token = String::new();
                            Ok((None, None))
                        }

                        '"' => {
                            self.state = 15;
                            self._token = String::new();
                            Ok((None, None))
                        }

                        '=' => {
                            // println!("At state {} c: {}", self.state, c);
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 17;
                            Ok((None, None))
                        }

                        '!' => {
                            // println!("At state {} c: {}", self.state, c);
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 19;
                            Ok((None, None))
                        }

                        '<' | '>' => {
                            // println!("At state {} c: {}", self.state, c);
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 20;
                            Ok((None, None))
                        }

                        '+' | '-' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 21;
                            Ok((None, None))
                        }

                        '|' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 22;
                            Ok((None, None))
                        }

                        '*' | '/' | '%' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 23;
                            Ok((None, None))
                        }

                        '&' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 24;
                            Ok((None, None))
                        }

                        '(' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 25;
                            Ok((None, None))
                        }

                        ')' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 26;
                            Ok((None, None))
                        }

                        '{' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 27;
                            Ok((None, None))
                        }

                        '}' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 28;
                            Ok((None, None))
                        }

                        '[' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 29;
                            Ok((None, None))
                        }

                        ']' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 30;
                            Ok((None, None))
                        }

                        ',' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 31;
                            Ok((None, None))
                        }

                        ';' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 32;
                            Ok((None, None))
                        }

                        ':' => {
                            self._token = String::new();
                            self._token.push(c);
                            self.state = 34;
                            Ok((None, None))
                        }

                        _ => Ok((None, Some(WarningKind::IllegalCharacter))),
                    }
                }
            }

            //state 1 cases
            1 => match c {
                '/' => {
                    self.state = 2;
                    Ok((None, None))
                }

                '*' => {
                    self.state = 3;
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Div)), None))
                }
            },

            //state 2 cases
            2 => match c {
                '\n' => {
                    self.state = 0;
                    Ok((None, None))
                }
                _ => {
                    //otherwise stay at this state
                    self.state = 2;
                    Ok((None, None))
                }
            },

            //state 3 cases
            3 => match c {
                '*' => {
                    self.state = 4;
                    Ok((None, None))
                }
                _ => {
                    //otherwise stay at this state
                    self.state = 3;
                    Ok((None, None))
                }
            },

            //state 4 cases
            4 => match c {
                '/' => {
                    self.state = 0;
                    Ok((None, None))
                }
                _ => {
                    //otherwise go back to state 3
                    self.state = 3;
                    Ok((None, None))
                }
            },

            //state 5 cases
            5 => match c {
                'A'..='Z' | 'a'..='z' => {
                    println!(
                        "At state {} c: {} \n Found a leter or Ditigit",
                        self.state, c
                    );
                    self._token.push(c);
                    self.state = 5;
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Identifier(self._token.clone())), None))
                }
            },

            //found a digit
            6 => match c {
                '0'..='9' => {
                    self.state = 6;
                    self._token.push(c);
                    Ok((None, None))
                }
                '.' => {
                    self._token.push(c);
                    self.state = 7;
                    Ok((None, None))
                }
                'E' => {
                    self._token.push(c);
                    self.state = 9;
                    Ok((None, None))
                }
                _ => {
                    //End of token
                    self.state = 0;
                    Ok((Some(Token::Number(self._token.clone())), None))
                }
            },

            //foudn a period
            7 => match c {
                '0'..='9' => {
                    self.state = 8;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    //End of token
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
            },

            8 => match c {
                '0'..='9' => {
                    self.state = 8;
                    self._token.push(c);
                    Ok((None, None))
                }
                'E' => {
                    self.state = 9;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    //end of the token
                    self.state = 0;
                    Ok((Some(Token::Number(self._token.clone())), None))
                }
            },

            //found an E
            9 => match c {
                '0'..='9' => {
                    self.state = 11;
                    self._token.push(c);
                    Ok((None, None))
                }
                '+' | '-' => {
                    self.state = 10;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    //otherwise go back to state 3
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
            },

            //found + or -
            10 => match c {
                '0'..='9' => {
                    self.state = 11;
                    self._token.push(c);
                    Ok((None, None))
                }

                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
            },

            //return state
            11 => match c {
                '0'..='9' => {
                    self.state = 11;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Number(self._token.clone())), None))
                }
            },

            12 => match c {
                '\'' => {
                    //empety char
                    self.state = 13;
                    Ok((None, None))
                }
                _ => {
                    //otherwise go back to state 3
                    if c.is_whitespace() {
                        self.state = 0;
                        Ok((None, Some(WarningKind::IllegalCharacter)))
                    } else {
                        self.state = 14;
                        self._token.push(c);
                        Ok((None, None))
                    }
                }
            },
            //retunr state
            13 => {
                //otherwise go back to state 3
                self.state = 0;
                if self._token.is_empty() {
                    Ok((Some(Token::CharLiteral(None)), None))
                } else {
                    Ok((
                        Some(Token::CharLiteral(Some(
                            self._token.chars().nth(0).unwrap(),
                        ))),
                        None,
                    ))
                }
            }

            14 => match c {
                '\'' => {
                    self.state = 13;
                    Ok((None, None))
                }
                _ => {
                    //otherwise go back to state 3
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
            },

            15 => match c {
                '"' => {
                    //end of string
                    self.state = 16;
                    Ok((None, None))
                }
                '\n' => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
                _ => {
                    self.state = 15;
                    self._token.push(c);
                    Ok((None, None))
                }
            },

            16 => {
                self.state = 0;
                Ok((Some(Token::StringLiteral(self._token.clone())), None))
            }

            17 => match c {
                '=' => {
                    // println!("At state {} c: {}", self.state, c);
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::AssignOp), None))
                }
            },

            18 => {
                // println!("State {} and Token {}", self.state, self._token);
                self.state = 0;
                match self._token.as_str() {
                    "==" => Ok((Some(Token::RelOp(RelOp::Eq)), None)),
                    ">=" => Ok((Some(Token::RelOp(RelOp::GtEq)), None)),
                    "<=" => Ok((Some(Token::RelOp(RelOp::LtEq)), None)),
                    "!=" => Ok((Some(Token::RelOp(RelOp::Neq)), None)),
                    _ => {
                        println!("FSM failed to stored data");
                        Err(ErrorKind::CorruptState)
                    }
                }
            }

            19 => match c {
                '=' => {
                    self.state = 18;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Not), None))
                }
            },

            20 => match c {
                '=' => {
                    self.state = 18;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    match self._token.as_str() {
                        "<" => Ok((Some(Token::RelOp(RelOp::Lt)), None)),

                        ">" => Ok((Some(Token::RelOp(RelOp::Gt)), None)),
                        _ => {
                            panic!("FSM failed to stored data");
                        }
                    }
                }
            },

            //return state
            21 => match self._token.as_str() {
                "+" => {
                    self.state = 0;
                    Ok((Some(Token::AddOp(AddOp::Add)), None))
                }

                "-" => {
                    self.state = 0;
                    Ok((Some(Token::AddOp(AddOp::Sub)), None))
                }
                "||" => {
                    self.state = 0;
                    Ok((Some(Token::AddOp(AddOp::BoolOr)), None))
                }
                _ => {
                    self.state = 0;
                    println!("FSM failed to stored data");
                    Err(ErrorKind::CorruptState)
                }
            },

            22 => match c {
                '|' => {
                    self.state = 21;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
            },

            23 => match self._token.as_str() {
                "*" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Mul)), None))
                }
                "/" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Div)), None))
                }
                "%" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Mod)), None))
                }
                "&&" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::BoolAnd)), None))
                }
                _ => {
                    self.state = 0;
                    println!("FSM failed to stored data");
                    Err(ErrorKind::CorruptState)
                }
            },

            24 => match c {
                '&' => {
                    self.state = 23;
                    self._token.push(c);
                    Ok((None, None))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter)))
                }
            },

            25 => {
                self.state = 0;
                Ok((Some(Token::LParen), None))
            }

            26 => {
                self.state = 0;
                Ok((Some(Token::RParen), None))
            }

            27 => {
                self.state = 0;
                Ok((Some(Token::LCurly), None))
            }

            28 => {
                self.state = 0;
                Ok((Some(Token::RCurly), None))
            }

            29 => {
                self.state = 0;
                Ok((Some(Token::LBracket), None))
            }

            30 => {
                self.state = 0;
                Ok((Some(Token::RBracket), None))
            }

            31 => {
                self.state = 0;
                Ok((Some(Token::Comma), None))
            }

            32 => {
                self.state = 0;
                Ok((Some(Token::Semicolon), None))
            }

            33 => {
                self.state = 0;
                Ok((Some(Token::Not), None))
            }

            33 => {
                self.state = 0;
                Ok((Some(Token::Colon), None))
            }
            _ => {
                println!(
                    "Try matching state {}, but couldn't. Current c: \"{}\"",
                    self.state, c
                );
                Err(ErrorKind::CorruptState)
            }
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
                            self.fsm.take().unwrap(); // invalidate the FSM, abort scanning
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
