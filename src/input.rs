use anyhow::Result;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn from_stdin() -> Result<Vec<String>> {
    read_lines(std::io::stdin().lock())
}

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
