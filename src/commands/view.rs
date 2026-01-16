use crate::har::Har;
use crate::output::OutputFormat;
use crate::output::table::print_entry_detail;
use crate::output::json::print_entry_json;
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Args)]
pub struct ViewCmd {
    /// Entry index (1-based) to view
    #[arg()]
    pub index: usize,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,

    /// Show full body content
    #[arg(long)]
    pub full: bool,

    /// Hide response body
    #[arg(long)]
    pub no_body: bool,

    /// Show only headers
    #[arg(long)]
    pub headers_only: bool,
}

impl ViewCmd {
    pub fn run(&self, har: &Har, color: bool) -> Result<()> {
        if self.index == 0 || self.index > har.log.entries.len() {
            bail!(
                "Entry index {} out of range (1-{})",
                self.index,
                har.log.entries.len()
            );
        }

        let entry = &har.log.entries[self.index - 1];

        match self.output {
            OutputFormat::Json => print_entry_json(entry, true)?,
            _ => {
                let show_body = self.full && !self.no_body && !self.headers_only;
                print_entry_detail(self.index, entry, color, show_body);
            }
        }

        Ok(())
    }
}
