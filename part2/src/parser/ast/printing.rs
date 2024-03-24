//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan
//!
//! Implementation of pretty printing.

use std::fmt::Display;

use super::*;

/// Indentation size
const INDENT_SIZE: usize = 2;

/// Trait used to pretty print AST
trait PrettyPrint: Sized {
    /// Print out self, using the given current indent size
    fn print(&self, indent: usize) -> String;

    /// Determine whether the element is short (can be printed on one line)
    fn is_short(&self) -> bool;

    /// Print out a list, using the given current indent size
    fn print_list(list: &[Self], indent: usize) -> String {
        let mut s = String::new();
        s += "[";

        let short = list.iter().all(|e| e.is_short());

        if !short {
            s += "\n";
        }

        for (i, e) in list.iter().enumerate() {
            if !short {
                s += &" ".repeat(indent + INDENT_SIZE);
            }

            s += &e.print(indent + INDENT_SIZE);

            if i < list.len() - 1 {
                s += ",";
                if short {
                    s += " ";
                }
            }

            if !short {
                s += "\n"
            }
        }

        if !short {
            s += &" ".repeat(indent)
        }

        s += "]";
        s
    }
}

/// A reference to some pretty-printable value
#[derive(Clone, Copy)]
enum PrintableReference<'a> {
    /// Program
    Program(&'a Program),
    /// Definition
    Definition(&'a Definition),
    /// Variable definitions
    VarDef(&'a VarDef),
    /// Identifier definition
    Identifier(&'a Identifier),
    /// Statement
    Statement(&'a Statement),
    /// Expression
    Expression(&'a Expression),
    /// Operator
    Operator(&'a Operator),
    /// Type
    Type(&'a Type),
    /// List of references
    List(&'a [PrintableReference<'a>]),
    /// Optional reference
    Option(&'a Option<PrintableReference<'a>>),
}

impl<'a> PrettyPrint for PrintableReference<'a> {
    fn print(&self, indent: usize) -> String {
        match self {
            PrintableReference::Program(p) => p.print(indent),
            PrintableReference::Definition(d) => d.print(indent),
            PrintableReference::VarDef(v) => v.print(indent),
            PrintableReference::Identifier(i) => i.print(indent),
            PrintableReference::Statement(s) => s.print(indent),
            PrintableReference::Expression(e) => e.print(indent),
            PrintableReference::Operator(o) => o.print(indent),
            PrintableReference::Type(t) => t.print(indent),
            PrintableReference::List(l) => Self::print_list(l, indent),
            PrintableReference::Option(o) => o.map_or_else(String::new, |o| o.print(indent)),
        }
    }

    fn is_short(&self) -> bool {
        match self {
            PrintableReference::Program(p) => p.is_short(),
            PrintableReference::Definition(d) => d.is_short(),
            PrintableReference::VarDef(v) => v.is_short(),
            PrintableReference::Identifier(i) => i.is_short(),
            PrintableReference::Statement(s) => s.is_short(),
            PrintableReference::Expression(e) => e.is_short(),
            PrintableReference::Operator(o) => o.is_short(),
            PrintableReference::Type(t) => t.is_short(),
            PrintableReference::List(l) => l.iter().all(|e| e.is_short()),
            PrintableReference::Option(o) => o.map_or(true, |o| o.is_short()),
        }
    }
}

impl<'a> From<&'a Program> for PrintableReference<'a> {
    fn from(value: &'a Program) -> Self {
        Self::Program(value)
    }
}

impl<'a> From<&'a Definition> for PrintableReference<'a> {
    fn from(value: &'a Definition) -> Self {
        Self::Definition(value)
    }
}

impl<'a> From<&'a VarDef> for PrintableReference<'a> {
    fn from(value: &'a VarDef) -> Self {
        Self::VarDef(value)
    }
}

impl<'a> From<&'a Identifier> for PrintableReference<'a> {
    fn from(value: &'a Identifier) -> Self {
        Self::Identifier(value)
    }
}

impl<'a> From<&'a Statement> for PrintableReference<'a> {
    fn from(value: &'a Statement) -> Self {
        Self::Statement(value)
    }
}

impl<'a> From<&'a Expression> for PrintableReference<'a> {
    fn from(value: &'a Expression) -> Self {
        Self::Expression(value)
    }
}

impl<'a> From<&'a Operator> for PrintableReference<'a> {
    fn from(value: &'a Operator) -> Self {
        Self::Operator(value)
    }
}

impl<'a> From<&'a Type> for PrintableReference<'a> {
    fn from(value: &'a Type) -> Self {
        Self::Type(value)
    }
}

impl<'a> From<&'a [PrintableReference<'a>]> for PrintableReference<'a> {
    fn from(value: &'a [PrintableReference<'a>]) -> Self {
        Self::List(value)
    }
}

impl<'a> From<&'a Option<PrintableReference<'a>>> for PrintableReference<'a> {
    fn from(value: &'a Option<PrintableReference<'a>>) -> Self {
        Self::Option(value)
    }
}

/// Print the arguments to a function
fn print_args<'a, T: IntoIterator<Item = PrintableReference<'a>>>(
    args: T,
    indent: usize,
) -> String {
    let mut s = String::new();
    s += "(";

    let args: Vec<_> = args.into_iter().collect();

    let short = args.iter().all(|e| e.is_short());

    if !short {
        s += "\n";
    }

    for (i, e) in args.iter().enumerate() {
        let e = e.print(indent + INDENT_SIZE);
        if e.is_empty() {
            continue;
        }

        if !short {
            s += &" ".repeat(indent + INDENT_SIZE);
        }

        s += &e;

        if i < args.len() - 1 {
            s += ",";

            if short {
                s += " ";
            }
        }

        if !short {
            s += "\n";
        }
    }

    if !short {
        s += &" ".repeat(indent)
    }

    s += ")";
    s
}

impl PrettyPrint for Program {
    fn print(&self, indent: usize) -> String {
        format!(
            "prog{}",
            print_args(self.0.iter().map(PrintableReference::Definition), indent)
        )
    }

    fn is_short(&self) -> bool {
        false
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print(0))
    }
}

impl PrettyPrint for Definition {
    fn print(&self, indent: usize) -> String {
        match self {
            Definition::Func(id, ast_type, args, body) => {
                let args: Vec<_> = args.iter().map(Into::into).collect();
                format!(
                    "funcDef{}",
                    print_args(
                        [id.into(), ast_type.into(), (&args[..]).into(), body.into()],
                        indent
                    )
                )
            }
            Definition::Var(id, ast_type) => {
                let id: Vec<_> = id.iter().map(PrintableReference::Identifier).collect();
                format!(
                    "varDef{}",
                    print_args([(&id[..]).into(), ast_type.into()], indent)
                )
            }
        }
    }

    fn is_short(&self) -> bool {
        false
    }
}

impl PrettyPrint for VarDef {
    fn print(&self, indent: usize) -> String {
        let id: Vec<_> = self.0.iter().map(PrintableReference::Identifier).collect();

        format!(
            "varDef{}",
            print_args([(&id[..]).into(), (&self.1).into()], indent)
        )
    }

    fn is_short(&self) -> bool {
        false
    }
}

impl PrettyPrint for Identifier {
    fn print(&self, _indent: usize) -> String {
        self.to_string()
    }

    fn is_short(&self) -> bool {
        true
    }
}

impl PrettyPrint for Statement {
    fn print(&self, indent: usize) -> String {
        match self {
            Statement::Expr(e) => format!("exprState{}", print_args([e.into()], indent)),
            Statement::Break => "breakState()".to_string(),
            Statement::Block(var_def, statements) => {
                let var_def: Vec<_> = var_def.iter().map(PrintableReference::VarDef).collect();
                let statements: Vec<_> = statements
                    .iter()
                    .map(PrintableReference::Statement)
                    .collect();
                format!(
                    "blockState{}",
                    print_args(
                        [((&var_def[..]).into()), ((&statements[..]).into())],
                        indent
                    )
                )
            }
            Statement::If(condition, if_block, else_block) => {
                format!(
                    "ifState{}",
                    print_args(
                        [
                            condition.into(),
                            (&**if_block).into(),
                            (&else_block.as_ref().map(|e| (&**e).into())).into()
                        ],
                        indent
                    )
                )
            }
            Statement::Null => "nullState()".to_string(),
            Statement::Return(expr) => format!(
                "returnState{}",
                print_args(
                    [(&expr.as_ref().map(PrintableReference::Expression)).into()],
                    indent
                )
            ),
            Statement::While(condition, body) => format!(
                "whileState{}",
                print_args([condition.into(), (&**body).into()], indent)
            ),
            Statement::Read(args) => {
                let args: Vec<_> = args.iter().map(PrintableReference::Identifier).collect();
                format!("readState{}", print_args([(&args[..]).into()], indent))
            }
            Statement::Write(args) => {
                let args: Vec<_> = args.iter().map(PrintableReference::Expression).collect();
                format!("writeState{}", print_args([(&args[..]).into()], indent))
            }
            Statement::Newline => "newLineState()".to_string(),
        }
    }

    fn is_short(&self) -> bool {
        false
    }
}

impl PrettyPrint for Expression {
    fn print(&self, indent: usize) -> String {
        match self {
            Expression::Number(n) => n.clone(),
            Expression::Identifier(id) => id.clone(),
            Expression::CharLiteral(c) => c.map_or_else(String::new, |c| c.to_string()),
            Expression::StringLiteral(s) => format!("string(\"{}\")", s),
            Expression::FuncCall(id, args) => {
                let args: Vec<_> = args.iter().map(PrintableReference::Expression).collect();
                format!(
                    "funcCall{}",
                    print_args([id.into(), (&args[..]).into()], indent)
                )
            }
            Expression::Expr(op, lhs, rhs) => format!(
                "expr{}",
                print_args([op.into(), (&**lhs).into(), (&**rhs).into()], indent)
            ),
            Expression::Minus(expr) => format!("minus{}", print_args([(&**expr).into()], indent)),
            Expression::Not(expr) => format!("not{}", print_args([(&**expr).into()], indent)),
        }
    }

    fn is_short(&self) -> bool {
        match self {
            Expression::Number(_) => true,
            Expression::Identifier(_) => true,
            Expression::CharLiteral(_) => true,
            Expression::StringLiteral(_) => false,
            Expression::FuncCall(_, _) => false,
            Expression::Expr(_, _, _) => false,
            Expression::Minus(_) => false,
            Expression::Not(_) => false,
        }
    }
}

impl PrettyPrint for Operator {
    fn print(&self, _indent: usize) -> String {
        match self {
            Operator::Add => String::from("ADD"),
            Operator::Sub => String::from("SUB"),
            Operator::Mul => String::from("MUL"),
            Operator::Div => String::from("DIV"),
            Operator::Mod => String::from("MOD"),
            Operator::BoolOr => String::from("BOOL_OR"),
            Operator::BoolAnd => String::from("BOOL_AND"),
            Operator::LtEq => String::from("LT_EQ"),
            Operator::Lt => String::from("LT"),
            Operator::Eq => String::from("EQ"),
            Operator::Gt => String::from("GT"),
            Operator::GtEq => String::from("GT_EQ"),
            Operator::Neq => String::from("NEQ"),
            Operator::Assign => String::from("ASSIGN"),
        }
    }

    fn is_short(&self) -> bool {
        true
    }
}

impl PrettyPrint for Type {
    fn print(&self, _indent: usize) -> String {
        match self {
            Type::Int => String::from("int"),
            Type::Char => String::from("char"),
        }
    }

    fn is_short(&self) -> bool {
        true
    }
}
