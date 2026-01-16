pub mod types;

pub use types::*;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Parse a HAR file from path
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Har> {
    let path = path.as_ref();
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);
    parse_reader(reader)
}

/// Parse HAR from a reader
pub fn parse_reader<R: Read>(reader: R) -> Result<Har> {
    serde_json::from_reader(reader).context("Failed to parse HAR file")
}

/// Parse HAR from a string
pub fn parse_str(s: &str) -> Result<Har> {
    serde_json::from_str(s).context("Failed to parse HAR JSON")
}

/// Parse HAR from stdin
pub fn parse_stdin() -> Result<Har> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    parse_reader(reader)
}
