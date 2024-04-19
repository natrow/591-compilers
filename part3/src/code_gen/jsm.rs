//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Code generation for the Jasmin target (JVM)

use super::{Error, LabelMaker, SymbolTable};
use crate::parser::ast::{Definition, Expression, Operator, Program, Statement, Type as AstType};

/// Generate code for a given ToyC program
///
/// # Errors
///
/// Generates semantic errors in the AST, see [Error].
pub fn generate_code(
    ast: &Program,
    file_name: &str,
    class_name: &str,
    dump_table: bool,
) -> Result<String, Error> {
    let mut symbol_table = SymbolTable::new_global();
    let mut code = String::new();
    let mut method_count = 0;
    let mut label_maker = LabelMaker::new();

    // file headers
    code += "; created using EGRE-591 ToyC compiler by Nathan Rowan and Trevin Vaughan\n\n";

    code += &format!(".source {}\n", file_name);
    code += &format!(".class {}\n", class_name);
    code += ".super java/lang/Object\n\n";

    // create <init> method
    code += &format!("; >> METHOD {} <<\n", method_count);
    code += ".method <init>()V\n";
    code += "    .limit stack 1\n";
    code += "    .limit locals 1\n";
    code += "    aload_0\n";
    code += "    invokespecial java/lang/Object/<init>()V\n";
    code += "    return\n";
    code += ".end method\n\n";
    method_count += 1;

    // create main method (jvm entrypoint)
    code += ".method public static main([Ljava/lang/String;)V\n";
    code += "    .limit stack 1\n"; // calculating stack size is optionals
    code += "    .limit locals 1\n";
    code += &format!("    invokestatic {}/toyc_main()I\n", class_name);
    code += "    pop\n";
    code += "    return\n";
    code += ".end method\n\n";
    method_count += 1;

    code += "; begin ToyC code generation...\n\n";

    for def in ast.0.iter() {
        match def {
            Definition::Func(id, return_type, args, body) => {
                if id == "main" {
                    // main must have signature int main()

                    if !matches!(return_type, AstType::Int) {
                        return Err(Error::InvalidReturn);
                    }

                    if !args.is_empty() {
                        return Err(Error::InvalidSubroutineParameters);
                    }

                    // setup for new function
                    code += &format!("; >> METHOD {} <<\n", method_count);
                    symbol_table.new_func(id)?;

                    // create fake main method as toyc runtime entrypoint
                    code += ".method static toyc_main()I\n";
                    code += "    .limit stack 999\n"; // calculating stack size is optionals
                    code += "    .limit locals 999\n";

                    // insert code generation
                    code += &generate_code_for_statement(
                        body,
                        &mut symbol_table,
                        dump_table,
                        &mut label_maker,
                    )?;

                    // wrap up new function
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

    code += "; end ToyC code generation\n";

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
    dump_table: bool,
    label_maker: &mut LabelMaker,
) -> Result<String, Error> {
    let mut code = String::new();

    match statement {
        Statement::Expr(e) => {
            let (new_code, is_int) = generate_code_for_expression(e, scope, label_maker)?;

            if !is_int {
                return Err(Error::IncompatibleTypes);
            };

            code += &new_code;
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

            // print the symbol table
            if dump_table {
                println!("{:#?}", scope);
            }

            // generate code for each statement
            for statement in statements {
                code +=
                    &generate_code_for_statement(statement, &mut scope, dump_table, label_maker)?;
            }
        }
        Statement::If(_, _, _) => todo!(),
        Statement::Null => (),
        Statement::Return(val) => {
            if let Some(val) = val {
                let (new_code, is_int) = generate_code_for_expression(val, scope, label_maker)?;

                if !is_int {
                    return Err(Error::IncompatibleTypes);
                };

                code += &new_code;
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
                code += "    invokevirtual java/util/Scanner/nextInt()I\n";
                // store the integer
                code += &format!("    istore{}{}\n", sep(var), var);
            }
        }
        Statement::Write(expressions) => {
            for e in expressions {
                let (new_code, is_int) = generate_code_for_expression(e, scope, label_maker)?;

                code += "    getstatic java/lang/System/out Ljava/io/PrintStream;\n";

                code += &new_code;

                if is_int {
                    code += "    invokevirtual java/io/PrintStream/print(I)V\n";
                } else {
                    code += "    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V\n";
                }
            }
        }
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
///
/// Returns the code and a bool representing whether it's an integer or not
fn generate_code_for_expression(
    expression: &Expression,
    scope: &SymbolTable,
    label_maker: &mut LabelMaker,
) -> Result<(String, bool), Error> {
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
                        let (rhs_code, is_int) =
                            generate_code_for_expression(rhs, scope, label_maker)?;

                        if !is_int {
                            return Err(Error::IncompatibleTypes);
                        };

                        code += &rhs_code;
                        // duplicate the result
                        code += "    dup\n";
                        // store one copy to the stack frame, leaving the other on the operator stack
                        code += &format!("    istore{}{}\n", sep(offset), offset);
                    }
                    _ => return Err(Error::InvalidAssign),
                }
            } else {
                // generate code for the left and right sides
                let (lhs_code, lhs_is_int) = generate_code_for_expression(lhs, scope, label_maker)?;
                let (rhs_code, rhs_is_int) = generate_code_for_expression(rhs, scope, label_maker)?;

                if !lhs_is_int || !rhs_is_int {
                    return Err(Error::IncompatibleTypes);
                };

                code += &lhs_code;
                code += &rhs_code;

                // consume the values
                match op {
                    Operator::Add => code += "    iadd\n",
                    Operator::Sub => code += "    isub\n",
                    Operator::Mul => code += "    imul\n",
                    Operator::Div => code += "    idiv\n",
                    Operator::Mod => code += "    irem\n",
                    Operator::BoolOr => todo!(),
                    Operator::BoolAnd => todo!(),
                    Operator::LtEq => {
                        // label if jump taken
                        let if_label = label_maker.mk_label();
                        // label after conditional
                        let end_label = label_maker.mk_label();
                        // do comparison
                        code += &format!("    if_icmple {}\n", if_label);
                        // false: load 0 and jump to end
                        code += "    iconst_0\n";
                        code += &format!("    goto {}\n", end_label);
                        // true: load 1
                        code += &format!("{}:\n", if_label);
                        code += "    iconst_1\n";
                        // end
                        code += &format!("{}:\n", end_label);
                    }
                    Operator::Lt => {
                        // label if jump taken
                        let if_label = label_maker.mk_label();
                        // label after conditional
                        let end_label = label_maker.mk_label();
                        // do comparison
                        code += &format!("    if_icmplt {}\n", if_label);
                        // false: load 0 and jump to end
                        code += "    iconst_0\n";
                        code += &format!("    goto {}\n", end_label);
                        // true: load 1
                        code += &format!("{}:\n", if_label);
                        code += "    iconst_1\n";
                        // end
                        code += &format!("{}:\n", end_label);
                    }
                    Operator::Eq => {
                        // label if jump taken
                        let if_label = label_maker.mk_label();
                        // label after conditional
                        let end_label = label_maker.mk_label();
                        // do comparison
                        code += &format!("    if_icmpeq {}\n", if_label);
                        // false: load 0 and jump to end
                        code += "    iconst_0\n";
                        code += &format!("    goto {}\n", end_label);
                        // true: load 1
                        code += &format!("{}:\n", if_label);
                        code += "    iconst_1\n";
                        // end
                        code += &format!("{}:\n", end_label);
                    }
                    Operator::Gt => {
                        // label if jump taken
                        let if_label = label_maker.mk_label();
                        // label after conditional
                        let end_label = label_maker.mk_label();
                        // do comparison
                        code += &format!("    if_icmpgt {}\n", if_label);
                        // false: load 0 and jump to end
                        code += "    iconst_0\n";
                        code += &format!("    goto {}\n", end_label);
                        // true: load 1
                        code += &format!("{}:\n", if_label);
                        code += "    iconst_1\n";
                        // end
                        code += &format!("{}:\n", end_label);
                    }
                    Operator::GtEq => {
                        // label if jump taken
                        let if_label = label_maker.mk_label();
                        // label after conditional
                        let end_label = label_maker.mk_label();
                        // do comparison
                        code += &format!("    if_icmpge {}\n", if_label);
                        // false: load 0 and jump to end
                        code += "    iconst_0\n";
                        code += &format!("    goto {}\n", end_label);
                        // true: load 1
                        code += &format!("{}:\n", if_label);
                        code += "    iconst_1\n";
                        // end
                        code += &format!("{}:\n", end_label);
                    }
                    Operator::Neq => {
                        // label if jump taken
                        let if_label = label_maker.mk_label();
                        // label after conditional
                        let end_label = label_maker.mk_label();
                        // do comparison
                        code += &format!("    if_icmpne {}\n", if_label);
                        // false: load 0 and jump to end
                        code += "    iconst_0\n";
                        code += &format!("    goto {}\n", end_label);
                        // true: load 1
                        code += &format!("{}:\n", if_label);
                        code += "    iconst_1\n";
                        // end
                        code += &format!("{}:\n", end_label);
                    }
                    Operator::Assign => unreachable!(),
                }
            }
        }
        // negate an integer
        Expression::Minus(e) => {
            let (new_code, is_int) = generate_code_for_expression(e, scope, label_maker)?;

            if !is_int {
                return Err(Error::IncompatibleTypes);
            };

            code += &new_code;
            code += "    ineg\n";
        }
        // negate a boolean
        Expression::Not(_) => todo!(),
    }

    let integer = !matches!(
        expression,
        Expression::CharLiteral(_) | Expression::StringLiteral(_)
    );

    Ok((code, integer))
}
