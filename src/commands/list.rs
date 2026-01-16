use crate::har::Har;
use crate::output::OutputFormat;
use crate::output::table::print_entries_table;
use crate::output::json::print_summaries_json;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct ListCmd {
    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,

    /// Limit number of entries shown
    #[arg(short = 'n', long)]
    pub limit: Option<usize>,

    /// Show first N entries
    #[arg(long)]
    pub head: Option<usize>,

    /// Show last N entries
    #[arg(long)]
    pub tail: Option<usize>,

    /// Maximum URL length before truncation
    #[arg(long, default_value = "60")]
    pub max_url: usize,

    /// Long format (more columns)
    #[arg(short = 'l', long)]
    pub long: bool,
}

impl ListCmd {
    pub fn run(&self, har: &Har, color: bool) -> Result<()> {
        let entries: Vec<(usize, &crate::har::Entry)> = har.log.entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i + 1, e))
            .collect();

        // Apply head/tail/limit
        let entries = self.apply_limits(entries);

        match self.output {
            OutputFormat::Json => print_summaries_json(&entries, true)?,
            OutputFormat::Compact => self.print_compact(&entries)?,
            OutputFormat::Table => print_entries_table(&entries, color, self.max_url),
        }

        Ok(())
    }

    fn apply_limits<'a>(&self, entries: Vec<(usize, &'a crate::har::Entry)>) -> Vec<(usize, &'a crate::har::Entry)> {
        let len = entries.len();

        if let Some(head) = self.head {
            return entries.into_iter().take(head).collect();
        }

        if let Some(tail) = self.tail {
            return entries.into_iter().skip(len.saturating_sub(tail)).collect();
        }

        if let Some(limit) = self.limit {
            return entries.into_iter().take(limit).collect();
        }

        entries
    }

    fn print_compact(&self, entries: &[(usize, &crate::har::Entry)]) -> Result<()> {
        for (i, entry) in entries {
            println!("{}\t{}\t{}\t{:.0}ms\t{}",
                i,
                entry.request.method,
                entry.response.status,
                entry.time,
                entry.request.url
            );
        }
        Ok(())
    }
}
