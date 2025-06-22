use clap::Parser;

use crate::output_format::OutputFormat;

#[derive(Parser)]
#[command(
    version,
    name = "net-reduce",
    author = "Daniel Brendgen-Czerwonk",
    about = "Simple tool for reducing (removing more specifics) CIDR/IP addresses from standard input"
)]
/// Command-line interface configuration for the net-reduce tool.
///
/// This struct defines the available command-line arguments and options
pub struct Cli {
    /// File to read from, if not specified stdin is used
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Output format, can be json, yaml or list
    #[arg(short, long, value_name = "FORMAT", default_value = "list")]
    pub output_format: OutputFormat,
}
