//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Code generation for the Jasmin target (JVM)

use super::{Error, SymbolTable};
use crate::parser::ast::{Definition, Expression, Operator, Program, Statement, Type as AstType};

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
        Statement::Expr(e) => {
            code += &generate_code_for_expression(e, scope)?;
            code += "    pop\n"; // discard the result
        }
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
                code += &generate_code_for_expression(val, scope)?;
                code += "    ireturn\n";
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
fn generate_code_for_expression(
    expression: &Expression,
    scope: &SymbolTable,
) -> Result<String, Error> {
    let mut code = String::new();

    match expression {
        // load a number constant
        Expression::Number(n) => {
            code += &format!("    ldc {}\n", n);
        }
        // load an identifier value
        Expression::Identifier(id) => {
            let offset = scope.get_variable(id)?;
            code += &format!("    iload{}{}\n", sep(offset), offset);
        }
        // char literals are unimplemented
        Expression::CharLiteral(c) => return Err(Error::CharLiteral(*c)),
        // load a string literal
        Expression::StringLiteral(s) => {
            code += &format!("    ldc \"{}\"\n", s);
        }
        // function calls are unimplemented
        Expression::FuncCall(id, _) => return Err(Error::NonMainFunction(id.to_owned())),
        // binary operation expressions
        Expression::Expr(op, lhs, rhs) => {
            // assign statements are treated differently
            if matches!(op, Operator::Assign) {
                match &**lhs {
                    // lhs must be an id
                    Expression::Identifier(id) => {
                        // get the variable from the scope
                        let offset = scope.get_variable(id)?;
                        // generate code for the rhs
                        code += &generate_code_for_expression(rhs, scope)?;
                        // duplicate the result
                        code += "    dup\n";
                        // store one copy to the stack frame, leaving the other on the operator stack
                        code += &format!("    istore{}{}\n", sep(offset), offset);
                    }
                    _ => return Err(Error::InvalidAssign),
                }
            } else {
                // generate code for the left and right sides
                code += &generate_code_for_expression(lhs, scope)?;
                code += &generate_code_for_expression(rhs, scope)?;

                // consume the values
                match op {
                    Operator::Add => code += "    iadd\n",
                    Operator::Sub => code += "    isub\n",
                    Operator::Mul => code += "    imul\n",
                    Operator::Div => code += "    idiv\n",
                    Operator::Mod => code += "    irem\n",
                    Operator::BoolOr => todo!(),
                    Operator::BoolAnd => todo!(),
                    Operator::LtEq => todo!(),
                    Operator::Lt => todo!(),
                    Operator::Eq => todo!(),
                    Operator::Gt => todo!(),
                    Operator::GtEq => todo!(),
                    Operator::Neq => todo!(),
                    Operator::Assign => unreachable!(),
                }
            }
        }
        // negate an integer
        Expression::Minus(e) => {
            code += &generate_code_for_expression(e, scope)?;
            code += "    ineg\n";
        }
        // negate a boolean
        Expression::Not(_) => todo!(),
    }

    Ok(code)
}
