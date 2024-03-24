//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan
//!
//! Implementation note: originally we had used a boolean to represent
//! that a character must be re-scanned. It became obvious that this is
//! equivalent to `token.is_some()`, so the logic has been simplified.

use std::str::FromStr;

use crate::scanner::error::*;
use crate::scanner::token::*;

/// Scanner implemented as a finite state machine. This module is private to ensure
/// correct usage of the 'step' and 'finish' functions.
///
/// Note: This FSM keeps no track of its location in a file, only the kind of error
/// (or warning) that occurred.
#[derive(Default, Clone)]
pub struct Fsm {
    /// Current state, represented as an 8-bit unsigned integer (max value: 34)
    state: u8,
    /// Current token being scanned, used to fill attribute fields
    token: String,
    /// Current number of nested comment tags
    comment_level: usize,
}

impl Fsm {
    /// Short-hand method to update the state and return no tokens or warnings
    fn take_edge(&mut self, edge: u8) -> Result<(Option<Token>, Option<Warning>), Error> {
        self.state = edge;
        Ok((None, None))
    }

    /// Same as [Self::take_edge()] but also pushes character to the stack
    fn take_edge_and_push(
        &mut self,
        edge: u8,
        c: char,
    ) -> Result<(Option<Token>, Option<Warning>), Error> {
        self.token.push(c);
        self.take_edge(edge)
    }

    /// Resets state to 0, then returns the given token and no warnings.
    ///
    /// The caller of this function must ensure that the input character is re-scanned.
    fn return_token(&mut self, t: Token) -> Result<(Option<Token>, Option<Warning>), Error> {
        self.state = 0;
        Ok((Some(t), None))
    }

    /// Returns keyword or identifier token after lookup
    fn make_id_or_keyword(&self) -> Token {
        if let Ok(k) = Keyword::from_str(&self.token) {
            Token::Keyword(k)
        } else {
            Token::Identifier(self.token.clone())
        }
    }

    /// Returns relop after successful matching of current state
    fn make_relop(&self) -> Token {
        match (self.state, self.token.as_str()) {
            (18, "=") => Token::RelOp(RelOp::Eq),
            (18, ">") => Token::RelOp(RelOp::GtEq),
            (18, "<") => Token::RelOp(RelOp::LtEq),
            (18, "!") => Token::RelOp(RelOp::Neq),
            (20, "<") => Token::RelOp(RelOp::Lt),
            (20, ">") => Token::RelOp(RelOp::Gt),
            _ => unreachable!("make_relop() called with unknown state"),
        }
    }

    /// Returns addop after successful matching of current state
    fn make_addop(&self) -> Token {
        match self.token.as_str() {
            "+" => Token::AddOp(AddOp::Add),
            "-" => Token::AddOp(AddOp::Sub),
            "|" => Token::AddOp(AddOp::BoolOr),
            _ => unreachable!("make_addop() called with unknown state"),
        }
    }

    /// Returns mulop after successful matching of current state
    fn make_mulop(&self) -> Token {
        match self.token.as_str() {
            "*" => Token::MulOp(MulOp::Mul),
            "/" => Token::MulOp(MulOp::Div),
            "%" => Token::MulOp(MulOp::Mod),
            "&" => Token::MulOp(MulOp::BoolAnd),
            _ => unreachable!("make_mulop() called with unknown state"),
        }
    }

    /// Returns an illegal character warning and no token
    fn warn_illegal_character(&self) -> Result<(Option<Token>, Option<Warning>), Error> {
        Ok((None, Some(Warning::IllegalCharacter)))
    }

    /// Implementation of the DFA transitions.
    ///
    /// Can return an error, or a pair of an optional token and optional warning.
    ///
    /// The current character must be re-scanned iff a token is returned.
    ///
    /// Note: accepting states only return a value on *the next edge*.
    pub fn step(&mut self, c: char) -> Result<(Option<Token>, Option<Warning>), Error> {
        // FSM implementation
        match self.state {
            0 => {
                if c.is_ascii_whitespace() {
                    self.take_edge(0)
                } else {
                    // clear input token buffer
                    self.token.clear();

                    match c {
                        '/' => self.take_edge(1),                               // comments or div
                        'A'..='Z' | 'a'..='z' => self.take_edge_and_push(5, c), // id's or keywords
                        '0'..='9' => self.take_edge_and_push(6, c),             // numbers
                        '\'' => self.take_edge(12),                             // char literals
                        '"' => self.take_edge(15),                              // string literals
                        '=' => self.take_edge_and_push(17, c), // equality or assign
                        '!' => self.take_edge_and_push(19, c), // inequality
                        '<' | '>' => self.take_edge_and_push(20, c), // relop
                        '+' | '-' => self.take_edge_and_push(21, c), // addop
                        '|' => self.take_edge_and_push(22, c), // bool or
                        '*' | '%' => self.take_edge_and_push(23, c), // mulop
                        '&' => self.take_edge_and_push(24, c), // bool and
                        '(' => self.take_edge(25),             // lparen
                        ')' => self.take_edge(26),             // rparen
                        '{' => self.take_edge(27),             // lcurly
                        '}' => self.take_edge(28),             // rcurly
                        '[' => self.take_edge(29),             // lbracket
                        ']' => self.take_edge(30),             // rbracket
                        ',' => self.take_edge(31),             // comma
                        ';' => self.take_edge(32),             // semicolon
                        ':' => self.take_edge(33),             // colon
                        _ => self.warn_illegal_character(),
                    }
                }
            }
            1 => match c {
                '/' => self.take_edge(2),
                '*' => {
                    self.comment_level += 1;
                    self.take_edge(3)
                }
                _ => self.return_token(Token::MulOp(MulOp::Div)),
            },
            2 => match c {
                '\n' => self.take_edge(0),
                _ => self.take_edge(2),
            },
            3 => match c {
                '*' => self.take_edge(4),
                '/' => self.take_edge(34),
                _ => self.take_edge(3),
            },
            4 => match c {
                '/' => {
                    self.comment_level -= 1;
                    if self.comment_level == 0 {
                        self.take_edge(0)
                    } else {
                        self.take_edge(3)
                    }
                }
                _ => self.take_edge(3),
            },
            5 => match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => self.take_edge_and_push(5, c),
                _ => self.return_token(self.make_id_or_keyword()),
            },
            6 => match c {
                '0'..='9' => self.take_edge_and_push(6, c),
                '.' => self.take_edge_and_push(7, c),
                'E' => self.take_edge_and_push(9, c),
                _ => self.return_token(Token::Number(self.token.clone())),
            },
            7 => match c {
                '0'..='9' => self.take_edge_and_push(8, c),
                _ => self.warn_illegal_character(),
            },
            8 => match c {
                '0'..='9' => self.take_edge_and_push(8, c),
                'E' => self.take_edge_and_push(9, c),
                _ => self.return_token(Token::Number(self.token.clone())),
            },
            9 => match c {
                '0'..='9' => self.take_edge_and_push(11, c),
                '+' | '-' => self.take_edge_and_push(10, c),
                _ => self.warn_illegal_character(),
            },
            10 => match c {
                '0'..='9' => self.take_edge_and_push(11, c),
                _ => self.warn_illegal_character(),
            },
            11 => match c {
                '0'..='9' => self.take_edge_and_push(11, c),
                _ => self.return_token(Token::Number(self.token.clone())),
            },
            12 => match c {
                '\'' => self.take_edge(13),
                '\n' => Err(Error::NewlineInCharLiteral),
                _ => self.take_edge_and_push(14, c),
            },
            13 => self.return_token(Token::CharLiteral(self.token.chars().nth(0))),
            14 => match c {
                '\'' => self.take_edge(13),
                _ => Err(Error::UnclosedCharLiteral),
            },
            15 => match c {
                '"' => self.take_edge(16),
                '\n' => Err(Error::NewlineInStringLiteral),
                _ => self.take_edge_and_push(15, c),
            },
            16 => self.return_token(Token::StringLiteral(self.token.clone())),
            17 => match c {
                '=' => self.take_edge(18),
                _ => self.return_token(Token::AssignOp),
            },
            18 => self.return_token(self.make_relop()),
            19 => match c {
                '=' => self.take_edge(18),
                _ => self.return_token(Token::Not),
            },
            20 => match c {
                '=' => self.take_edge(18),
                _ => self.return_token(self.make_relop()),
            },
            21 => self.return_token(self.make_addop()),
            22 => match c {
                '|' => self.take_edge(21),
                _ => self.warn_illegal_character(),
            },
            23 => self.return_token(self.make_mulop()),
            24 => match c {
                '&' => self.take_edge(23),
                _ => self.warn_illegal_character(),
            },
            25 => self.return_token(Token::LParen),
            26 => self.return_token(Token::RParen),
            27 => self.return_token(Token::LCurly),
            28 => self.return_token(Token::RCurly),
            29 => self.return_token(Token::LBracket),
            30 => self.return_token(Token::RBracket),
            31 => self.return_token(Token::Comma),
            32 => self.return_token(Token::Semicolon),
            33 => self.return_token(Token::Colon),
            34 => match c {
                '*' => {
                    self.comment_level += 1;
                    self.take_edge(3)
                }
                _ => self.take_edge(3),
            },
            _ => unreachable!("step() called with unknown state"),
        }
    }

    /// Finishes DFA, returning no tokens and no warnings
    fn finish_none(self) -> Result<(Option<Token>, Option<Warning>), Error> {
        Ok((None, None))
    }

    /// Finishes DFA, returning a token and no warnings
    fn finish_token(t: Token) -> Result<(Option<Token>, Option<Warning>), Error> {
        Ok((Some(t), None))
    }

    /// Finishes DFA, returning an illegal character warning
    fn finish_illegal_char(self) -> Result<(Option<Token>, Option<Warning>), Error> {
        Ok((None, Some(Warning::IllegalCharacter)))
    }

    /// Finishes DFA, returning an error
    fn finish_err(self, e: Error) -> Result<(Option<Token>, Option<Warning>), Error> {
        Err(e)
    }

    /// Consumes the DFA and evaluates the validity of the final state.
    pub fn finish(self) -> Result<(Option<Token>, Option<Warning>), Error> {
        match self.state {
            0 => self.finish_none(),
            1 => Self::finish_token(Token::MulOp(MulOp::Div)),
            2 => self.finish_none(), // comment at the end of the file
            3 | 4 | 34 => self.finish_err(Error::UnclosedComment),
            5 => Self::finish_token(self.make_id_or_keyword()),
            6 | 8 | 11 => Self::finish_token(Token::Number(self.token)),
            12 | 14 => self.finish_err(Error::UnclosedCharLiteral),
            13 => Self::finish_token(Token::CharLiteral(self.token.chars().nth(0))),
            15 => self.finish_err(Error::UnclosedStringLiteral),
            16 => Self::finish_token(Token::StringLiteral(self.token)),
            17 => Self::finish_token(Token::AssignOp),
            18 | 20 => Self::finish_token(self.make_relop()),
            19 => Self::finish_token(Token::Not),
            21 => Self::finish_token(self.make_addop()),
            23 => Self::finish_token(self.make_mulop()),
            25 => Self::finish_token(Token::LParen),
            26 => Self::finish_token(Token::RParen),
            27 => Self::finish_token(Token::LCurly),
            28 => Self::finish_token(Token::RCurly),
            29 => Self::finish_token(Token::LBracket),
            30 => Self::finish_token(Token::RBracket),
            31 => Self::finish_token(Token::Comma),
            32 => Self::finish_token(Token::Semicolon),
            33 => Self::finish_token(Token::Colon),
            35.. => unreachable!("finish() called with unknown state"),
            _ => self.finish_illegal_char(),
        }
    }
}
