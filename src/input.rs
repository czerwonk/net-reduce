use anyhow::Result;

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads input lines from standard input (stdin).
///
/// This function reads all lines from stdin until EOF is reached and collects them in a vector of
/// strings.
///
/// # Returns
///
/// Returns `Ok(Vec<String>)` containing all input lines on success,
/// or an `Err` if an I/O error occurs during reading.
///
/// # Examples
///
/// ```no_run
/// use net_reduce::input;
///
/// let lines = input::from_stdin().expect("Failed to read from stdin");
/// ```
pub fn from_stdin() -> Result<Vec<String>> {
    read_lines(std::io::stdin().lock())
}

/// Reads input lines from a specified file.
///
/// This function opens the file at the given path and reads all lines,
/// collecting them into a vector of strings
///
/// # Arguments
///
/// * `path` - The file system path to the input file
///
/// # Returns
///
/// Returns `Ok(Vec<String>)` containing all lines from the file on success,
/// or an `Err` if the file cannot be opened or an I/O error occurs.
///
/// # Examples
///
/// ```no_run
/// use net_reduce::input;
///
/// let lines = input::from_file("/path/to/cidrs.txt")
///     .expect("Failed to read file");
/// ```
pub fn from_file(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    read_lines(reader)
}

fn read_lines<R: BufRead>(reader: R) -> Result<Vec<String>> {
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
