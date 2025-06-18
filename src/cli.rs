use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(name = "net-reduce")]
#[command(author = "Daniel Brendgen-Czerwonk")]
#[command(
    about = "Simple tool for reducing (removing more specifics) CIDR/IP addresses from standard input"
)]
/// Command-line interface configuration for the net-reduce tool.
///
/// This struct defines the available command-line arguments and options
pub struct Cli {
    /// File to read from, if not specified stdin is used
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,
}
