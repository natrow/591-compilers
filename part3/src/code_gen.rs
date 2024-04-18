//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Code generation implemented for part 3 of the project

pub mod jsm;

use std::collections::HashMap;

use crate::parser::ast::Type as AstType;

/// Errors that can happen during code generation
#[derive(Debug, Clone)]
pub enum Error {
    /// A function was declared in a non-global scope
    NonGlobalFunction(String),
    /// A name was re-used in an illegal way (shadowing is allowed)
    NameCollision(String),
    /// A global variable was defined, but this is not implemented
    GlobalVariable(String),
    /// Main function was not found
    MissingMain,
    /// Return type was different than expected
    InvalidReturn,
    /// Function invocation uses different parameters than expected
    InvalidSubroutineParameters,
    /// Division by zero is undefined
    DivisionByZero,
    /// Functions that aren't main aren't implemented
    NonMainFunction(String),
    /// Break statements aren't implemented
    BreakStatement,
    /// A variable was missing
    MissingVariable(String),
    /// Uses a type that isn't implemented
    TypeUnimplemented(AstType),
    /// Assigns to something other than an identifier
    InvalidAssign,
    /// Character literals aren't implemented
    CharLiteral(Option<char>),
}

/// Types of symbols in the symbol table
#[derive(Debug, Clone, Copy)]
enum Type {
    /// Integers (variables)
    Int,
    /// Functions
    Func,
}

/// Individual entries in the symbol table
#[derive(Debug, Clone, Copy)]
struct TableEntry {
    /// offset, e.g. r0 in Arm or load_0 in the jvm
    offset: usize,
    /// whether the variable is local to the current scope
    local: bool,
    /// whether this is a function or variable
    symbol_type: Type,
}

/// The symbol table itself
#[derive(Debug, Clone)]
struct SymbolTable {
    /// first available offset to be used
    current_offset: usize,
    /// actual table
    elements: HashMap<String, TableEntry>,
    /// whether or not this is the global symbol table
    global: bool,
}

impl SymbolTable {
    /// Create the top-level, global symbol table
    fn new_global() -> Self {
        Self {
            current_offset: 0,
            elements: HashMap::new(),
            global: true,
        }
    }

    /// create a new scope, marking all existing symbols as non-local
    fn new_scope(&self) -> Self {
        let mut new = self.clone();

        new.global = false;
        for e in new.elements.values_mut() {
            e.local = false;
        }

        new
    }

    /// attempt to make a new function in the table
    fn new_func(&mut self, id: &str) -> Result<(), Error> {
        // must be global scope and unique name
        if !self.global {
            return Err(Error::NonGlobalFunction(id.to_owned()));
        } else if self.elements.contains_key(id) {
            return Err(Error::NameCollision(id.to_owned()));
        }
        // create the entry
        let func = TableEntry {
            offset: 0, // functions don't live in memory so this field is ignored
            local: true,
            symbol_type: Type::Func,
        };
        // insert it to the table
        self.elements.insert(id.to_owned(), func);

        Ok(())
    }

    /// attempt to make a new variable in the table
    fn new_var(&mut self, id: &str) -> Result<(), Error> {
        // cannot reuse name in the same scope
        if let Some(e) = self.elements.get(id) {
            if !e.local {
                return Err(Error::NameCollision(id.to_owned()));
            }
        }
        // create the table entry
        let var = TableEntry {
            offset: self.current_offset,
            local: true,
            symbol_type: Type::Int,
        };
        // increment offset
        self.current_offset += 1;
        // insert it to the table
        self.elements.insert(id.to_owned(), var);

        Ok(())
    }

    /// determine whether a function exists
    fn get_function(&self, id: &str) -> bool {
        if let Some(e) = self.elements.get(id) {
            matches!(e.symbol_type, Type::Func)
        } else {
            false
        }
    }

    /// determine whether a variable exists and return its offset
    fn get_variable(&self, id: &str) -> Result<usize, Error> {
        if let Some(e) = self.elements.get(id) {
            if matches!(e.symbol_type, Type::Int) {
                return Ok(e.offset);
            }
        }

        Err(Error::MissingVariable(id.to_owned()))
    }
}
