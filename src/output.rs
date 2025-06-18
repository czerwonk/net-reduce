use std::io::Write;

use anyhow::{Context, Result};

pub fn to_stdout(lines: Vec<String>) -> Result<()> {
    write_output(&mut std::io::stdout(), lines).context("failed to write to stdout")
}

fn write_output<W: Write>(writer: &mut W, lines: Vec<String>) -> Result<()> {
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}
