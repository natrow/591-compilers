use std::{fmt::Display, str::FromStr};

/// Keywords recognized by the scanner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Do,
    Read,
    Char,
    While,
    Continue,
    Return,
    Switch,
    Break,
    If,
    Case,
    Newline,
    Else,
    Default,
    For,
    Write,
}

impl Keyword {
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

    pub const fn to_str(self) -> &'static str {
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
}

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
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (class, attribute) = match self {
            Token::Keyword(k) => (k.to_str(), k.to_string()),
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
            Token::RCurly => ("LCURLY", "}".to_string()),
            Token::LBracket => ("LBRACKET", "[".to_string()),
            Token::RBracket => ("RBRACKET", "]".to_owned()),
            Token::Comma => ("COMMA", ",".to_owned()),
            Token::Semicolon => ("SEMICOLON", ";".to_owned()),
            Token::Not => ("NOT", "!".to_owned()),
            Token::Colon => ("COLON", ":".to_owned()),
        };
        write!(f, "(<{}>, \"{}\")", class, attribute)
    }
}
