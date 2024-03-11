//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan

use crate::{
    file_buffer::Context,
    scanner::{
        token::{AddOp::*, Keyword::*, Token, Token::*},
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
    verbose: bool,
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
            verbose,
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

    fn nt_expression(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_expression_(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_relop_expression(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_relop_expression_(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_simple_expression(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_simple_expression_(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_term(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_term_(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_primary(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_primary_(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_function_call(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_function_call_(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_actual_parameters(&mut self) -> Result<()> {
        todo!()
    }

    fn nt_actual_parameters_(&mut self) -> Result<()> {
        todo!()
    }
}
