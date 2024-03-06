//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan
//!
//! Run `cargo doc --open` to view this documentation in a browser.

#![warn(missing_docs)] // friendly reminder to add comments
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::all)]

use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, ValueEnum};
use colored::Colorize;

pub mod file_buffer;
pub mod scanner;

use file_buffer::MaybeContext;
use scanner::{error::Error as ScannerError, Scanner};

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

    for path in args.input_files {
        // this is the Rust equivalent of the try-catch pattern
        let try_catch = || {
            let scanner = Scanner::new(&path, debug_scanner, verbose)?;

            // this will scan the entire file into a list of tokens
            // instead, the parser would consume the iterator and return an AST
            let tokens = scanner.collect::<Result<Vec<_>, _>>()?;

            Ok::<_, MaybeContext<ScannerError>>(tokens)
        };

        if let Err(e) = try_catch() {
            eprintln!("{} {}", "[ERROR]".red(), e);
        }
    }

    ExitCode::SUCCESS
}
