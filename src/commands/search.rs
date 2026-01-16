use crate::har::{Har, Entry};
use crate::output::OutputFormat;
use crate::output::table::print_entries_table;
use crate::output::json::print_summaries_json;
use anyhow::Result;
use clap::Args;
use regex::Regex;

#[derive(Debug, Args)]
pub struct SearchCmd {
    /// Search pattern (text or regex with -r)
    #[arg()]
    pub pattern: String,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,

    /// Case insensitive search
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Use regex pattern
    #[arg(short = 'r', long)]
    pub regex: bool,

    /// Search in headers
    #[arg(long)]
    pub headers: bool,

    /// Search in body (response body)
    #[arg(long)]
    pub body: bool,

    /// Search only in URLs (default if no flags)
    #[arg(long)]
    pub url: bool,

    /// Invert match (show non-matching entries)
    #[arg(short = 'v', long)]
    pub invert: bool,

    /// Only count matches
    #[arg(short = 'c', long)]
    pub count: bool,

    /// Maximum URL length for table output
    #[arg(long, default_value = "60")]
    pub max_url: usize,
}

impl SearchCmd {
    pub fn run(&self, har: &Har, color: bool) -> Result<()> {
        let matcher = self.create_matcher()?;

        let entries: Vec<(usize, &Entry)> = har.log.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                let matches = self.entry_matches(e, &matcher);
                if self.invert { !matches } else { matches }
            })
            .map(|(i, e)| (i + 1, e))
            .collect();

        if self.count {
            println!("{}", entries.len());
            return Ok(());
        }

        match self.output {
            OutputFormat::Json => print_summaries_json(&entries, true)?,
            OutputFormat::Compact => {
                for (i, entry) in &entries {
                    println!("{}\t{}\t{}", i, entry.request.method, entry.request.url);
                }
            }
            OutputFormat::Table => print_entries_table(&entries, color, self.max_url),
        }

        Ok(())
    }

    fn create_matcher(&self) -> Result<Matcher> {
        if self.regex {
            let pattern = if self.ignore_case {
                format!("(?i){}", self.pattern)
            } else {
                self.pattern.clone()
            };
            let re = Regex::new(&pattern)?;
            Ok(Matcher::Regex(re))
        } else {
            Ok(Matcher::Text {
                pattern: if self.ignore_case {
                    self.pattern.to_lowercase()
                } else {
                    self.pattern.clone()
                },
                ignore_case: self.ignore_case,
            })
        }
    }

    fn entry_matches(&self, entry: &Entry, matcher: &Matcher) -> bool {
        // Default to URL search if no specific flags
        let search_url = self.url || (!self.headers && !self.body);
        let search_headers = self.headers;
        let search_body = self.body;

        if search_url && matcher.matches(&entry.request.url) {
            return true;
        }

        if search_headers {
            for h in &entry.request.headers {
                if matcher.matches(&h.name) || matcher.matches(&h.value) {
                    return true;
                }
            }
            for h in &entry.response.headers {
                if matcher.matches(&h.name) || matcher.matches(&h.value) {
                    return true;
                }
            }
        }

        if search_body {
            // Search request body
            if let Some(ref post_data) = entry.request.post_data {
                if let Some(ref text) = post_data.text {
                    if matcher.matches(text) {
                        return true;
                    }
                }
            }

            // Search response body (decode if base64)
            if let Some(text) = entry.response.content.text_content() {
                if matcher.matches(&text) {
                    return true;
                }
            }
        }

        false
    }
}

enum Matcher {
    Text { pattern: String, ignore_case: bool },
    Regex(Regex),
}

impl Matcher {
    fn matches(&self, text: &str) -> bool {
        match self {
            Matcher::Text { pattern, ignore_case } => {
                if *ignore_case {
                    text.to_lowercase().contains(pattern)
                } else {
                    text.contains(pattern)
                }
            }
            Matcher::Regex(re) => re.is_match(text),
        }
    }
}
