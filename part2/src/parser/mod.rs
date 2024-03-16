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
    pub fn parse(mut self) -> Result<()> {
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
        Ok(())
    }

    /// Determines whether the buffer matches the predict set
    // fn predict(&self, set: Vec<Token>) -> Result<()> {
    //     if set.iter().any(|t| t.syntax_eq(&self.buffer)) {
    //         Ok(())
    //     } else {
    //         Err(self.error(set))
    //     }
    // }

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
    fn nt_toy_c_program(&mut self) -> Result<()> {
        self.debug("reducing ToyCProgram");

        self.nt_toy_c_program_()?;

        // don't call take because it will load the buffer after EOF, causing a panic
        match self.buffer {
            Eof => Ok(()),
            _ => Err(self.expected(&[Eof])),
        }
    }

    /// Definition ToyCProgram' | ε
    fn nt_toy_c_program_(&mut self) -> Result<()> {
        self.debug("reducing ToyCProgram'");

        match self.buffer {
            Keyword(Int | Char) => {
                self.nt_definition()?;
                self.nt_toy_c_program_()?;

                Ok(())
            }
            Eof => Ok(()),
            _ => Err(self.expected(&[Keyword(Int), Keyword(Char), Eof])),
        }
    }

    /// Type <identifier> Definition'
    fn nt_definition(&mut self) -> Result<()> {
        self.debug("reducing Definition");

        self.nt_type()?;
        self.take(Identifier(String::new()))?;
        self.nt_definition_()?;

        Ok(())
    }

    /// FunctionDefinition | <;>
    fn nt_definition_(&mut self) -> Result<()> {
        self.debug("reducing Definition'");

        match self.buffer {
            LParen => self.nt_function_definition(),
            Semicolon => self.load_next_token(),
            _ => Err(self.expected(&[LParen, Semicolon])),
        }
    }

    /// <int> | <char>
    fn nt_type(&mut self) -> Result<()> {
        self.debug("reducing Type");

        match self.buffer {
            Keyword(Int) | Keyword(Char) => self.load_next_token(),
            _ => Err(self.expected(&[Keyword(Int), Keyword(Char)])),
        }
    }

    /// FunctionHeader FunctionBody
    fn nt_function_definition(&mut self) -> Result<()> {
        self.debug("reducing FunctionDefinition");

        self.nt_function_header()?;
        self.nt_function_body()?;

        Ok(())
    }

    /// <(> FunctionHeader' <)>
    fn nt_function_header(&mut self) -> Result<()> {
        self.debug("reducing FunctionHeader");

        self.take(LParen)?;
        self.nt_function_header_()?;
        self.take(RParen)?;

        Ok(())
    }

    /// FormalParamList | ε
    fn nt_function_header_(&mut self) -> Result<()> {
        self.debug("reducing FunctionHeader'");

        match self.buffer {
            Keyword(Int) | Keyword(Char) => {
                self.nt_formal_param_list()?;

                Ok(())
            }
            RParen => Ok(()),
            _ => Err(self.expected(&[Keyword(Int), Keyword(Char), RParen])),
        }
    }

    /// CompoundStatement
    fn nt_function_body(&mut self) -> Result<()> {
        self.debug("reducing FunctionBody");

        self.nt_compound_statement()?;

        Ok(())
    }

    /// Type <identifier> FormalParamList'
    fn nt_formal_param_list(&mut self) -> Result<()> {
        self.debug("reducing FormalParamList");

        self.nt_type()?;
        self.take(Identifier(String::new()))?;
        self.nt_formal_param_list_()?;

        Ok(())
    }

    /// <,> Type <identifier> FormalParamList' | ε
    fn nt_formal_param_list_(&mut self) -> Result<()> {
        self.debug("reducing FormalParamList'");

        match self.buffer {
            Comma => {
                self.take(Comma)?;
                self.nt_type()?;
                self.take(Identifier(String::new()))?;
                self.nt_formal_param_list_()?;

                Ok(())
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
    fn nt_statement(&mut self) -> Result<()> {
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
    fn nt_expression_statement(&mut self) -> Result<()> {
        self.debug("reducing ExpressionStatement");

        self.nt_expression()?;
        self.take(Semicolon)?;

        Ok(())
    }

    /// <break> <;>
    fn nt_break_statement(&mut self) -> Result<()> {
        self.debug("reducing BreakStatement");

        self.take(Keyword(Break))?;
        self.take(Semicolon)?;

        Ok(())
    }

    /// <{> CompoundStatement' CompoundStatement'' <}>
    fn nt_compound_statement(&mut self) -> Result<()> {
        self.debug("reducing CompoundStatement");

        self.take(LCurly)?;
        self.nt_compound_statement_()?;
        self.nt_compound_statement__()?;
        self.take(RCurly)?;

        Ok(())
    }

    /// Type <identifier> <;> CompoundStatement' | ε
    fn nt_compound_statement_(&mut self) -> Result<()> {
        self.debug("reducing CompoundStatement'");

        match self.buffer {
            Keyword(Int | Char) => {
                self.nt_type()?;
                self.take(Identifier(String::new()))?;
                self.take(Semicolon)?;
                self.nt_compound_statement_()?;

                Ok(())
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
            | LParen => Ok(()),
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
    fn nt_compound_statement__(&mut self) -> Result<()> {
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
                self.nt_statement()?;
                self.nt_compound_statement__()?;

                Ok(())
            }
            RCurly => Ok(()),
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
    fn nt_if_statement(&mut self) -> Result<()> {
        self.debug("reducing IfStatement");

        self.take(Keyword(If))?;
        self.take(LParen)?;
        self.nt_expression()?;
        self.take(RParen)?;
        self.nt_statement()?;
        self.nt_if_statement_()?;

        Ok(())
    }

    /// <else> Statement | ε
    fn nt_if_statement_(&mut self) -> Result<()> {
        self.debug("reducing IfStatement'");

        match self.buffer {
            Keyword(Else) => {
                self.take(Keyword(Else))?;
                self.nt_statement()?;

                Ok(())
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
            | LParen => Ok(()),
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
    fn nt_null_statement(&mut self) -> Result<()> {
        self.debug("reducing NullStatement");

        self.take(Semicolon)?;

        Ok(())
    }

    /// <return> ReturnStatement' <;>
    fn nt_return_statement(&mut self) -> Result<()> {
        self.debug("reducing ReturnStatement");

        self.take(Keyword(Return))?;
        self.nt_return_statement_()?;
        self.take(Semicolon)?;

        Ok(())
    }

    /// Expression | ε
    fn nt_return_statement_(&mut self) -> Result<()> {
        self.debug("reducing ReturnStatement'");

        match self.buffer {
            AddOp(Sub) | LParen | StringLiteral(_) | CharLiteral(_) | Number(_) | Not
            | Identifier(_) => self.nt_expression(),
            Semicolon => Ok(()),
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
    fn nt_while_statement(&mut self) -> Result<()> {
        self.debug("reducing WhileStatement");

        self.take(Keyword(While))?;
        self.take(LParen)?;
        self.nt_expression()?;
        self.take(RParen)?;
        self.nt_statement()?;

        Ok(())
    }

    /// <read> <(> <identifier> ReadStatement' <)> <;>
    fn nt_read_statement(&mut self) -> Result<()> {
        self.debug("reducing ReadStatement");

        self.take(Keyword(Read))?;
        self.take(LParen)?;
        self.take(Identifier(String::new()))?;
        self.nt_read_statement_()?;
        self.take(RParen)?;
        self.take(Semicolon)?;

        Ok(())
    }

    /// <,> <identifier> ReadStatement' | ε
    fn nt_read_statement_(&mut self) -> Result<()> {
        self.debug("reducing ReadStatement'");

        match self.buffer {
            Comma => {
                self.take(Comma)?;
                self.take(Identifier(String::new()))?;
                self.nt_read_statement_()?;

                Ok(())
            }
            RParen => Ok(()),
            _ => Err(self.expected(&[Comma, RParen])),
        }
    }

    /// <write> <(> ActualParameters <)> <;>
    fn nt_write_statement(&mut self) -> Result<()> {
        self.debug("reducing WriteStatement");

        self.take(Keyword(Write))?;
        self.take(LParen)?;
        self.nt_actual_parameters()?;
        self.take(RParen)?;
        self.take(Semicolon)?;

        Ok(())
    }

    /// <newline> <;>
    fn nt_newline_statement(&mut self) -> Result<()> {
        self.debug("reducing NewlineStatement");

        self.take(Keyword(Newline))?;
        self.take(Semicolon)?;

        Ok(())
    }

    ///RelopExpression Expression′
    fn nt_expression(&mut self) -> Result<()> {
        self.debug("reducing expression");

        match self.buffer {
            Not | CharLiteral(_) | Number(_) | AddOp(_) | LParen | Identifier(_)
            | StringLiteral(_) => {
                self.nt_relop_expression()?;
                self.nt_expression_()?;

                Ok(())
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

    ///<assignop> RelopExpression Expression′ | ε
    fn nt_expression_(&mut self) -> Result<()> {
        self.debug("reducing expression'");

        match self.buffer {
            AssignOp => {
                self.take(AssignOp)?;
                self.nt_relop_expression()?;
                self.nt_expression_()?;

                Ok(())
            }
            Semicolon | RParen | Comma => Ok(()),
            _ => Err(self.expected(&[Semicolon, Comma, AssignOp, RParen])),
        }
    }

    ///SimpleExpression RelopExpression′
    fn nt_relop_expression(&mut self) -> Result<()> {
        self.debug("reducing relopExpression");

        match self.buffer {
            AddOp(_) | StringLiteral(_) | CharLiteral(_) | Not | Identifier(_) | Number(_)
            | LParen => {
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;

                Ok(())
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

    ///<relop> SimpleExpression RelopExpression′ | ε
    fn nt_relop_expression_(&mut self) -> Result<()> {
        self.debug("reducing relop expression'");

        match self.buffer {
            RelOp(Eq) => {
                self.take(RelOp(Eq))?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;
                Ok(())
            }

            RelOp(Lt) => {
                self.take(RelOp(Lt))?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;
                Ok(())
            }

            RelOp(Gt) => {
                self.take(RelOp(Gt))?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;
                Ok(())
            }

            RelOp(LtEq) => {
                self.take(RelOp(LtEq))?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;
                Ok(())
            }

            RelOp(GtEq) => {
                self.take(RelOp(GtEq))?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;
                Ok(())
            }

            RelOp(Neq) => {
                self.take(RelOp(Neq))?;
                self.nt_simple_expression()?;
                self.nt_relop_expression_()?;
                Ok(())
            }

            Semicolon | Comma | RParen | AssignOp => Ok(()),
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

    ///Term SimpleExpression′
    fn nt_simple_expression(&mut self) -> Result<()> {
        self.debug("reducing simple expression");

        match self.buffer {
            StringLiteral(_) | AddOp(_) | CharLiteral(_) | Number(_) | Identifier(_) | LParen
            | Not => {
                self.nt_term()?;
                self.nt_simple_expression_()?;
                Ok(())
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

    ///<addop> Term SimpleExpression′ | ε
    fn nt_simple_expression_(&mut self) -> Result<()> {
        self.debug("reducing simple expression'");

        match self.buffer {
            AddOp(_) => match self.buffer {
                AddOp(Sub) => {
                    self.take(AddOp(Sub))?;
                    self.nt_simple_expression()?;
                    self.nt_relop_expression_()?;
                    Ok(())
                }

                AddOp(Add) => {
                    self.take(AddOp(Add))?;
                    self.nt_simple_expression()?;
                    self.nt_relop_expression_()?;
                    Ok(())
                }

                AddOp(BoolOr) => {
                    self.take(AddOp(BoolOr))?;
                    self.nt_simple_expression()?;
                    self.nt_relop_expression_()?;
                    Ok(())
                }

                _ => Err(self.expected(&[AddOp(Sub), AddOp(Add), AddOp(BoolOr)])),
            },

            Semicolon | AssignOp | RelOp(_) | Comma | RParen => Ok(()),

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

    ///Primary Term′
    fn nt_term(&mut self) -> Result<()> {
        self.debug("reducing term");

        match self.buffer {
            StringLiteral(_) | CharLiteral(_) | LParen | AddOp(_) | Number(_) | Not
            | Identifier(_) => {
                self.nt_primary()?;
                self.nt_term_()?;
                Ok(())
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

    ///this function does the branching for term'
    /// because the take() can't take MulOP(_) as an argurment
    fn term_p_branching_helper(&mut self) -> Result<()> {
        match self.buffer {
            MulOp(Mul) => {
                self.take(MulOp(Mul))?;
                self.nt_primary()?;
                self.nt_term_()?;
                Ok(())
            }

            MulOp(Div) => {
                self.take(MulOp(Div))?;
                self.nt_primary()?;
                self.nt_term_()?;
                Ok(())
            }

            MulOp(Mod) => {
                self.take(MulOp(Mod))?;
                self.nt_primary()?;
                self.nt_term_()?;
                Ok(())
            }

            MulOp(BoolAnd) => {
                self.take(MulOp(BoolAnd))?;
                self.nt_primary()?;
                self.nt_term_()?;
                Ok(())
            }

            _ => Err(self.expected(&[MulOp(BoolAnd), MulOp(Div), MulOp(Mod), MulOp(Mul)])),
        }
    }

    ///<mulop> Primary Term′ | ε
    fn nt_term_(&mut self) -> Result<()> {
        self.debug("reducing term'");

        match self.buffer {
            MulOp(_) => self.term_p_branching_helper(),

            AddOp(_) | Comma | Semicolon | RParen | RelOp(_) | AssignOp => Ok(()),

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

    /// identifier primary'
    /// | <number>
    /// | <stringConstant>
    /// | <charConstant>
    /// | <(>Expression <)>
    /// | <-> Primary
    /// | <not> Primary
    fn nt_primary(&mut self) -> Result<()> {
        self.debug("reducing primary");

        match self.buffer {
            Identifier(_) => {
                self.take(Identifier(String::new()))?;

                match self.buffer {
                    //need to check if the identifier is followed my a function call
                    LParen => {
                        self.nt_primary_()?;
                        Ok(())
                    }
                    //else wise it just an identifier
                    _ => Ok(()),
                }
            }

            Number(_) => {
                self.take(Number(String::new()))?;
                Ok(())
            }

            StringLiteral(_) => {
                self.take(StringLiteral(String::new()))?;
                Ok(())
            }

            CharLiteral(_) => {
                self.take(CharLiteral(None))?;
                Ok(())
            }

            LParen => {
                self.take(LParen)?;
                self.nt_expression()?;
                self.take(RParen)?;
                Ok(())
            }

            AddOp(Sub) => {
                self.take(AddOp(Sub))?;
                self.nt_primary()?;
                Ok(())
            }

            Not => {
                self.take(Not)?;
                self.nt_primary()?;
                Ok(())
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
        self.debug("reducing primary'");

        match self.buffer {
            //LParen signal the begin of a function Call
            LParen => {
                self.nt_function_call()?;
                Ok(())
            }

            Comma | Semicolon | AddOp(_) | RParen | AssignOp | MulOp(_) | RelOp(_) => Ok(()),

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
                Ok(())
            }

            _ => Err(self.expected(&[LParen])),
        }
    }

    ///actualParameters | ε
    fn nt_function_call_(&mut self) -> Result<()> {
        self.debug("Reducing functionCall'");
        /*StringLiteral,
        Identifier,
        CharLiteral,
        AddOp,
        Number,
        RParen,
        Not,
        LParen, */
        match self.buffer {
            StringLiteral(_) | Identifier(_) | CharLiteral(_) | AddOp(_) | Number(_) | Not
            | LParen => {
                self.nt_actual_parameters()?;
                Ok(())
            }

            RParen => Ok(()),

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

    ///Expression ActualParameters′
    fn nt_actual_parameters(&mut self) -> Result<()> {
        self.debug("reducing actualExpression");

        match self.buffer {
            LParen | Not | CharLiteral(_) | StringLiteral(_) | Identifier(_)
            | Number(_) | AddOp(Sub) //double check this addOP()
            => {
                self.nt_expression()?;
                self.nt_actual_parameters_()?;

                Ok(())
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

    ///<,> Expression ActualParameters′ | ε
    fn nt_actual_parameters_(&mut self) -> Result<()> {
        self.debug("reducing actual parameters");

        match self.buffer {
            Comma => {
                self.take(Comma)?;
                self.nt_expression()?;
                self.nt_actual_parameters_()?;
                Ok(())
            }

            RParen => Ok(()),

            _ => Err(self.expected(&[LParen, Comma])),
        }
    }
}
