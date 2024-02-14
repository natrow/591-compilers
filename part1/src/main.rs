use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, ValueEnum};
use colored::Colorize;

use scanner::{
    error::{Error, ErrorKind},
    token::Token,
    Scanner,
};

pub mod scanner;

#[derive(Clone, PartialEq, Eq, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    debug: Option<DebugLevel>,
    #[arg(short, long)]
    verbose: bool,
    input_files: Vec<PathBuf>,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DebugLevel {
    All,
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
