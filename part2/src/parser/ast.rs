//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use crate::scanner::token::{AddOp, Keyword, MulOp, RelOp, Token};

mod printing;

/// Identifiers, which are represented as strings
pub type Identifier = String;

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

impl TryFrom<Token> for Expression {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Number(x) => Ok(Expression::Number(x)),
            Token::CharLiteral(x) => Ok(Expression::CharLiteral(x)),
            Token::StringLiteral(x) => Ok(Expression::StringLiteral(x)),
            _ => Err(()),
        }
    }
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
    Assign,
}

impl From<AddOp> for Operator {
    fn from(value: AddOp) -> Self {
        match value {
            AddOp::Add => Operator::Add,
            AddOp::BoolOr => Operator::BoolOr,
            AddOp::Sub => Operator::Sub,
        }
    }
}

impl From<MulOp> for Operator {
    fn from(value: MulOp) -> Self {
        match value {
            MulOp::Mul => Operator::Mul,
            MulOp::Div => Operator::Div,
            MulOp::Mod => Operator::Mod,
            MulOp::BoolAnd => Operator::BoolAnd,
        }
    }
}

impl From<RelOp> for Operator {
    fn from(value: RelOp) -> Self {
        match value {
            RelOp::Eq => Operator::Eq,
            RelOp::Neq => Operator::Neq,
            RelOp::Lt => Operator::Lt,
            RelOp::LtEq => Operator::LtEq,
            RelOp::GtEq => Operator::GtEq,
            RelOp::Gt => Operator::Gt,
        }
    }
}

impl TryFrom<Token> for Operator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::AddOp(x) => Ok(x.into()),
            Token::MulOp(x) => Ok(x.into()),
            Token::RelOp(x) => Ok(x.into()),
            Token::AssignOp => Ok(Operator::Assign),
            _ => Err(()),
        }
    }
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
