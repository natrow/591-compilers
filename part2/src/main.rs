//! EGRE 591 part2 - Nathan Rowan and Trevin Vaughan
//!
//! Run `cargo doc --open` to view this documentation in a browser.

// friendly reminders to add comments
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::all)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_errors_doc)]

use std::{path::PathBuf, process::ExitCode};

use clap::{Parser as ClapParser, ValueEnum};
use colored::Colorize;

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
    /// Display messages that aid in tracing the
    /// compilation process
    #[arg(short, long, value_enum)]
    debug: Option<DebugLevel>,
    /// Display all information
    #[arg(short, long)]
    verbose: bool,
    /// Display the abstract syntax tree
    #[arg(short, long)]
    abstract_: bool,
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
        // this is the Rust equivalent of the try-catch pattern
        let try_catch = || {
            let scanner = Scanner::new(&path, debug_scanner, verbose).map_err(ParserError::from)?;

            let parser = Parser::new(scanner, debug_parser, verbose)?;

            let ast = parser.parse()?;

            Ok::<Program, MaybeContext<ParserError>>(ast)
        };

        match try_catch() {
            Ok(ast) => {
                if args.abstract_ {
                    // todo: implement display
                    println!("{:#?}", ast)
                }
            }
            Err(e) => {
                eprintln!("{} {}", "[ERROR]".red(), e);
            }
        }
    }

    ExitCode::SUCCESS
}
