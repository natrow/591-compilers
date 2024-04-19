//! EGRE 591 part3 - Nathan Rowan and Trevin Vaughan
//!
//! Run `cargo doc --open` to view this documentation in a browser.

// friendly reminders to add comments
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::all)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_errors_doc)]

use std::{fs::write, path::PathBuf, process::ExitCode};

use clap::{Parser as ClapParser, ValueEnum};
use code_gen::jsm::generate_code;
use colored::Colorize;

pub mod code_gen;
pub mod context;
pub mod file_buffer;
pub mod parser;
pub mod scanner;

use context::MaybeContext;
use parser::{ast::Program, error::Error as ParserError, Parser};
use scanner::Scanner;

/// Command line arguments accepted by the scanner
#[derive(Clone, PartialEq, Eq, ClapParser)]
#[command(version, about)]
struct Args {
    /// specifies class file name
    #[arg(short, long)]
    class: Option<String>,
    /// specifies target file name
    #[arg(short, long)]
    output: Option<String>,
    /// display messages that aid in tracing the
    /// compilation process
    #[arg(short, long, value_enum)]
    debug: Option<DebugLevel>,
    /// dump the abstract syntax tree
    #[arg(short, long)]
    abstract_: bool,
    /// dump the symbol table(s)
    #[arg(short, long)]
    symbol: bool,
    /// dump the generated program
    #[arg(short, long)]
    code: bool,
    /// display all information
    #[arg(short, long)]
    verbose: bool,
    /// toyc source files
    input_files: Vec<PathBuf>,
}

/// Debug levels of the program
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DebugLevel {
    /// All messages
    All,
    /// Scanner messages only
    Scanner,
    /// Parser messages only
    Parser,
}

fn main() -> ExitCode {
    // parse command line arguments
    let args = Args::parse();

    // if the list of input files is empty throw an error
    if args.input_files.is_empty() {
        eprintln!("{} Missing input files!", "[ERROR]".red());
        return ExitCode::FAILURE;
    }

    let verbose = args.verbose;

    if verbose {
        println!("input files: {:?}", &args.input_files);
    }

    let debug_scanner = matches!(args.debug, Some(DebugLevel::All | DebugLevel::Scanner));
    let debug_parser = matches!(args.debug, Some(DebugLevel::All | DebugLevel::Parser));

    for path in args.input_files {
        // front-end of the compiler
        let parse = || {
            let scanner = Scanner::new(&path, debug_scanner, verbose).map_err(ParserError::from)?;

            let parser = Parser::new(scanner, debug_parser, verbose)?;

            let ast = parser.parse()?;

            if args.abstract_ {
                println!("<< Abstract Syntax >>\n{}", ast)
            }

            Ok::<Program, MaybeContext<ParserError>>(ast)
        };

        let ast = match parse() {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{} {}", "[ERROR]".red(), e);
                continue;
            }
        };

        // back-end of the compiler
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if args.symbol {
            println!("<< Symbol Table(s) >>");
        }

        let code = match generate_code(
            &ast,
            file_name,
            args.class.as_ref().unwrap_or(&String::from("ToyC")),
            args.symbol,
        ) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("{} {}", "[ERROR]".red(), e);
                continue;
            }
        };

        if args.code {
            println!("<< Generated Code >>\n{}", code)
        }

        if let Some(output) = &args.output {
            write(output, &code).unwrap();
        }
    }

    ExitCode::SUCCESS
}
