mod cli;
mod input;
mod output_format;

use std::process::ExitCode;

use crate::cli::Args;
use net_reduce::reduce_cidrs;

use anyhow::Result;
use clap::Parser;

fn main() -> ExitCode {
    let args = Args::parse();
    let output_format = args.output_format;

    let lines: Vec<String> = match read_input(args) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let reduced = reduce_cidrs(lines);

    let w = std::io::stdout();
    if let Err(e) = output_format.write(reduced, w) {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn read_input(args: Args) -> Result<Vec<String>> {
    match args.file {
        Some(file) => input::from_file(&file),
        None => input::from_stdin(),
    }
}
