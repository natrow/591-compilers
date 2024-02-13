use std::{fs::File, io::BufReader, path::PathBuf, process::ExitCode};

use clap::{Parser, ValueEnum};
use scanner::{
    error::{Error, ErrorKind},
    token::Token,
    Scanner,
};

pub mod scanner;

#[derive(Clone, PartialEq, Eq, Parser)]
#[command(version, about)]
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
        eprintln!("[ERROR] Missing input files!");
        return ExitCode::FAILURE;
    }

    if args.verbose {
        println!("input files: {:?}", &args.input_files);
    }

    for path in args.input_files {
        if let Ok(file) = File::open(&path) {
            let buf = BufReader::new(file);

            if let Ok(scanner) = Scanner::new(
                buf,
                matches!(args.debug, Some(DebugLevel::All | DebugLevel::Scanner)),
            ) {
                match scanner.collect::<Result<Vec<Token>, Error<ErrorKind>>>() {
                    Ok(tokens) => {
                        if args.verbose {
                            println!("scanned input file {:?}, tokens: {:#?}", path, tokens)
                        }
                    }
                    Err(e) => eprintln!("[ERROR] {e}"),
                }
            } else {
                eprintln!(
                    "[WARN] I/O error occured while scanning file {:?}, skipping...",
                    path
                );
            }
        } else {
            eprintln!("[WARN] Could not open file {:?}, skipping...", path);
        }
    }

    ExitCode::SUCCESS
}
