//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use std::ops::{Mul, Sub};

use crate::scanner::token::{Keyword, Token};

/// Identifiers, which are represented as strings
pub type Identifier = String;

pub type Number = String;
/// Variable definitions, which include a list of identifiers and a type
pub type VarDef = (Vec<Identifier>, Type);

impl TryFrom<Token> for Identifier {
    type Error = ();
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Token::Identifier(id) = value {
            Ok(id)
        } else {
            Err(())
        }
    }
}

/// The program, aka the top level of the AST
#[derive(Debug)]
pub struct Program(pub Vec<Definition>);

/// Definitions allowed in the AST
#[derive(Debug)]
pub enum Definition {
    /// a function definition
    Func(Identifier, Type, Vec<VarDef>, Statement),
    /// a variable definition
    Var(Vec<Identifier>, Type),
}

/// All statements allowed in the AST
///
/// Note: sub-statements must be heap-allocated to prevent infinitely sized types
#[derive(Debug)]
pub enum Statement {
    /// An expression statement
    Expr(Expression),
    /// A break statement
    Break,
    /// A block with variable definitions and more statements
    Block(Vec<VarDef>, Vec<Statement>),
    /// An if statement
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    /// A null statement
    Null,
    /// A return statement
    Return(Option<Expression>),
    /// A while statement
    While(Expression, Box<Statement>),
    /// A read statement
    Read(Vec<Identifier>),
    /// A write statement
    Write(Vec<Expression>),
    /// A newline statement
    Newline,
}

/// All expressions allowed in the AST
///
/// Note: sub-expressions must be heap-allocated to prevent infinitely sized types
#[derive(Debug)]
pub enum Expression {
    /// A number
    Number(String),
    /// An identifier
    Identifier(Identifier),
    /// A character literal
    CharLiteral(Option<char>),
    /// A string literal
    StringLiteral(String),
    /// A function call, including an identifier and a list of input expressions
    FuncCall(Identifier, Vec<Expression>),
    /// A binary operation with a left and right hand sub-expression
    Expr(Operator, Box<Expression>, Box<Expression>),
    /// Unary negation on numbers
    Minus(Box<Expression>),
    /// Unary negation on booleans
    Not(Box<Expression>),
}


/// All binary operations allowed in the AST
#[derive(Debug)]
pub enum Operator {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// ||
    BoolOr,
    /// &&
    BoolAnd,
    /// <=
    LtEq,
    /// <
    Lt,
    /// ==
    Eq,
    /// >
    Gt,
    /// >=
    GtEq,
    /// !=
    Neq,
    /// =
    Assign
}

/// Types allowed in the AST
#[derive(Debug)]
pub enum Type {
    /// Integers
    Int,
    /// Characters
    Char,
}

impl TryFrom<Token> for Type {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Keyword(Keyword::Int) => Ok(Type::Int),
            Token::Keyword(Keyword::Char) => Ok(Type::Char),
            _ => Err(()),
        }
    }
}
