//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan

use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    str::FromStr,
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
    token: String,
    /// Current number of nested comment tags
    comment_level: usize,
}

impl ScannerFsm {
    /// Implementation of the DFA transitions.
    ///
    /// Can return an error, or a pair of an optional token and optional warning.
    /// Also returns a bool indicating whether the token must be re-scanned. For example,
    /// when an acceptor state returns a token but the current character wasn't used.
    ///
    /// Note: accepting states only return a value on *the next edge*.
    fn step(&mut self, c: char) -> Result<(Option<Token>, Option<WarningKind>, bool), ErrorKind> {
        // FSM implementation
        match self.state {
            0 => {
                if c.is_ascii_whitespace() {
                    Ok((None, None, false))
                } else {
                    match c {
                        '/' => {
                            self.state = 1;
                            Ok((None, None, false))
                        }
                        'A'..='Z' | 'a'..='z' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 5;
                            Ok((None, None, false))
                        }
                        '0'..='9' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 6;
                            Ok((None, None, false))
                        }
                        '\'' => {
                            self.state = 12;
                            self.token.clear();
                            Ok((None, None, false))
                        }
                        '"' => {
                            self.state = 15;
                            self.token.clear();
                            Ok((None, None, false))
                        }
                        '=' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 17;
                            Ok((None, None, false))
                        }
                        '!' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 19;
                            Ok((None, None, false))
                        }
                        '<' | '>' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 20;
                            Ok((None, None, false))
                        }
                        '+' | '-' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 21;
                            Ok((None, None, false))
                        }
                        '|' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 22;
                            Ok((None, None, false))
                        }
                        '*' | '%' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 23;
                            Ok((None, None, false))
                        }
                        '&' => {
                            self.token.clear();
                            self.token.push(c);
                            self.state = 24;
                            Ok((None, None, false))
                        }
                        '(' => {
                            self.token.clear();
                            self.state = 25;
                            Ok((None, None, false))
                        }
                        ')' => {
                            self.token.clear();
                            self.state = 26;
                            Ok((None, None, false))
                        }
                        '{' => {
                            self.token.clear();
                            self.state = 27;
                            Ok((None, None, false))
                        }
                        '}' => {
                            self.token.clear();
                            self.state = 28;
                            Ok((None, None, false))
                        }
                        '[' => {
                            self.token.clear();
                            self.state = 29;
                            Ok((None, None, false))
                        }
                        ']' => {
                            self.token.clear();
                            self.state = 30;
                            Ok((None, None, false))
                        }
                        ',' => {
                            self.token.clear();
                            self.state = 31;
                            Ok((None, None, false))
                        }
                        ';' => {
                            self.token.clear();
                            self.state = 32;
                            Ok((None, None, false))
                        }
                        ':' => {
                            self.token.clear();
                            self.state = 33;
                            Ok((None, None, false))
                        }
                        _ => Ok((None, Some(WarningKind::IllegalCharacter), false)),
                    }
                }
            }
            1 => match c {
                '/' => {
                    self.state = 2;
                    Ok((None, None, false))
                }
                '*' => {
                    self.comment_level += 1;
                    self.state = 3;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Div)), None, true))
                }
            },
            2 => match c {
                '\n' => {
                    // discard comment & return to state 0
                    self.state = 0;
                    Ok((None, None, false))
                }
                _ => Ok((None, None, false)),
            },
            3 => match c {
                '*' => {
                    self.state = 4;
                    Ok((None, None, false))
                }
                '/' => {
                    self.state = 34;
                    Ok((None, None, false))
                }
                _ => Ok((None, None, false)),
            },
            4 => match c {
                '/' => {
                    // discard comment & return to state 0
                    // TODO - nested comments
                    self.comment_level -= 1;
                    if self.comment_level == 0 {
                        self.state = 0;
                    } else {
                        self.state = 3;
                    }
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 3;
                    Ok((None, None, false))
                }
            },
            5 => match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    self.token.push(c);
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    if let Ok(k) = Keyword::from_str(&self.token) {
                        Ok((Some(Token::Keyword(k)), None, true))
                    } else {
                        Ok((Some(Token::Identifier(self.token.clone())), None, true))
                    }
                }
            },
            6 => match c {
                '0'..='9' => {
                    self.token.push(c);
                    Ok((None, None, false))
                }
                '.' => {
                    self.token.push(c);
                    self.state = 7;
                    Ok((None, None, false))
                }
                'E' => {
                    self.token.push(c);
                    self.state = 9;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Number(self.token.clone())), None, true))
                }
            },
            7 => match c {
                '0'..='9' => {
                    self.state = 8;
                    self.token.push(c);
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter), false))
                }
            },
            8 => match c {
                '0'..='9' => {
                    self.state = 8;
                    self.token.push(c);
                    Ok((None, None, false))
                }
                'E' => {
                    self.state = 9;
                    self.token.push(c);
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Number(self.token.clone())), None, true))
                }
            },
            9 => match c {
                '0'..='9' => {
                    self.state = 11;
                    self.token.push(c);
                    Ok((None, None, false))
                }
                '+' | '-' => {
                    self.state = 10;
                    self.token.push(c);
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter), false))
                }
            },
            10 => match c {
                '0'..='9' => {
                    self.state = 11;
                    self.token.push(c);
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter), false))
                }
            },
            11 => match c {
                '0'..='9' => {
                    self.token.push(c);
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Number(self.token.clone())), None, true))
                }
            },
            12 => match c {
                '\'' => {
                    self.state = 13;
                    Ok((None, None, false))
                }
                '\n' => {
                    self.state = 0;
                    Err(ErrorKind::NewlineInCharLiteral)
                }
                _ => {
                    self.state = 14;
                    self.token.push(c);
                    Ok((None, None, false))
                }
            },
            13 => {
                self.state = 0;
                let c = self.token.chars().nth(0);
                Ok((Some(Token::CharLiteral(c)), None, true))
            }
            14 => match c {
                '\'' => {
                    self.state = 13;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Err(ErrorKind::UnclosedCharLiteral)
                }
            },
            15 => match c {
                '"' => {
                    self.state = 16;
                    Ok((None, None, false))
                }
                '\n' => Err(ErrorKind::NewlineInStringLiteral),
                _ => {
                    self.token.push(c);
                    Ok((None, None, false))
                }
            },
            16 => {
                self.state = 0;
                Ok((Some(Token::StringLiteral(self.token.clone())), None, true))
            }
            17 => match c {
                '=' => {
                    self.state = 18;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::AssignOp), None, true))
                }
            },
            18 => {
                self.state = 0;
                match self.token.as_str() {
                    "=" => Ok((Some(Token::RelOp(RelOp::Eq)), None, true)),
                    ">" => Ok((Some(Token::RelOp(RelOp::GtEq)), None, true)),
                    "<" => Ok((Some(Token::RelOp(RelOp::LtEq)), None, true)),
                    "!" => Ok((Some(Token::RelOp(RelOp::Neq)), None, true)),
                    _ => Err(ErrorKind::CorruptState),
                }
            }
            19 => match c {
                '=' => {
                    self.state = 18;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((Some(Token::Not), None, true))
                }
            },
            20 => match c {
                '=' => {
                    self.state = 18;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    match self.token.as_str() {
                        "<" => Ok((Some(Token::RelOp(RelOp::Lt)), None, true)),
                        ">" => Ok((Some(Token::RelOp(RelOp::Gt)), None, true)),
                        _ => Err(ErrorKind::CorruptState),
                    }
                }
            },
            21 => match self.token.as_str() {
                "+" => {
                    self.state = 0;
                    Ok((Some(Token::AddOp(AddOp::Add)), None, true))
                }
                "-" => {
                    self.state = 0;
                    Ok((Some(Token::AddOp(AddOp::Sub)), None, true))
                }
                "|" => {
                    self.state = 0;
                    Ok((Some(Token::AddOp(AddOp::BoolOr)), None, true))
                }
                _ => Err(ErrorKind::CorruptState),
            },
            22 => match c {
                '|' => {
                    self.state = 21;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter), false))
                }
            },
            23 => match self.token.as_str() {
                "*" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Mul)), None, true))
                }
                "/" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Div)), None, true))
                }
                "%" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::Mod)), None, true))
                }
                "&" => {
                    self.state = 0;
                    Ok((Some(Token::MulOp(MulOp::BoolAnd)), None, true))
                }
                _ => Err(ErrorKind::CorruptState),
            },
            24 => match c {
                '&' => {
                    self.state = 23;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 0;
                    Ok((None, Some(WarningKind::IllegalCharacter), false))
                }
            },
            25 => {
                self.state = 0;
                Ok((Some(Token::LParen), None, true))
            }
            26 => {
                self.state = 0;
                Ok((Some(Token::RParen), None, true))
            }
            27 => {
                self.state = 0;
                Ok((Some(Token::LCurly), None, true))
            }
            28 => {
                self.state = 0;
                Ok((Some(Token::RCurly), None, true))
            }
            29 => {
                self.state = 0;
                Ok((Some(Token::LBracket), None, true))
            }
            30 => {
                self.state = 0;
                Ok((Some(Token::RBracket), None, true))
            }
            31 => {
                self.state = 0;
                Ok((Some(Token::Comma), None, true))
            }
            32 => {
                self.state = 0;
                Ok((Some(Token::Semicolon), None, true))
            }
            33 => {
                self.state = 0;
                Ok((Some(Token::Colon), None, true))
            }
            34 => match c {
                '*' => {
                    self.comment_level += 1;
                    self.state = 3;
                    Ok((None, None, false))
                }
                _ => {
                    self.state = 3;
                    Ok((None, None, false))
                }
            },
            _ => Err(ErrorKind::CorruptState),
        }
    }

    /// Consumes the DFA and evaluates the validity of the final state.
    fn finish(self) -> Result<(Option<Token>, Option<WarningKind>), ErrorKind> {
        match self.state {
            0 => Ok((None, None)),
            1 => Ok((Some(Token::MulOp(MulOp::Div)), None)),
            2 => Ok((None, None)), // comment at the end of the file
            3 | 4 | 34 => Err(ErrorKind::UnclosedComment),
            5 => {
                if let Ok(k) = Keyword::from_str(&self.token) {
                    Ok((Some(Token::Keyword(k)), None))
                } else {
                    Ok((Some(Token::Identifier(self.token)), None))
                }
            }
            6 | 8 | 11 => Ok((Some(Token::Number(self.token)), None)),
            12 | 14 => Err(ErrorKind::UnclosedCharLiteral),
            13 => Ok((Some(Token::CharLiteral(self.token.chars().nth(0))), None)),
            15 => Err(ErrorKind::UnclosedStringLiteral),
            16 => Ok((Some(Token::StringLiteral(self.token)), None)),
            17 => Ok((Some(Token::AssignOp), None)),
            18 => match self.token.as_str() {
                "=" => Ok((Some(Token::RelOp(RelOp::Eq)), None)),
                "!" => Ok((Some(Token::RelOp(RelOp::Neq)), None)),
                "<" => Ok((Some(Token::RelOp(RelOp::LtEq)), None)),
                ">" => Ok((Some(Token::RelOp(RelOp::GtEq)), None)),
                _ => Err(ErrorKind::CorruptState),
            },
            19 => Ok((Some(Token::Not), None)),
            20 => match self.token.as_str() {
                "<" => Ok((Some(Token::RelOp(RelOp::Lt)), None)),
                ">" => Ok((Some(Token::RelOp(RelOp::Gt)), None)),
                _ => Err(ErrorKind::CorruptState),
            },
            21 => match self.token.as_str() {
                "+" => Ok((Some(Token::AddOp(AddOp::Add)), None)),
                "-" => Ok((Some(Token::AddOp(AddOp::Sub)), None)),
                "|" => Ok((Some(Token::AddOp(AddOp::BoolOr)), None)),
                _ => Err(ErrorKind::CorruptState),
            },
            23 => match self.token.as_str() {
                "*" => Ok((Some(Token::MulOp(MulOp::Mul)), None)),
                "%" => Ok((Some(Token::MulOp(MulOp::Mod)), None)),
                "&" => Ok((Some(Token::MulOp(MulOp::BoolAnd)), None)),
                _ => Err(ErrorKind::CorruptState),
            },
            25 => Ok((Some(Token::LParen), None)),
            26 => Ok((Some(Token::RParen), None)),
            27 => Ok((Some(Token::LCurly), None)),
            28 => Ok((Some(Token::RCurly), None)),
            29 => Ok((Some(Token::LBracket), None)),
            30 => Ok((Some(Token::RBracket), None)),
            31 => Ok((Some(Token::Comma), None)),
            32 => Ok((Some(Token::Semicolon), None)),
            33 => Ok((Some(Token::Colon), None)),
            35.. => Err(ErrorKind::CorruptState),
            _ => Ok((None, Some(WarningKind::IllegalCharacter))),
        }
    }
}

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
            // 1: attempt to read from current line
            'a: loop {
                // 1: attempt to read from current line
                for c in self.line[self.line_index..].chars() {
                    match fsm.step(c) {
                        Ok((t, w, r)) => {
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

                            if !r {
                                self.line_index += 1;
                            } else if t.is_none() {
                                continue 'a;
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
                }
                // 2: get next line
                match self.buffer.next() {
                    // newlines are stripped so add them back
                    Some(Ok(line)) => self.line = "\n".to_owned() + &line,
                    Some(Err(e)) => return Some(Err(self.handle_io_error(e))),

                    // 3: repeat until all lines read
                    None => break 'a,
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
