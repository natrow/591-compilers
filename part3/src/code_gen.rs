//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Code generation implemented for part 3 of the project

use std::collections::HashMap;

use crate::parser::ast::{Definition, Expression, Program, Statement, Type as AstType};

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

/// Generate code for a given ToyC program
///
/// # Errors
///
/// Generates semantic errors in the AST, see [Error].
pub fn generate_code(ast: &Program, file_name: &str) -> Result<String, Error> {
    let mut symbol_table = SymbolTable::new_global();
    let mut code = String::new();
    let mut method_count = 0;

    code += &format!(".source {}\n", file_name);
    code += ".class ToyC\n";
    code += ".super java/lang/Object\n\n";

    code += &format!("; >> METHOD {} <<\n", method_count);
    code += ".method <init()V\n";
    code += "    .limit stack 1\n";
    code += "    .limit locals 1\n";
    code += "    aload_0\n";
    code += "    invokespecial java/lang/Object/<init>()V";
    code += "    return\n";
    code += ".end method\n\n";
    method_count += 1;

    for def in ast.0.iter() {
        match def {
            Definition::Func(id, return_type, args, body) => {
                if id == "main" {
                    code += &format!("; >> METHOD {} <<\n", method_count);

                    symbol_table.new_func(id)?;

                    // main must have signature int main()

                    if !matches!(return_type, AstType::Int) {
                        return Err(Error::InvalidReturn);
                    }

                    if !args.is_empty() {
                        return Err(Error::InvalidSubroutineParameters);
                    }

                    // create method in jvm
                    code += ".method public static main([Ljava/lang/String;)I\n";
                    code += "    .limit stack 999\n"; // calculating stack size is optionals
                    code += "    .limit locals 999\n";

                    // todo: actual function body
                    code += &generate_code_for_statement(body, &mut symbol_table)?;

                    code += ".end method\n\n";
                    method_count += 1;
                } else {
                    // implementing functions other than main is extra credit...
                    return Err(Error::NonMainFunction(id.to_owned()));
                }
            }
            Definition::Var(id, _) => return Err(Error::GlobalVariable(id[0].to_owned())),
        }
    }

    if !symbol_table.get_function("main") {
        return Err(Error::MissingMain);
    }

    Ok(code)
}

/// Generates code for a given statement in a ToyC program
///
/// # Errors
///
/// Generates semantic errors in the AST, see [Error].
fn generate_code_for_statement(
    statement: &Statement,
    scope: &mut SymbolTable,
) -> Result<String, Error> {
    let mut code = String::new();

    match statement {
        Statement::Expr(_) => todo!(),
        Statement::Break => return Err(Error::BreakStatement),
        Statement::Block(vars, statements) => {
            // create a new scope
            let mut scope = scope.new_scope();

            // add each variable identifier to the scope
            for (ids, ast_type) in vars {
                if !matches!(ast_type, AstType::Int) {
                    return Err(Error::TypeUnimplemented(*ast_type));
                }

                for id in ids {
                    scope.new_var(id)?;
                }
            }

            // generate code for each statement
            for statement in statements {
                code += &generate_code_for_statement(statement, &mut scope)?;
            }
        }
        Statement::If(_, _, _) => todo!(),
        Statement::Null => (),
        Statement::Return(val) => {
            if let Some(val) = val {
                todo!()
            } else {
                // all functions must return an int
                return Err(Error::InvalidReturn);
            }
        }
        Statement::While(_, _) => todo!(),
        Statement::Read(args) => {
            let scanner = scope.current_offset;
            scope.current_offset += 1;

            // construct a scanner
            code += "    new java/util/Scanner\n";
            code += "    dup\n";
            // get standard input
            code += "    getstatic java/lang/System/in Ljava/io/InputStream;\n";
            // initialize scanner
            code += "    invokespecial java/util/Scanner/<init>(Ljava/io/InputStream;)V\n";
            // store the scanner to the stack frame
            code += &format!("    astore{}{}\n", sep(scanner), scanner);

            for arg in args {
                let var = scope.get_variable(arg)?;

                // load the scanner
                code += &format!("    aload{}{}\n", sep(scanner), scanner);
                // read an integer
                code += "invokevirtual java/util/Scanner/nextInt()I\n";
                // store the integer
                code += &format!("    istore{}{}\n", sep(var), var);
            }
        }
        Statement::Write(_) => todo!(),
        Statement::Newline => {
            // get standard output
            code += "    getstatic java/lang/System/out Ljava/io/PrintStream;\n";
            // print a newline
            code += "    invokevirtual java/io/PrintStream/println()V\n";
        }
    }

    Ok(code)
}

/// Creates a separator for jvm instructions such as `astore_1`
fn sep(offset: usize) -> char {
    if offset < 4 {
        '_'
    } else {
        ' '
    }
}

/// Generates code for expressions. Leaves the result on the stack to be used in statements
fn generate_code_for_expression(expression: &Expression, scope: &SymbolTable) -> Result<(), Error> {
    todo!()
}
