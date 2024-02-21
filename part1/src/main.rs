//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan
//!
//! Run `cargo doc --open` to view this documentation in a browser.

#![warn(missing_docs)] // friendly reminder to add comments
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::all)]

use std::{fmt::Display, path::PathBuf, process::ExitCode};

use clap::{Parser, ValueEnum};
use colored::Colorize;

pub mod file_buffer;
pub mod scanner;

use file_buffer::Context;
use scanner::{error::Error, token::Token, Scanner};

/// Command line arguments accepted by the scanner
#[derive(Clone, PartialEq, Eq, Parser)]
#[command(version, about)]
struct Args {
    /// Display messages that aid in tracing the
    /// compilation process
    #[arg(short, long, value_enum)]
    debug: Option<DebugLevel>,
    /// Display all information
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
}

/// An error type that may or may not have locational context
enum MaybeContext<E: Display> {
    /// Variant that happens when there is locational context
    Context(Context<E>),
    /// Variant that happens when there is no locational context
    NoContext(E),
}

impl From<Error> for MaybeContext<Error> {
    fn from(value: Error) -> Self {
        Self::NoContext(value)
    }
}

impl<E: Display> From<Context<E>> for MaybeContext<E> {
    fn from(value: Context<E>) -> Self {
        Self::Context(value)
    }
}

impl<E: Display> Display for MaybeContext<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeContext::Context(c) => c.fmt(f),
            MaybeContext::NoContext(n) => n.fmt(f),
        }
    }
}

fn main() -> ExitCode {
    // parse command line arguments
    let args = Args::parse();

    // if the list of input files is empty throw an error
    if args.input_files.is_empty() {
        eprintln!("{} Missing input files!", "[ERROR]".red());
        return ExitCode::FAILURE;
    }

    if args.verbose {
        println!("input files: {:?}", &args.input_files);
    }

    let debug_scanner = matches!(args.debug, Some(DebugLevel::All | DebugLevel::Scanner));

    for path in args.input_files {
        match catch_errors(path, debug_scanner, args.verbose) {
            Ok(_) => {}
            Err(e) => eprintln!("{} {}", "[ERROR]".red(), e),
        }
    }

    ExitCode::SUCCESS
}

/// Runs scanner on the given file path, collecting all the errors into one value
fn catch_errors(
    path: PathBuf,
    debug_scanner: bool,
    verbose: bool,
) -> Result<(), MaybeContext<Error>> {
    let scanner = Scanner::new(&path, debug_scanner, verbose)?;

    // iterators are lazily evaluated, .collect() will force the entire file to be read
    let res: Result<Vec<Token>, Context<Error>> = scanner.collect();
    // the actual value can be ignored, but pass errors to the caller
    let _tokens = res?;

    Ok(())
}
