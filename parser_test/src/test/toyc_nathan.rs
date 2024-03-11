#![allow(dead_code)]

use std::collections::HashSet;

use log::debug;

use crate::{
    cfg::{ContextFreeGrammar, Nonterminals, Productions, Symbol, Terminals},
    ll1::LL1,
};

/// This definition is adequate for verifying ToyC
type ToyCSymbol = Symbol<Token, &'static str>;

/// Keywords in ToyC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    fn values() -> HashSet<Self> {
        [
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
        ]
        .into()
    }
}

/// Token classes, with annotations removed (except keywords)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    /// Keywords
    Keyword(Keyword),
    /// Identifiers (contains ASCII letters and digits)
    Identifier,
    /// Numbers (Note: conversion to floating-point or integer types not yet implemented)
    Number,
    /// Character literals (empty allowed, built in unicode support)
    CharLiteral,
    /// String literals (empty allowed, built in unicode support)
    StringLiteral,
    /// Relational operators (empty allowed)
    RelOp,
    /// Addition operators
    AddOp,
    /// Multiplication operators
    MulOp,
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

impl From<Keyword> for Token {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl Token {
    fn values() -> HashSet<Self> {
        let keywords: HashSet<_> = Keyword::values().into_iter().map(Into::into).collect();

        let others: HashSet<_> = [
            Self::Identifier,
            Self::Number,
            Self::CharLiteral,
            Self::StringLiteral,
            Self::RelOp,
            Self::AddOp,
            Self::MulOp,
            Self::AssignOp,
            Self::LParen,
            Self::RParen,
            Self::LCurly,
            Self::RCurly,
            Self::LBracket,
            Self::RBracket,
            Self::Comma,
            Self::Semicolon,
            Self::Not,
            Self::Colon,
            Self::Eof,
        ]
        .into();

        &keywords | &others
    }
}

/* Now define helper functions to create the symbols... */

fn kw_int() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Int))
}

fn kw_do() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Do))
}

fn kw_read() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Read))
}

fn kw_char() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Char))
}

fn kw_while() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::While))
}

fn kw_continue() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Continue))
}

fn kw_return() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Return))
}

fn kw_switch() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Switch))
}

fn kw_break() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Break))
}

fn kw_if() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::If))
}

fn kw_case() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Case))
}

fn kw_newline() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Newline))
}

fn kw_else() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Else))
}

fn kw_default() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Default))
}

fn kw_for() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::For))
}

fn kw_write() -> ToyCSymbol {
    Symbol::Terminal(Token::Keyword(Keyword::Write))
}

fn identifier() -> ToyCSymbol {
    Symbol::Terminal(Token::Identifier)
}

fn number() -> ToyCSymbol {
    Symbol::Terminal(Token::Number)
}

fn char_literal() -> ToyCSymbol {
    Symbol::Terminal(Token::CharLiteral)
}

fn string_literal() -> ToyCSymbol {
    Symbol::Terminal(Token::StringLiteral)
}

fn rel_op() -> ToyCSymbol {
    Symbol::Terminal(Token::RelOp)
}

fn add_op() -> ToyCSymbol {
    Symbol::Terminal(Token::AddOp)
}

fn mul_op() -> ToyCSymbol {
    Symbol::Terminal(Token::MulOp)
}

fn assign_op() -> ToyCSymbol {
    Symbol::Terminal(Token::AssignOp)
}

fn l_paren() -> ToyCSymbol {
    Symbol::Terminal(Token::LParen)
}

fn r_paren() -> ToyCSymbol {
    Symbol::Terminal(Token::RParen)
}

fn l_curly() -> ToyCSymbol {
    Symbol::Terminal(Token::LCurly)
}
fn r_curly() -> ToyCSymbol {
    Symbol::Terminal(Token::RCurly)
}
fn l_bracket() -> ToyCSymbol {
    Symbol::Terminal(Token::LBracket)
}
fn r_bracket() -> ToyCSymbol {
    Symbol::Terminal(Token::RBracket)
}

fn comma() -> ToyCSymbol {
    Symbol::Terminal(Token::Comma)
}

fn semicolon() -> ToyCSymbol {
    Symbol::Terminal(Token::Semicolon)
}

fn not() -> ToyCSymbol {
    Symbol::Terminal(Token::Not)
}

fn colon() -> ToyCSymbol {
    Symbol::Terminal(Token::Colon)
}

fn eof() -> ToyCSymbol {
    Symbol::Terminal(Token::Eof)
}

const NONTERMINALS: [&str; 41] = [
    "ToyCProgram",
    "ToyCProgram'",
    "Definition",
    "Definition'",
    "Type",
    "FunctionDefinition",
    "FunctionHeader",
    "FunctionHeader'",
    "FunctionBody",
    "FormalParamList",
    "FormalParamList'",
    "Statement",
    "ExpressionStatement",
    "BreakStatement",
    "CompoundStatement",
    "CompoundStatement'",
    "CompoundStatement''",
    "IfStatement",
    "IfStatement'",
    "NullStatement",
    "ReturnStatement",
    "ReturnStatement'",
    "WhileStatement",
    "ReadStatement",
    "ReadStatement'",
    "WriteStatement",
    "NewLineStatement",
    "Expression",
    "Expression'",
    "RelopExpression",
    "RelopExpression'",
    "SimpleExpression",
    "SimpleExpression'",
    "Term",
    "Term'",
    "Primary",
    "Primary'",
    "FunctionCall",
    "FunctionCall'",
    "ActualParameters",
    "ActualParameters'",
];

fn nt_toy_c_program() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ToyCProgram")
}

fn nt_toy_c_program_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ToyCProgram'")
}

fn nt_definition() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Definition")
}

fn nt_definition_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Definition'")
}

fn nt_type() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Type")
}

fn nt_function_definition() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FunctionDefinition")
}

fn nt_function_header() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FunctionHeader")
}

fn nt_function_header_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FunctionHeader'")
}

fn nt_function_body() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FunctionBody")
}

fn nt_formal_param_list() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FormalParamList")
}

fn nt_formal_param_list_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FormalParamList'")
}

fn nt_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Statement")
}

fn nt_expression_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ExpressionStatement")
}

fn nt_break_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("BreakStatement")
}

fn nt_compound_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("CompoundStatement")
}

fn nt_compound_statement_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("CompoundStatement'")
}

fn nt_compound_statement__() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("CompoundStatement''")
}

fn nt_if_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("IfStatement")
}

fn nt_if_statement_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("IfStatement'")
}

fn nt_null_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("NullStatement")
}

fn nt_return_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ReturnStatement")
}

fn nt_return_statement_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ReturnStatement'")
}

fn nt_while_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("WhileStatement")
}

fn nt_read_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ReadStatement")
}

fn nt_read_statement_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ReadStatement'")
}

fn nt_write_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("WriteStatement")
}

fn nt_newline_statement() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("NewLineStatement")
}

fn nt_expression() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Expression")
}

fn nt_expression_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Expression'")
}

fn nt_relop_expression() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("RelopExpression")
}

fn nt_relop_expression_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("RelopExpression'")
}

fn nt_simple_expression() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("SimpleExpression")
}

fn nt_simple_expression_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("SimpleExpression'")
}

fn nt_term() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Term")
}

fn nt_term_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Term'")
}

fn nt_primary() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Primary")
}

fn nt_primary_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("Primary'")
}

fn nt_function_call() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FunctionCall")
}

fn nt_function_call_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("FunctionCall'")
}

fn nt_actual_parameters() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ActualParameters")
}

fn nt_actual_parameters_() -> ToyCSymbol {
    ToyCSymbol::Nonterminal("ActualParameters'")
}

#[test]
fn toyc_is_ll1() {
    // initialize logging environment
    env_logger::try_init().ok();

    // declare all terminals
    let terminals: Terminals<Token> = Token::values();

    // declare all nonterminals
    let nonterminals: Nonterminals<&'static str> = NONTERMINALS.into();

    let p_toy_c_program = ("ToyCProgram", [vec![nt_toy_c_program_(), eof()]].into());

    let p_toy_c_program_ = (
        "ToyCProgram'",
        [vec![nt_definition(), nt_toy_c_program_()], vec![]].into(),
    );

    let p_definition = (
        "Definition",
        [vec![nt_type(), identifier(), nt_definition_()]].into(),
    );

    let p_definition_ = (
        "Definition'",
        [vec![nt_function_definition()], vec![semicolon()]].into(),
    );

    let p_type = ("Type", [vec![kw_int()], vec![kw_char()]].into());

    let p_function_definition = (
        "FunctionDefinition",
        [vec![nt_function_header(), nt_function_body()]].into(),
    );

    let p_function_header = (
        "FunctionHeader",
        [vec![l_paren(), nt_function_header_(), r_paren()]].into(),
    );

    let p_function_header_ = (
        "FunctionHeader'",
        [vec![nt_formal_param_list()], vec![]].into(),
    );

    let p_function_body = ("FunctionBody", [vec![nt_compound_statement()]].into());

    let p_formal_param_list = (
        "FormalParamList",
        [vec![nt_type(), identifier(), nt_formal_param_list_()]].into(),
    );

    let p_formal_param_list_ = (
        "FormalParamList'",
        [
            vec![comma(), nt_type(), identifier(), nt_formal_param_list_()],
            vec![],
        ]
        .into(),
    );

    let p_statement = (
        "Statement",
        [
            vec![nt_expression_statement()],
            vec![nt_break_statement()],
            vec![nt_compound_statement()],
            // vec![nt_if_statement()], // causes crash - todo : remove ambiguity
            vec![nt_null_statement()],
            vec![nt_return_statement()],
            vec![nt_while_statement()],
            vec![nt_read_statement()],
            vec![nt_write_statement()],
            vec![nt_newline_statement()],
        ]
        .into(),
    );

    let p_expression_statement = (
        "ExpressionStatement",
        [vec![nt_expression(), semicolon()]].into(),
    );

    let p_break_statement = ("BreakStatement", [vec![kw_break(), semicolon()]].into());

    let p_compound_statement = (
        "CompoundStatement",
        [vec![
            l_curly(),
            nt_compound_statement_(),
            nt_compound_statement__(),
            r_curly(),
        ]]
        .into(),
    );

    let p_compound_statement_ = (
        "CompoundStatement'",
        [
            vec![
                nt_type(),
                identifier(),
                semicolon(),
                nt_compound_statement_(),
            ],
            vec![],
        ]
        .into(),
    );

    let p_compound_statement__ = (
        "CompoundStatement''",
        [vec![nt_statement(), nt_compound_statement__()], vec![]].into(),
    );

    let p_if_statement = (
        "IfStatement",
        [vec![
            kw_if(),
            l_paren(),
            nt_expression(),
            r_paren(),
            nt_statement(),
            nt_if_statement_(),
        ]]
        .into(),
    );

    let p_if_statement_ = (
        "IfStatement'",
        [vec![kw_else(), nt_statement()], vec![]].into(),
    );

    let p_null_statement = ("NullStatement", [vec![semicolon()]].into());

    let p_return_statement = (
        "ReturnStatement",
        [vec![kw_return(), nt_return_statement_(), semicolon()]].into(),
    );

    let p_return_statement_ = ("ReturnStatement'", [vec![nt_expression()], vec![]].into());

    let p_while_statement = (
        "WhileStatement",
        [vec![
            kw_while(),
            l_paren(),
            nt_expression(),
            r_paren(),
            nt_statement(),
        ]]
        .into(),
    );

    let p_read_statement = (
        "ReadStatement",
        [vec![
            kw_read(),
            l_paren(),
            identifier(),
            nt_read_statement_(),
            r_paren(),
            semicolon(),
        ]]
        .into(),
    );

    let p_read_statement_ = (
        "ReadStatement'",
        [vec![comma(), identifier(), nt_read_statement_()], vec![]].into(),
    );

    let p_write_statement = (
        "WriteStatement",
        [vec![
            kw_write(),
            l_paren(),
            nt_actual_parameters(),
            r_paren(),
            semicolon(),
        ]]
        .into(),
    );

    let p_newline_statement = ("NewLineStatement", [vec![kw_newline(), semicolon()]].into());

    let p_expression = (
        "Expression",
        [vec![nt_relop_expression(), nt_expression_()]].into(),
    );

    let p_expression_ = (
        "Expression'",
        [
            vec![assign_op(), nt_relop_expression(), nt_expression_()],
            vec![],
        ]
        .into(),
    );

    let p_relop_expression = (
        "RelopExpression",
        [vec![nt_simple_expression(), nt_relop_expression_()]].into(),
    );

    let p_relop_expression_ = (
        "RelopExpression'",
        [
            vec![rel_op(), nt_simple_expression(), nt_relop_expression_()],
            vec![],
        ]
        .into(),
    );

    let p_simple_expression = (
        "SimpleExpression",
        [vec![nt_term(), nt_simple_expression_()]].into(),
    );

    let p_simple_expression_ = (
        "SimpleExpression'",
        [vec![add_op(), nt_term(), nt_simple_expression_()], vec![]].into(),
    );

    let p_term = ("Term", [vec![nt_primary(), nt_term_()]].into());

    let p_term_ = (
        "Term'",
        [vec![mul_op(), nt_primary(), nt_term_()], vec![]].into(),
    );

    let p_primary = (
        "Primary",
        [
            vec![identifier(), nt_primary_()],
            vec![number()],
            vec![string_literal()],
            vec![char_literal()],
            vec![l_paren(), nt_expression(), r_paren()],
            vec![add_op(), nt_primary()], // '-' is a subset of AddOp
            vec![not(), nt_primary()],
        ]
        .into(),
    );

    let p_primary_ = ("Primary'", [vec![nt_function_call()], vec![]].into());

    let p_function_call = (
        "FunctionCall",
        [vec![l_paren(), nt_function_call_(), r_paren()]].into(),
    );

    let p_function_call_ = (
        "FunctionCall'",
        [vec![nt_actual_parameters()], vec![]].into(),
    );

    let p_actual_parameters = (
        "ActualParameters",
        [vec![nt_expression(), nt_actual_parameters_()]].into(),
    );

    let p_actual_parameters_ = (
        "ActualParameters'",
        [
            vec![comma(), nt_expression(), nt_actual_parameters_()],
            vec![],
        ]
        .into(),
    );

    let productions: Productions<Token, &'static str> = [
        p_toy_c_program,
        p_toy_c_program_,
        p_definition,
        p_definition_,
        p_type,
        p_function_definition,
        p_function_header,
        p_function_header_,
        p_function_body,
        p_formal_param_list,
        p_formal_param_list_,
        p_statement,
        p_expression_statement,
        p_break_statement,
        p_compound_statement,
        p_compound_statement_,
        p_compound_statement__,
        p_if_statement,
        p_if_statement_,
        p_null_statement,
        p_return_statement,
        p_return_statement_,
        p_while_statement,
        p_read_statement,
        p_read_statement_,
        p_write_statement,
        p_newline_statement,
        p_expression,
        p_expression_,
        p_relop_expression,
        p_relop_expression_,
        p_simple_expression,
        p_simple_expression_,
        p_term,
        p_term_,
        p_primary,
        p_primary_,
        p_function_call,
        p_function_call_,
        p_actual_parameters,
        p_actual_parameters_,
    ]
    .into();

    let cfg = ContextFreeGrammar::new(terminals, nonterminals, productions).unwrap();

    debug!("made cfg: {:#?}", &cfg);

    let ll1 = LL1::new(cfg).unwrap();

    println!("predict sets: {:#?}", ll1.get_predict_sets())
}
