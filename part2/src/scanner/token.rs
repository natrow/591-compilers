//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use std::{fmt::Display, str::FromStr};

/// Keywords recognized by the scanner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    /// int
    Int,
    /// do
    Do,
    /// read
    Read,
    /// char
    Char,
    /// while
    While,
    /// continue
    Continue,
    /// return
    Return,
    /// switch
    Switch,
    /// break
    Break,
    /// if
    If,
    /// case
    Case,
    /// newline
    Newline,
    /// else
    Else,
    /// default
    Default,
    /// for
    For,
    /// write
    Write,
}

impl Keyword {
    /// Array of all possible enum values (used for lookups)
    pub const VALUES: [Self; 16] = [
        Self::Int,
        Self::Do,
        Self::Read,
        Self::Char,
        Self::While,
        Self::Continue,
        Self::Return,
        Self::Switch,
        Self::Break,
        Self::If,
        Self::Case,
        Self::Newline,
        Self::Else,
        Self::Default,
        Self::For,
        Self::Write,
    ];

    /// Convert keyword into string literal (static allocation)
    pub const fn to_str(&self) -> &'static str {
        match self {
            Keyword::Int => "int",
            Keyword::Do => "do",
            Keyword::Read => "read",
            Keyword::Char => "char",
            Keyword::While => "while",
            Keyword::Continue => "continue",
            Keyword::Return => "return",
            Keyword::Switch => "switch",
            Keyword::Break => "break",
            Keyword::If => "if",
            Keyword::Case => "case",
            Keyword::Newline => "newline",
            Keyword::Else => "else",
            Keyword::Default => "default",
            Keyword::For => "for",
            Keyword::Write => "write",
        }
    }

    /// Convert keyword into upper-case string literal (static allocation)
    pub const fn to_upper(&self) -> &'static str {
        match self {
            Keyword::Int => "INT",
            Keyword::Do => "DO",
            Keyword::Read => "READ",
            Keyword::Char => "CHAR",
            Keyword::While => "WHILE",
            Keyword::Continue => "CONTINUE",
            Keyword::Return => "RETURN",
            Keyword::Switch => "SWITCH",
            Keyword::Break => "BREAK",
            Keyword::If => "IF",
            Keyword::Case => "CASE",
            Keyword::Newline => "NEWLINE",
            Keyword::Else => "ELSE",
            Keyword::Default => "DEFAULT",
            Keyword::For => "FOR",
            Keyword::Write => "WRITE",
        }
    }
}

/// Attempt to convert a string slice into a keyword by doing a search.
///
/// O(n) array search - fine for such a short list.
///
/// `HashMaps` cannot (yet) be statically allocated without the use of dependencies.
impl FromStr for Keyword {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::VALUES
            .iter()
            .find(|value| value.to_str() == s)
            .copied()
            .ok_or(())
    }
}

/// Used to print a keyword OR convert it into a heap-allocated `String`
impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: &'static str = self.to_str();

        write!(f, "{}", str)
    }
}

/// Relational operators recognized by the scanner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelOp {
    /// ==
    Eq,
    /// !=
    Neq,
    /// <
    Lt,
    /// <=
    LtEq,
    /// >=
    GtEq,
    /// >
    Gt,
}

impl Display for RelOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RelOp::Eq => "==",
            RelOp::Neq => "!=",
            RelOp::Lt => "<",
            RelOp::LtEq => "<=",
            RelOp::GtEq => ">=",
            RelOp::Gt => ">",
        };

        write!(f, "{}", str)
    }
}

/// Addition operators recognized by the scanner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddOp {
    /// +
    Add,
    /// -
    Sub,
    /// ||
    BoolOr,
}

impl Display for AddOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AddOp::Add => "+",
            AddOp::Sub => "-",
            AddOp::BoolOr => "||",
        };

        write!(f, "{}", str)
    }
}

/// Multiplication operators recognized by the scanner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulOp {
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// &&
    BoolAnd,
}

impl Display for MulOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MulOp::Mul => "*",
            MulOp::Div => "/",
            MulOp::Mod => "%",
            MulOp::BoolAnd => "&&",
        };

        write!(f, "{}", str)
    }
}

/// All token classes recognized by the scanner (and their annotations)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Keywords
    Keyword(Keyword),
    /// Identifiers (contains ASCII letters and digits)
    Identifier(String),
    /// Numbers (Note: conversion to floating-point or integer types not yet implemented)
    Number(String),
    /// Character literals (empty allowed, built in unicode support)
    CharLiteral(Option<char>),
    /// String literals (empty allowed, built in unicode support)
    StringLiteral(String),
    /// Relational operators (empty allowed)
    RelOp(RelOp),
    /// Addition operators
    AddOp(AddOp),
    /// Multiplication operators
    MulOp(MulOp),
    /// =
    AssignOp,
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LCurly,
    /// }
    RCurly,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// !
    Not,
    /// :
    Colon,
    /// End of File
    Eof,
}

impl Token {
    /// Determines whether two tokens are syntactically equivalent.
    ///
    /// This ignores the attributes of tokens other than keywords and operators.
    pub fn syntax_eq(&self, rhs: &Token) -> bool {
        match (self, rhs) {
            (Token::Keyword(l), Token::Keyword(r)) => l == r,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::CharLiteral(_), Token::CharLiteral(_)) => true,
            (Token::StringLiteral(_), Token::StringLiteral(_)) => true,
            (Token::RelOp(l), Token::RelOp(r)) => l == r,
            (Token::AddOp(l), Token::AddOp(r)) => l == r,
            (Token::MulOp(l), Token::MulOp(r)) => l == r,
            (Token::AssignOp, Token::AssignOp) => true,
            (Token::LParen, Token::LParen) => true,
            (Token::RParen, Token::RParen) => true,
            (Token::LCurly, Token::LCurly) => true,
            (Token::RCurly, Token::RCurly) => true,
            (Token::LBracket, Token::LBracket) => true,
            (Token::RBracket, Token::RBracket) => true,
            (Token::Comma, Token::Comma) => true,
            (Token::Semicolon, Token::Semicolon) => true,
            (Token::Not, Token::Not) => true,
            (Token::Colon, Token::Colon) => true,
            (Token::Eof, Token::Eof) => true,
            (_, _) => false,
        }
    }

    /// Convert the token into string literal (static allocation)
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::Keyword(k) => k.to_str(),
            Self::Identifier(_) => "<identifier>",
            Self::Number(_) => "<number>",
            Self::CharLiteral(_) => "<char literal>",
            Self::StringLiteral(_) => "<string literal>",
            Self::RelOp(op) => match op {
                RelOp::Eq => "==",
                RelOp::Gt => ">",
                RelOp::GtEq => ">=",
                RelOp::Lt => "<",
                RelOp::LtEq => "<=",
                RelOp::Neq => "!=",
            },
            Self::AddOp(op) => match op {
                AddOp::Add => "+",
                AddOp::Sub => "-",
                AddOp::BoolOr => "||",
            },
            Self::MulOp(op) => match op {
                MulOp::BoolAnd => "&&",
                MulOp::Div => "/",
                MulOp::Mod => "%",
                MulOp::Mul => "*",
            },
            Self::AssignOp => "=",
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LCurly => "{",
            Self::RCurly => "}",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::Comma => "','",
            Self::Semicolon => ";",
            Self::Not => "!",
            Self::Colon => ":",
            Self::Eof => "<EOF>",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (class, attribute) = match self {
            Token::Keyword(k) => (k.to_upper(), k.to_string()),
            Token::Identifier(s) => ("ID", s.clone()),
            Token::Number(s) => ("NUMBER", s.clone()),
            Token::CharLiteral(c) => (
                "CHARLITERAL",
                c.map(|c| c.to_string()).unwrap_or(String::new()),
            ),
            Token::StringLiteral(s) => ("STRING", s.clone()),
            Token::RelOp(k) => ("RELOP", k.to_string()),
            Token::AddOp(k) => ("ADDOP", k.to_string()),
            Token::MulOp(k) => ("MULOP", k.to_string()),
            Token::AssignOp => ("ASSIGNOP", "=".to_string()),
            Token::LParen => ("LPAREN", "(".to_string()),
            Token::RParen => ("RPAREN", ")".to_string()),
            Token::LCurly => ("LCURLY", "{".to_string()),
            Token::RCurly => ("RCURLY", "}".to_string()),
            Token::LBracket => ("LBRACKET", "[".to_string()),
            Token::RBracket => ("RBRACKET", "]".to_owned()),
            Token::Comma => ("COMMA", ",".to_owned()),
            Token::Semicolon => ("SEMICOLON", ";".to_owned()),
            Token::Not => ("NOT", "!".to_owned()),
            Token::Colon => ("COLON", ":".to_owned()),
            Token::Eof => ("EOF", "EOF".to_owned()),
        };
        write!(f, "(<{}>, \"{}\")", class, attribute)
    }
}
