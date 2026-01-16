use crate::har::{Har, Entry};
use crate::filter::eval::FilterExpr;
use crate::output::json::create_filtered_har;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct FilterCmd {
    /// Filter expression (e.g., 'status >= 400', 'method == "POST"')
    #[arg()]
    pub expr: String,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output as valid HAR (default), otherwise output JSON array of entries
    #[arg(long)]
    pub entries_only: bool,
}

impl FilterCmd {
    pub fn run(&self, har: &Har) -> Result<()> {
        let filter = FilterExpr::parse(&self.expr)?;

        let matching_entries: Vec<(usize, &Entry)> = har.log.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| filter.matches(e))
            .map(|(i, e)| (i + 1, e))
            .collect();

        if self.entries_only {
            let entries: Vec<&Entry> = matching_entries.iter().map(|(_, e)| *e).collect();
            println!("{}", serde_json::to_string_pretty(&entries)?);
        } else {
            // Output as valid HAR
            let filtered = create_filtered_har(har, &matching_entries);
            println!("{}", serde_json::to_string_pretty(&filtered)?);
        }

        Ok(())
    }
}
