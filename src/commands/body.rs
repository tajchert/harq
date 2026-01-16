use crate::har::Har;
use anyhow::{Result, bail};
use clap::Args;
use std::io::{self, Write};

#[derive(Debug, Args)]
pub struct BodyCmd {
    /// Entry index (1-based)
    #[arg()]
    pub index: usize,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Get request body instead of response
    #[arg(long)]
    pub request: bool,

    /// Pretty print JSON bodies
    #[arg(short, long)]
    pub pretty: bool,

    /// Output raw bytes (for binary content)
    #[arg(long)]
    pub raw: bool,
}

impl BodyCmd {
    pub fn run(&self, har: &Har) -> Result<()> {
        if self.index == 0 || self.index > har.log.entries.len() {
            bail!(
                "Entry index {} out of range (1-{})",
                self.index,
                har.log.entries.len()
            );
        }

        let entry = &har.log.entries[self.index - 1];

        if self.request {
            self.output_request_body(entry)
        } else {
            self.output_response_body(entry)
        }
    }

    fn output_request_body(&self, entry: &crate::har::Entry) -> Result<()> {
        let Some(ref post_data) = entry.request.post_data else {
            bail!("Entry {} has no request body", self.index);
        };

        let Some(ref text) = post_data.text else {
            bail!("Entry {} has no request body text", self.index);
        };

        if self.pretty && post_data.mime_type.contains("json") {
            self.pretty_print_json(text)?;
        } else {
            println!("{}", text);
        }

        Ok(())
    }

    fn output_response_body(&self, entry: &crate::har::Entry) -> Result<()> {
        let content = &entry.response.content;

        // Get decoded bytes
        let Some(bytes) = content.decoded_text() else {
            bail!("Entry {} has no response body", self.index);
        };

        if self.raw {
            // Output raw bytes to stdout
            io::stdout().write_all(&bytes)?;
            return Ok(());
        }

        // Convert to string
        let text = String::from_utf8_lossy(&bytes);

        if self.pretty {
            let mime = content.mime_type.as_deref().unwrap_or("");
            if mime.contains("json") {
                self.pretty_print_json(&text)?;
                return Ok(());
            }
        }

        println!("{}", text);
        Ok(())
    }

    fn pretty_print_json(&self, text: &str) -> Result<()> {
        match serde_json::from_str::<serde_json::Value>(text) {
            Ok(json) => {
                println!("{}", serde_json::to_string_pretty(&json)?);
            }
            Err(_) => {
                // Not valid JSON, print as-is
                println!("{}", text);
            }
        }
        Ok(())
    }
}
