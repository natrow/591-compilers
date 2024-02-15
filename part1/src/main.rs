//! EGRE 591 part1 - Nathan Rowan and Trevin Vaughan
//!
//! Run `cargo doc --open` to view this documentation in a browser.

#![warn(missing_docs)] // friendly reminder to add comments
#![warn(clippy::missing_docs_in_private_items)]

use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, ValueEnum};
use colored::Colorize;

use scanner::{
    error::{Error, ErrorKind},
    token::Token,
    Scanner,
};

pub mod scanner;

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

    if args.verbose {
        println!("input files: {:?}", &args.input_files);
    }

    for path in args.input_files {
        match Scanner::new(
            &path,
            matches!(args.debug, Some(DebugLevel::All | DebugLevel::Scanner)),
        ) {
            Ok(scanner) => match scanner.collect::<Result<Vec<Token>, Error<ErrorKind>>>() {
                Ok(tokens) => {
                    if args.verbose {
                        println!("scanned input file {:?}, tokens: {:#?}", path, tokens)
                    }
                }
                Err(e) => eprintln!("{} {}", "[ERROR]".red(), e),
            },
            Err(e) => eprintln!(
                "{} I/O error occured while scanning file {}: {}, skipping...",
                "[WARNING]".yellow(),
                path.to_string_lossy().purple(),
                e.to_string().blue()
            ),
        }
    }

    ExitCode::SUCCESS
}
