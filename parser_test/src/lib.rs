//! Test crate for experimenting with LL(1) grammars and parsers.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod cfg;
pub mod ll1;
pub mod token;

#[cfg(test)]
mod test1 {
    use std::collections::HashSet;

    use log::debug;

    use crate::{
        cfg::{ContextFreeGrammar, Symbol},
        ll1::LL1,
        token::{self, Keyword, Token},
    };

    /// toyc LL(1) test
    mod toyc;

    //into for test 1
    // impl From<&'static str> for Symbol<char, &'static str> {
    //     fn from(value: &'static str) -> Self {
    //         Self::Nonterminal(value)
    //     }
    // }

    // impl From<char> for Symbol<char, &'static str> {
    //     fn from(value: char) -> Self {
    //         Self::Terminal(value)
    //     }
    // }

    //into for test2
    impl From<&'static str> for Symbol<token::Token, &'static str> {
        fn from(value: &'static str) -> Self {
            Self::Nonterminal(value)
        }
    }

    impl From<token::Token> for Symbol<token::Token, &'static str> {
        fn from(value: token::Token) -> Self {
            Self::Terminal(value)
        }
    }

    #[test]
    fn is_ll1_part2() {
        env_logger::try_init().ok();
        //Expr = Expression and stateMnt = statement
        // debug!("Evaluating grammar:");
        // debug!("ToyCProgram -> {{ Definition }} EOF");
        // debug!("Def -> Type id ( FunctionDefinition | ; )");
        // debug!("Type -> int | char");
        // debug!("Func_Def -> Func_Header Func_Body");
        // debug!("Func_Header -> ({{formalParList}})");
        // debug!("Func_body' -> Comp_Statement");
        // debug!("formalParList -> Type id {{, Type id}}");
        // debug!(
        //     "Statement ->  ExpressionStatement\n
        // | Break_Statement\n
        // | Comp_Statement\n
        // | if_Statement\n
        // | Null_Statement\n
        // | Return_Statement\n
        // | while_Statement\n
        // | Return_Statement\n
        // | Write_Statement\n
        // | NL_stateMnt"
        // );
        // debug!("Expr_Statement-> Expr");
        // debug!("Break_Statement-> break");
        // debug!("Comp_Statement -> {{ {{ Type id ; }} {{Statement }} }}");
        // debug!("if_Statement-> if ( Expr ) Statement [ else Statement ]");
        // debug!("Null_Statement-> ;");
        // debug!("Return_Statement ->  return [ Expr ] ;");
        // debug!("while_Statement ->  while ( Expr ) Statement");
        // debug!("Read_Statement->  read ( id {{ , id }} ) ");
        // debug!("Write_Statement -> write ( ActualParameters ) ;");
        // debug!("NL_stateMnt -> newline ;");
        // debug!("Expr -> RelopExpr {{ assignop RelopExpr }}");
        // debug!("Relep_Expr -> SimpleExpr {{ relop SimpleExpr }}");
        // debug!("Simple_Expr -> Term {{ addop Term }}");
        // debug!("Term -> Primary {{ mulop Primary }}");
        // debug!(
        //     "Primary -> id [ FunctionCall ]
        // | number
        // | stringConstant
        // | charConstant
        // | ( Expr )
        // | ( - | not ) Primary"
        // );
        // debug!("FuncCall -> ( [ ActualParam ] )");
        // debug!("ActualParam -> Expr {{ , Expr }}");

        let nonterminals: HashSet<&str> = [
            "ToyCProgram",
            "ToyCProgram'",
            "Definition",
            "Type",
            "FunctionDefinition",
            "FunctionHeader",
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
            "FunctionCall",
            "ActualParameters",
            "ActualParameters'",
        ]
        .into();

        let terminals: HashSet<Token> = [
            (Token::Eof),
            (Token::Keyword(Keyword::Int)),
            (Token::Keyword(Keyword::Char)),
            (Token::LParen),
            (Token::RParen),
            (Token::LCurly),
            (Token::RCurly),
            (Token::Identifier),
            (Token::Comma),
            (Token::Semicolon),
            (Token::Keyword(Keyword::Break)),
            (Token::LBracket),
            (Token::RBracket),
            (Token::Keyword(Keyword::If)),
            (Token::Keyword(Keyword::Else)),
            (Token::Keyword(Keyword::Return)),
            (Token::Keyword(Keyword::While)),
            (Token::Keyword(Keyword::Read)),
            (Token::Keyword(Keyword::Write)),
            (Token::Keyword(Keyword::Newline)),
            (Token::AssignOp),
            (Token::AddOp(token::AddOp::DNC)),
            (Token::RelOp),
            (Token::MulOp),
            (Token::Number),
            (Token::StringLiteral),
            (Token::CharLiteral),
            (Token::AddOp(token::AddOp::Sub)),
            (Token::Not),
        ]
        .into();

        let toyCProgram: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["ToyCProgram'".into(), (Token::Eof).into()]].into();

        let toyCProgramP: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["Definition".into(), "ToyCProgram'".into()], vec![]].into();

        //Definition → Type identifier FunctionDefinition | Type identifier;
        let definition: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                "Type".into(),
                Token::Identifier.into(),
                "FunctionDefinition".into(),
            ],
            vec![
                "Type".into(),
                Token::Identifier.into(),
                Token::Semicolon.into(),
            ],
        ]
        .into();

        //Type → int | char
        let types: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            //this has to be call "types" because "types" in keyword in Rust
            vec![Token::Keyword(Keyword::Int).into()],
            vec![Token::Keyword(Keyword::Char).into()],
        ]
        .into();

        //FunctionDefinition → FunctionHeader FunctionBody
        let function_definition: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["FunctionHeader".into(), "FunctionBody".into()]].into();

        //FunctionHeader → () | (FormalParamList)
        let function_header: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![Token::RParen.into(), Token::LParen.into()],
            vec![
                Token::RParen.into(),
                "FormalParamList".into(),
                Token::LParen.into(),
            ],
        ]
        .into();

        //FunctionBody → CompoundStatement
        let function_body: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["CompoundStatement".into()]].into();

        //FormalParamList → Type identifier FormalParamList′
        let formal_param_list: HashSet<Vec<Symbol<token::Token, &'static str>>> = [vec![
            "Type".into(),
            Token::Identifier.into(),
            "FormalParamList'".into(),
        ]]
        .into();

        //FormalParamList′ → , Type identifier FormalParamList′ | ε
        let formal_param_list_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::Comma.into(),
                "Type".into(),
                "FormalParamList'".into(),
            ],
            vec![],
        ]
        .into();
        //ExpressionStatement → Expression;
        // /Statement → ExpressionStatement
        // | BreakStatement
        // | CompoundStatement
        // | IfStatement
        // | NullStatement
        // | ReturnStatement
        // | WhileStatement
        // | ReadStatement
        // | WriteStatement
        // | NewLineStatement

        let statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec!["ExpressionStatement".into()],
            vec!["BreakStatement".into()],
            vec!["CompoundStatement".into()],
            vec!["IfStatement".into()],
            vec!["NullStatement".into()],
            vec!["ReturnStatement".into()],
            vec!["WhileStatement".into()],
            vec!["ReadStatement".into()],
            vec!["WriteStatement".into()],
            vec!["NewLineStatement".into()],
        ]
        .into();

        //ExpressionStatement → Expression;
        let expression_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["Expression".into()]].into();

        // BreakStatement → break;
        let break_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec![token::Token::Keyword(Keyword::Break).into()]].into();

        //CompoundStatement → {CompoundStatement′ CompoundStatement′′ }
        let compound_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [vec![
            Token::LCurly.into(),
            "CompoundStatement'".into(),
            "CompoundStatement''".into(),
            Token::RCurly.into(),
        ]]
        .into();

        //CompoundStatement′ → Type identifier ; CompoundStatement′ | ε
        let compound_statement_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                "Type".into(),
                Token::Identifier.into(),
                Token::Semicolon.into(),
                "CompoundStatement'".into(),
            ],
            vec![],
        ]
        .into();

        //CompoundStatement′′ → Statement CompoundStatement′′ | ε
        let compound_statement_p_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec!["Statement".into(), "CompoundStatement''".into()],
            vec![],
        ]
        .into();

        //IfStatement → if (Expression) Statement IfStatement′
        let if_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [vec![
            Token::Keyword(Keyword::If).into(),
            Token::LParen.into(),
            "Expression".into(),
            Token::RParen.into(),
            "Statement".into(),
            "IfStatement'".into(),
        ]]
        .into();

        //IfStatement′ → else Statement | ε
        let if_statement_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![Token::Keyword(Keyword::Else).into(), "Statement".into()],
            vec![],
        ]
        .into();

        let null_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec![Token::Semicolon.into()]].into();

        let return_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::Keyword(Keyword::Break).into(),
                "Expression".into(),
                Token::Comma.into(),
            ],
            vec![],
        ]
        .into();

        //WhileStatement → while (Expression) Statement
        let while_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [vec![
            Token::Keyword(Keyword::While).into(),
            Token::LParen.into(),
            "Expression".into(),
            Token::RParen.into(),
            "Statement".into(),
        ]]
        .into();

        //ReadStatement → read (identifier ReadStatement′ );
        let read_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [vec![
            Token::Keyword(Keyword::Read).into(),
            Token::LParen.into(),
            Token::Identifier.into(),
            "ReadStatement'".into(),
            Token::RParen.into(),
            Token::Semicolon.into(),
        ]]
        .into();

        // ReadStatement′ → , identifier ReadStatement′ | ε
        let read_statement_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::Comma.into(),
                Token::Identifier.into(),
                "ReadStatement'".into(),
            ],
            vec![],
        ]
        .into();

        //WriteStatement → write (ActualParameters) ;
        let write_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> = [vec![
            Token::Keyword(Keyword::Write).into(),
            Token::LParen.into(),
            "ActualParameters".into(),
            Token::RParen.into(),
            Token::Semicolon.into(),
        ]]
        .into();

        //NewLineStatement → newline;
        let new_line_statement: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec![Token::Keyword(Keyword::Newline).into()]].into();

        //Expression → RelopExpression Expression′
        let expression: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["RelopExpression".into(), "Expression".into()]].into();

        //Expression′ → assignop RelopExpression Expression′ | ε
        let expression_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::AssignOp.into(),
                "RelopExpression".into(),
                "Expression'".into(),
            ],
            vec![],
        ]
        .into();

        //RelopExpression → SimpleExpression RelopExpression′
        let relop_expression: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["SimpleExpression".into(), "RelopExpression'".into()]].into();

        //RelopExpression′ → relop SimpleExpression RelopExpression′ | ε<
        let relop_expression_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::RelOp.into(),
                "SimpleExpression".into(),
                "RelopExpression'".into(),
            ],
            vec![],
        ]
        .into();

        //SimpleExpression → Term SimpleExpression′
        let simple_expression: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["Term".into(), "SimpleExpression'".into()]].into();

        //SimpleExpression′ → addop Term SimpleExpression′ | ε
        let simple_expression_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::AddOp(token::AddOp::DNC).into(),
                "Term".into(),
                "SimpleExpression'".into(),
            ],
            vec![],
        ]
        .into();

        //Term → Primary Term′
        let term: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["Primary".into(), "Term'".into()]].into();

        //Term′ → mulop Primary Term′ | ε
        let term_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![Token::MulOp.into(), "Primary".into(), "Term'".into()],
            vec![],
        ]
        .into();

        //Primary → identifier FunctionCall | ε
        // | number
        // | stringConstant
        // | charConstant
        // | (Expression)
        // | - Primary| not Primary
        let Primary: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![Token::Identifier.into(), "FunctionCall".into()],
            vec![Token::Identifier.into()],
            vec![Token::Number.into()],
            vec![Token::StringLiteral.into()],
            vec![Token::CharLiteral.into()],
            vec![
                Token::LParen.into(),
                "Expression".into(),
                Token::RParen.into(),
            ],
            vec![Token::AddOp(token::AddOp::Sub).into(), "Primary".into()],
            vec![Token::Not.into(), "Primary".into()],
        ]
        .into();

        //FunctionCall → (ActualParameters | ε)
        let function_call: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::LParen.into(),
                "ActualParameters".into(),
                Token::RParen.into(),
            ],
            vec![Token::LParen.into(), Token::RParen.into()],
        ]
        .into();

        //ActualParameters → Expression ActualParameters′
        let actual_parameters: HashSet<Vec<Symbol<token::Token, &'static str>>> =
            [vec!["Expression".into(), "ActualParameters'".into()]].into();

        //ActualParameters′ → , Expression ActualParameters′ | ε
        let actual_parameters_p: HashSet<Vec<Symbol<token::Token, &'static str>>> = [
            vec![
                Token::Comma.into(),
                "Expression".into(),
                "ActualParameters'".into(),
            ],
            vec![],
        ]
        .into();

        let productions: std::collections::HashMap<&str, HashSet<Vec<Symbol<Token, &str>>>> = [
            ("ToyCProgram", toyCProgram),
            ("ToyCProgram'", toyCProgramP),
            ("Definition", definition),
            ("Type", types),
            ("FunctionDefinition", function_definition),
            ("FunctionHeader", function_header),
            ("FunctionBody", function_body),
            ("FormalParamList", formal_param_list),
            ("FormalParamList'", formal_param_list_p),
            ("Statement", statement),
            ("ExpressionStatement", expression_statement),
            ("BreakStatement", break_statement),
            ("CompoundStatement", compound_statement),
            ("CompoundStatement'", compound_statement_p),
            ("CompoundStatement''", compound_statement_p_p),
            ("IfStatement", if_statement),
            ("IfStatement'", if_statement_p),
            ("NullStatement", null_statement),
            ("ReturnStatement", return_statement),
            ("WhileStatement", while_statement),
            ("ReadStatement", read_statement),
            ("ReadStatement'", read_statement_p),
            ("WriteStatement", write_statement),
            ("NewLineStatement", new_line_statement),
            ("Expression", expression),
            ("Expression'", expression_p),
            ("RelopExpression", relop_expression),
            ("RelopExpression'", relop_expression_p),
            ("SimpleExpression", simple_expression),
            ("SimpleExpression'", simple_expression_p),
            ("Term", term),
            ("Term'", term_p),
            ("Primary", Primary),
            ("FunctionCall", function_call),
            ("ActualParameters", actual_parameters),
            ("ActualParameters'", actual_parameters_p),
        ]
        .into();

        let cfg = ContextFreeGrammar::new(terminals, nonterminals, productions).unwrap();

        debug!("created grammar.");

        let _ll1 = LL1::new(cfg).unwrap();

        debug!("grammar is LL(1)");
    }
}
