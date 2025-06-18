mod cli;
mod input;
mod output;
mod reduce;

use std::process::exit;

use crate::cli::Cli;
use crate::reduce::reduce_cidrs;

use anyhow::Result;
use clap::Parser;

enum ExitCode {
    Success = 0,
    Error = 1,
}

fn main() {
    let args = Cli::parse();

    let lines: Vec<String> = match read_input(args) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("{}", e);
            exit(ExitCode::Error as i32);
        }
    };

    let reduced = reduce_cidrs(lines);

    if let Err(e) = output::to_stdout(reduced) {
        eprintln!("{}", e);
        exit(ExitCode::Error as i32);
    }

    exit(ExitCode::Success as i32);
}

fn read_input(args: Cli) -> Result<Vec<String>> {
    match args.from_file {
        Some(file) => input::from_file(&file),
        None => input::from_stdin(),
    }
}
