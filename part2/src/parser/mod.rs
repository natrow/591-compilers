//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use crate::{
    file_buffer::Context,
    scanner::{
        token::{
            AddOp::*,
            Keyword::*,
            MulOp::*,
            RelOp::*,
            Token::{self, *},
        },
        Scanner,
    },
};

pub mod error;
use error::Error;

pub mod ast;
use ast::*;

/// Short-hand version of Result<T, E> where E = Context<Error>
type Result<T> = core::result::Result<T, Context<Error>>;

/// Parser implementation, which consumes the scanner iterator.
pub struct Parser {
    /// The inner Scanner iterator
    scanner: Scanner,
    /// Whether or not to print debug information
    debug: bool,
    /// Whether or not to print verbose information
    _verbose: bool,
    /// Look-ahead buffer
    buffer: Token,
}

impl Parser {
    /// Construct the parser
    ///
    /// # Errors
    ///
    /// If the first token cannot be read (probably because of I/O) this function fails.
    #[allow(clippy::missing_panics_doc)] // never panics, EOF is inserted if file is empty
    pub fn new(mut scanner: Scanner, debug: bool, verbose: bool) -> Result<Self> {
        let token = scanner.next().unwrap()?;

        Ok(Self {
            scanner,
            debug,
            _verbose: verbose,
            buffer: token,
        })
    }

    /// Parse into an AST, consuming the parser
    ///
    /// # Errors
    ///
    /// Errors can happen during scanning, I/O, or because of syntax errors in the input file.
    pub fn parse(mut self) -> Result<Program> {
        self.nt_toy_c_program()
    }

    /* Inner implementation, using an LL(1) recursive descent predictive parser */

    /// Fills the look ahead buffer with the next token.
    ///
    /// # Panics
    ///
    /// Panics if called after the EOF marker.
    fn load_next_token(&mut self) -> Result<()> {
        let token = self.scanner.next().unwrap()?;
        self.buffer = token;
        todo!()
    }

    /// Takes a token from the buffer, reloading it and returning the token
    fn take(&mut self, expected: Token) -> Result<Token> {
        if self.buffer.syntax_eq(&expected) {
            let token = self.buffer.clone();
            self.load_next_token()?;
            Ok(token)
        } else {
            Err(self.error(vec![expected.to_str()]))
        }
    }

    /// Gives context to an error
    fn context(&self, e: Error) -> Context<Error> {
        self.scanner.context(e)
    }

    /// Creates a syntax error
    fn error(&self, expected: Vec<&'static str>) -> Context<Error> {
        self.context(Error::SyntaxError {
            got: self.buffer.clone(),
            expected,
        })
    }

    /// Creates a syntax error for a specific token
    fn expected(&self, expected: &[Token]) -> Context<Error> {
        self.error(expected.iter().map(Token::to_str).collect())
    }

    /// Prints debug messages
    fn debug(&self, msg: &str) {
        if self.debug {
            println!("[PARSER] {msg}")
        }
    }

    /// ToyCProgram' <EOF>
    fn nt_toy_c_program(&mut self) -> Result<Program> {
        self.debug("reducing ToyCProgram");

        let mut definitions = Vec::new();
        self.nt_toy_c_program_(&mut definitions)?;

        // don't call take because it will load the buffer after EOF, causing a panic
        match self.buffer {
            Eof => Ok(Program(definitions)),
            _ => Err(self.expected(&[Eof])),
        }
    }

    /// Definition ToyCProgram' | ε
    fn nt_toy_c_program_(&mut self, definitions: &mut Vec<Definition>) -> Result<()> {
        self.debug("reducing ToyCProgram'");

        match self.buffer {
            Keyword(Int | Char) => {
                definitions.push(self.nt_definition()?);
                self.nt_toy_c_program_(definitions)
            }
            Eof => Ok(()),
            _ => Err(self.expected(&[Keyword(Int), Keyword(Char), Eof])),
        }
    }

    /// Type <identifier> Definition'
    fn nt_definition(&mut self) -> Result<Definition> {
        self.debug("reducing Definition");

        let ast_type = self.nt_type()?;

        let ast_id = self.take(Identifier(String::new()))?.try_into().unwrap();
        self.nt_definition_(ast_type, ast_id)
    }

    /// FunctionDefinition | <;>
    fn nt_definition_(&mut self, ast_type: Type, ast_id: String) -> Result<Definition> {
        self.debug("reducing Definition'");

        match self.buffer {
            LParen => self.nt_function_definition(ast_type, ast_id),
            Semicolon => {
                self.load_next_token()?;
                Ok(Definition::Var(vec![ast_id], ast_type))
            }
            _ => Err(self.expected(&[LParen, Semicolon])),
        }
    }

    /// <int> | <char>
    fn nt_type(&mut self) -> Result<Type> {
        self.debug("reducing Type");

        match self.buffer {
            Keyword(Int) | Keyword(Char) => {
                let ast_type = self.buffer.clone().try_into().unwrap();
                self.load_next_token()?;
                Ok(ast_type)
            }
            _ => Err(self.expected(&[Keyword(Int), Keyword(Char)])),
        }
    }

    /// FunctionHeader FunctionBody
    fn nt_function_definition(&mut self, ast_type: Type, ast_id: String) -> Result<Definition> {
        self.debug("reducing FunctionDefinition");

        let var_def = self.nt_function_header()?;
        let statement = self.nt_function_body()?;

        Ok(Definition::Func(ast_id, ast_type, var_def, statement))
    }

    /// <(> FunctionHeader' <)>
    fn nt_function_header(&mut self) -> Result<Vec<VarDef>> {
        self.debug("reducing FunctionHeader");

        self.take(LParen)?;
        let var_def = self.nt_function_header_()?;
        self.take(RParen)?;

        Ok(var_def)
    }

    /// FormalParamList | ε
    fn nt_function_header_(&mut self) -> Result<Vec<VarDef>> {
        self.debug("reducing FunctionHeader'");

        match self.buffer {
            Keyword(Int) | Keyword(Char) => self.nt_formal_param_list(),
            RParen => Ok(Vec::new()),
            _ => Err(self.expected(&[Keyword(Int), Keyword(Char), RParen])),
        }
    }

    /// CompoundStatement
    fn nt_function_body(&mut self) -> Result<Statement> {
        self.debug("reducing FunctionBody");

        self.nt_compound_statement()
    }

    /// Type <identifier> FormalParamList'
    fn nt_formal_param_list(&mut self) -> Result<Vec<VarDef>> {
        self.debug("reducing FormalParamList");

        let ast_type = self.nt_type()?;
        let ast_id = self.take(Identifier(String::new()))?.try_into().unwrap();
        let mut var_def = vec![(vec![ast_id], ast_type)];
        self.nt_formal_param_list_(&mut var_def)?;

        Ok(var_def)
    }

    /// <,> Type <identifier> FormalParamList' | ε
    fn nt_formal_param_list_(&mut self, var_def: &mut Vec<VarDef>) -> Result<()> {
        self.debug("reducing FormalParamList'");

        match self.buffer {
            Comma => {
                self.take(Comma)?;
                let ast_type = self.nt_type()?;
                let ast_id = self.take(Identifier(String::new()))?.try_into().unwrap();
                var_def.push((vec![ast_id], ast_type));
                self.nt_formal_param_list_(var_def)
            }
            RParen => Ok(()),
            _ => Err(self.expected(&[Comma, RParen])),
        }
    }

    /// ExpressionStatement
    ///  | BreakStatement
    ///  | CompoundStatement
    ///  | IfStatement
    ///  | NullStatement
    ///  | ReturnStatement
    ///  | WhileStatement
    ///  | ReadStatement
    ///  | WriteStatement
    ///  | NewLineStatement
    fn nt_statement(&mut self) -> Result<Statement> {
        self.debug("reducing Statement");

        match self.buffer {
            Identifier(_) | Number(_) | LParen | Not | CharLiteral(_) | StringLiteral(_)
            | AddOp(Sub) => self.nt_expression_statement(),
            Keyword(Break) => self.nt_break_statement(),
            LCurly => self.nt_compound_statement(),
            Keyword(If) => self.nt_if_statement(),
            Semicolon => self.nt_null_statement(),
            Keyword(Return) => self.nt_return_statement(),
            Keyword(While) => self.nt_while_statement(),
            Keyword(Read) => self.nt_read_statement(),
            Keyword(Write) => self.nt_write_statement(),
            Keyword(Newline) => self.nt_newline_statement(),

            _ => Err(self.expected(&[
                Identifier(String::new()),
                Number(String::new()),
                LParen,
                Not,
                CharLiteral(None),
                StringLiteral(String::new()),
                AddOp(Sub),
                Keyword(Break),
                LCurly,
                Keyword(If),
                Semicolon,
                Keyword(Return),
                Keyword(While),
                Keyword(Read),
                Keyword(Write),
                Keyword(Newline),
            ])),
        }
    }

    /// Expression <;>
    fn nt_expression_statement(&mut self) -> Result<Statement> {
        self.debug("reducing ExpressionStatement");

        let expression = self.nt_expression()?;
        self.take(Semicolon)?;

        Ok(Statement::Expr(expression))
    }

    /// <break> <;>
    fn nt_break_statement(&mut self) -> Result<Statement> {
        self.debug("reducing BreakStatement");

        self.take(Keyword(Break))?;
        self.take(Semicolon)?;

        Ok(Statement::Break)
    }

    /// <{> CompoundStatement' CompoundStatement'' <}>
    fn nt_compound_statement(&mut self) -> Result<Statement> {
        self.debug("reducing CompoundStatement");

        self.take(LCurly)?;
        let mut var_def = Vec::new();
        self.nt_compound_statement_(&mut var_def)?;
        let mut statements = Vec::new();
        self.nt_compound_statement__(&mut statements)?;
        self.take(RCurly)?;

        Ok(Statement::Block(var_def, statements))
    }

    /// Type <identifier> <;> CompoundStatement' | ε
    fn nt_compound_statement_(&mut self, var_def: &mut Vec<VarDef>) -> Result<()> {
        self.debug("reducing CompoundStatement'");

        match self.buffer {
            Keyword(Int | Char) => {
                let ast_type = self.nt_type()?;
                let ast_id = self
                    .take(Identifier(String::new()))?
                    .clone()
                    .try_into()
                    .unwrap();
                var_def.push((vec![ast_id], ast_type));
                self.take(Semicolon)?;
                self.nt_compound_statement_(var_def)
            }
            Keyword(Read | Newline | Write | While | Break | Return | If)
            | Identifier(_)
            | Number(_)
            | StringLiteral(_)
            | CharLiteral(_)
            | AddOp(Sub)
            | LCurly
            | RCurly
            | Not
            | Semicolon
            | LParen => todo!(),
            _ => Err(self.expected(&[
                Keyword(Int),
                Keyword(Char),
                Keyword(Read),
                Keyword(Newline),
                Keyword(Write),
                Keyword(While),
                Keyword(Break),
                Keyword(Return),
                Keyword(If),
                Identifier(String::new()),
                Number(String::new()),
                StringLiteral(String::new()),
                CharLiteral(None),
                AddOp(Sub),
                LCurly,
                RCurly,
                Not,
                Semicolon,
                LParen,
            ])),
        }
    }

    /// Statement CompoundStatement'' | ε
    fn nt_compound_statement__(&mut self, statements: &mut Vec<Statement>) -> Result<()> {
        self.debug("reducing CompoundStatement''");

        match self.buffer {
            Keyword(Read | Newline | Write | While | Break | Return | If)
            | Identifier(_)
            | Number(_)
            | StringLiteral(_)
            | CharLiteral(_)
            | AddOp(Sub)
            | LCurly
            | Not
            | Semicolon
            | LParen => {
                let statement = self.nt_statement()?;
                statements.push(statement);
                self.nt_compound_statement__(statements)
            }
            RCurly => todo!(),
            _ => Err(self.expected(&[
                Keyword(Read),
                Keyword(Newline),
                Keyword(Write),
                Keyword(While),
                Keyword(Break),
                Keyword(Return),
                Keyword(If),
                Identifier(String::new()),
                Number(String::new()),
                StringLiteral(String::new()),
                CharLiteral(None),
                AddOp(Sub),
                LCurly,
                RCurly,
                Not,
                Semicolon,
                LParen,
            ])),
        }
    }

    /// <if> <(> Expression <)> Statement IfStatement'
    fn nt_if_statement(&mut self) -> Result<Statement> {
        self.debug("reducing IfStatement");

        self.take(Keyword(If))?;
        self.take(LParen)?;
        let expression = self.nt_expression()?;
        self.take(RParen)?;
        let true_statement = Box::new(self.nt_statement()?);
        let false_statement = self.nt_if_statement_()?.map(Box::new);

        Ok(Statement::If(expression, true_statement, false_statement))
    }

    /// <else> Statement | ε
    fn nt_if_statement_(&mut self) -> Result<Option<Statement>> {
        self.debug("reducing IfStatement'");

        match self.buffer {
            Keyword(Else) => {
                self.take(Keyword(Else))?;
                let statement = self.nt_statement()?;

                Ok(Some(statement))
            }
            Keyword(Read | Newline | Write | While | Break | Return | If)
            | Identifier(_)
            | Number(_)
            | StringLiteral(_)
            | CharLiteral(_)
            | AddOp(Sub)
            | LCurly
            | Not
            | Semicolon
            | LParen => Ok(None),
            _ => Err(self.expected(&[
                Keyword(Read),
                Keyword(Newline),
                Keyword(Write),
                Keyword(While),
                Keyword(Break),
                Keyword(Return),
                Keyword(If),
                Keyword(Else),
                Identifier(String::new()),
                Number(String::new()),
                StringLiteral(String::new()),
                CharLiteral(None),
                AddOp(Sub),
                LCurly,
                RCurly,
                Not,
                Semicolon,
                LParen,
            ])),
        }
    }

    /// <;>
    fn nt_null_statement(&mut self) -> Result<Statement> {
        self.debug("reducing NullStatement");

        self.take(Semicolon)?;

        Ok(Statement::Null)
    }

    /// <return> ReturnStatement' <;>
    fn nt_return_statement(&mut self) -> Result<Statement> {
        self.debug("reducing ReturnStatement");

        self.take(Keyword(Return))?;
        let expression = self.nt_return_statement_()?;
        self.take(Semicolon)?;

        Ok(Statement::Return(expression))
    }

    /// Expression | ε
    fn nt_return_statement_(&mut self) -> Result<Option<Expression>> {
        self.debug("reducing ReturnStatement'");

        match self.buffer {
            AddOp(Sub) | LParen | StringLiteral(_) | CharLiteral(_) | Number(_) | Not
            | Identifier(_) => {
                let expression = self.nt_expression()?;
                Ok(Some(expression))
            }
            Semicolon => Ok(None),
            _ => Err(self.expected(&[
                AddOp(Sub),
                LParen,
                StringLiteral(String::new()),
                CharLiteral(None),
                Number(String::new()),
                Not,
                Identifier(String::new()),
            ])),
        }
    }

    /// <while> <(> Expression <)> Statement
    fn nt_while_statement(&mut self) -> Result<Statement> {
        self.debug("reducing WhileStatement");

        self.take(Keyword(While))?;
        self.take(LParen)?;
        let expression = self.nt_expression()?;
        self.take(RParen)?;
        let statement = Box::new(self.nt_statement()?);

        Ok(Statement::While(expression, statement))
    }

    /// <read> <(> <identifier> ReadStatement' <)> <;>
    fn nt_read_statement(&mut self) -> Result<Statement> {
        self.debug("reducing ReadStatement");

        self.take(Keyword(Read))?;
        self.take(LParen)?;
        let ast_id = self
            .take(Identifier(String::new()))?
            .clone()
            .try_into()
            .unwrap();
        let mut ids = vec![ast_id];
        self.nt_read_statement_(&mut ids)?;
        self.take(RParen)?;
        self.take(Semicolon)?;

        Ok(Statement::Read(ids))
    }

    /// <,> <identifier> ReadStatement' | ε
    fn nt_read_statement_(&mut self, ids: &mut Vec<String>) -> Result<()> {
        self.debug("reducing ReadStatement'");

        match self.buffer {
            Comma => {
                self.take(Comma)?;
                let ast_id = self
                    .take(Identifier(String::new()))?
                    .clone()
                    .try_into()
                    .unwrap();
                ids.push(ast_id);
                self.nt_read_statement_(ids)
            }
            RParen => Ok(()),
            _ => Err(self.expected(&[Comma, RParen])),
        }
    }

    /// <write> <(> ActualParameters <)> <;>
    fn nt_write_statement(&mut self) -> Result<Statement> {
        self.debug("reducing WriteStatement");

        self.take(Keyword(Write))?;
        self.take(LParen)?;
        let params = self.nt_actual_parameters()?;
        self.take(RParen)?;
        self.take(Semicolon)?;

        Ok(Statement::Write(params))
    }

    /// <newline> <;>
    fn nt_newline_statement(&mut self) -> Result<Statement> {
        self.debug("reducing NewlineStatement");

        self.take(Keyword(Newline))?;
        self.take(Semicolon)?;

        Ok(Statement::Newline)
    }

    /// RelopExpression Expression'
    fn nt_expression(&mut self) -> Result<Expression> {
        self.debug("reducing Expression");

        match self.buffer {
            Not | CharLiteral(_) | Number(_) | AddOp(_) | LParen | Identifier(_)
            | StringLiteral(_) => {
                self.nt_relop_expression()?;
                self.nt_expression_()?;

                todo!()
            }
            _ => Err(self.expected(&[
                LParen,
                Not,
                LParen,
                CharLiteral(None),
                StringLiteral(String::new()),
                Identifier(String::new()),
                Number(String::new()),
                AddOp(Add),
                AddOp(Sub),
                AddOp(BoolOr),
            ])),
        }
    }

    /// <assignop> RelopExpression Expression' | ε
    fn nt_expression_(&mut self) -> Result<()> {
        self.debug("reducing Expression'");

        match self.buffer {
            AssignOp => {
                self.take(AssignOp)?;
                self.nt_relop_expression()?;
                self.nt_expression_()?;

                todo!()
            }
            Semicolon | RParen | Comma => todo!(),
            _ => Err(self.expected(&[Semicolon, Comma, AssignOp, RParen])),
        }
    }

    /// SimpleExpression RelopExpression'
    fn nt_relop_expression(&mut self) -> Result<()> {
        self.debug("reducing RelopExpression");

        match self.buffer {
            AddOp(_) | StringLiteral(_) | CharLiteral(_) | Not | Identifier(_) | Number(_)
            | LParen => {
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;

                todo!()
            }
            _ => Err(self.expected(&[
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                StringLiteral(String::new()),
                CharLiteral(None),
                Not,
                Identifier(String::new()),
                Number(String::new()),
                LParen,
            ])),
        }
    }

    ///<relop> SimpleExpression RelopExpression' | ε
    fn nt_relop_expression_(&mut self) -> Result<()> {
        self.debug("reducing RelopExpression'");

        match self.buffer {
            RelOp(_) => {
                self.load_next_token()?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;

                todo!()
            }
            Semicolon | Comma | RParen | AssignOp => todo!(),
            _ => Err(self.expected(&[
                AssignOp,
                RParen,
                RelOp(Gt),
                RelOp(GtEq),
                RelOp(Lt),
                RelOp(LtEq),
                RelOp(Eq),
                RelOp(Neq),
                Comma,
                Semicolon,
            ])),
        }
    }

    /// Term SimpleExpression'
    fn nt_simple_expression(&mut self) -> Result<()> {
        self.debug("reducing SimpleExpression");

        match self.buffer {
            StringLiteral(_) | AddOp(_) | CharLiteral(_) | Number(_) | Identifier(_) | LParen
            | Not => {
                self.nt_term()?;
                self.nt_simple_expression_()?;

                todo!()
            }
            _ => Err(self.expected(&[
                StringLiteral(String::new()),
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                CharLiteral(None),
                Number(String::new()),
                Identifier(String::new()),
                LParen,
                Not,
            ])),
        }
    }

    /// <addop> Term SimpleExpression' | ε
    fn nt_simple_expression_(&mut self) -> Result<()> {
        self.debug("reducing SimpleExpression'");

        match self.buffer {
            AddOp(_) => {
                self.load_next_token()?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;

                todo!()
            }
            Semicolon | AssignOp | RelOp(_) | Comma | RParen => todo!(),
            _ => Err(self.expected(&[
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                Semicolon,
                AssignOp,
                RelOp(Eq),
                RelOp(Neq),
                RelOp(Gt),
                RelOp(GtEq),
                RelOp(Lt),
                RelOp(LtEq),
                Comma,
                RParen,
            ])),
        }
    }

    /// Primary Term'
    fn nt_term(&mut self) -> Result<()> {
        self.debug("reducing Term");

        match self.buffer {
            StringLiteral(_) | CharLiteral(_) | LParen | AddOp(_) | Number(_) | Not
            | Identifier(_) => {
                self.nt_primary()?;
                self.nt_term_()?;

                todo!()
            }
            _ => Err(self.expected(&[
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                StringLiteral(String::new()),
                LParen,
                Number(String::new()),
                Not,
                Identifier(String::new()),
            ])),
        }
    }

    /// <mulop> Primary Term' | ε
    fn nt_term_(&mut self) -> Result<()> {
        self.debug("reducing Term'");

        match self.buffer {
            MulOp(_) => {
                self.load_next_token()?;
                self.nt_primary()?;
                self.nt_term_()?;

                todo!()
            }
            AddOp(_) | Comma | Semicolon | RParen | RelOp(_) | AssignOp => todo!(),
            _ => Err(self.expected(&[
                MulOp(BoolAnd),
                MulOp(Div),
                MulOp(Mod),
                MulOp(Mul),
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                AddOp(BoolOr),
                Comma,
                Semicolon,
                RParen,
                RelOp(Gt),
                RelOp(GtEq),
                RelOp(Lt),
                RelOp(LtEq),
                RelOp(Eq),
                RelOp(Neq),
                AssignOp,
            ])),
        }
    }

    /// Identifier Primary'
    /// | <Number>
    /// | <StringConstant>
    /// | <CharConstant>
    /// | <(> Expression <)>
    /// | <-> Primary
    /// | <Not> Primary
    fn nt_primary(&mut self) -> Result<()> {
        self.debug("reducing Primary");

        match self.buffer {
            Identifier(_) => {
                self.take(Identifier(String::new()))?;
                self.nt_primary_()?;

                todo!();
            }
            Number(_) => {
                self.take(Number(String::new()))?;

                todo!()
            }
            StringLiteral(_) => {
                self.take(StringLiteral(String::new()))?;

                todo!()
            }
            CharLiteral(_) => {
                self.take(CharLiteral(None))?;

                todo!()
            }
            LParen => {
                self.take(LParen)?;
                self.nt_expression()?;
                self.take(RParen)?;

                todo!()
            }
            AddOp(Sub) => {
                self.take(AddOp(Sub))?;
                self.nt_primary()?;

                todo!()
            }
            Not => {
                self.take(Not)?;
                self.nt_primary()?;

                todo!()
            }
            _ => Err(self.expected(&[
                AddOp(Sub),
                LParen,
                Number(String::new()),
                CharLiteral(None),
                Identifier(String::new()),
                StringLiteral(String::new()),
                Not,
            ])),
        }
    }

    /// FunctionCall | ε
    fn nt_primary_(&mut self) -> Result<()> {
        self.debug("reducing Primary'");

        match self.buffer {
            LParen => {
                self.nt_function_call()?;
                todo!()
            }
            Comma | Semicolon | AddOp(_) | RParen | AssignOp | MulOp(_) | RelOp(_) => todo!(),
            _ => Err(self.expected(&[
                MulOp(BoolAnd),
                MulOp(Div),
                MulOp(Mod),
                MulOp(Mul),
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                Comma,
                Semicolon,
                LParen,
                RParen,
                RelOp(Gt),
                RelOp(GtEq),
                RelOp(Lt),
                RelOp(LtEq),
                RelOp(Eq),
                RelOp(Neq),
                AssignOp,
            ])),
        }
    }

    /// <(> FunctionCall' <)>
    fn nt_function_call(&mut self) -> Result<()> {
        self.debug("reducing function call");

        match self.buffer {
            LParen => {
                self.take(LParen)?;
                self.nt_function_call_()?;
                self.take(RParen)?;
                todo!()
            }

            _ => Err(self.expected(&[LParen])),
        }
    }

    /// ActualParameters | ε
    fn nt_function_call_(&mut self) -> Result<()> {
        self.debug("Reducing FunctionCall'");

        match self.buffer {
            StringLiteral(_) | Identifier(_) | CharLiteral(_) | AddOp(_) | Number(_) | Not
            | LParen => {
                self.nt_actual_parameters()?;

                todo!()
            }
            RParen => todo!(),
            _ => Err(self.expected(&[
                AddOp(Sub),
                AddOp(Add),
                AddOp(BoolOr),
                LParen,
                RParen,
                StringLiteral(String::new()),
                Identifier(String::new()),
                CharLiteral(None),
                Not,
            ])),
        }
    }

    /// Expression ActualParameters'
    fn nt_actual_parameters(&mut self) -> Result<Vec<Expression>> {
        self.debug("reducing ActualExpression");

        match self.buffer {
            LParen | Not | CharLiteral(_) | StringLiteral(_) | Identifier(_) | Number(_)
            | AddOp(Sub) => {
                let expression = self.nt_expression()?;
                let mut expressions = vec![expression];
                self.nt_actual_parameters_(&mut expressions)?;

                Ok(expressions)
            }
            _ => Err(self.expected(&[
                LParen,
                Not,
                LParen,
                CharLiteral(None),
                StringLiteral(String::new()),
                Identifier(String::new()),
                Number(String::new()),
                AddOp(Sub), ////double check this AddOP()
            ])),
        }
    }

    /// <,> Expression ActualParameters' | ε
    fn nt_actual_parameters_(&mut self, expressions: &mut Vec<Expression>) -> Result<()> {
        self.debug("reducing ActualParameters");

        match self.buffer {
            Comma => {
                self.take(Comma)?;
                let expression = self.nt_expression()?;
                expressions.push(expression);
                self.nt_actual_parameters_(expressions)
            }
            RParen => Ok(()),
            _ => Err(self.expected(&[LParen, Comma])),
        }
    }
}
