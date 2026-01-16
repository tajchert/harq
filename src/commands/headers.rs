use crate::har::Har;
use crate::output::OutputFormat;
use anyhow::{Result, bail};
use clap::Args;
use colored::Colorize;

#[derive(Debug, Args)]
pub struct HeadersCmd {
    /// Entry index (1-based), or "all" for all entries
    #[arg()]
    pub index: String,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,

    /// Show only request headers
    #[arg(long)]
    pub request: bool,

    /// Show only response headers
    #[arg(long)]
    pub response: bool,

    /// Filter headers by name (case-insensitive contains)
    #[arg(short = 'f', long)]
    pub filter: Option<String>,
}

impl HeadersCmd {
    pub fn run(&self, har: &Har, color: bool) -> Result<()> {
        if self.index == "all" {
            return self.show_all_headers(har, color);
        }

        let idx: usize = self.index.parse()
            .map_err(|_| anyhow::anyhow!("Invalid index: {}. Use a number or 'all'", self.index))?;

        if idx == 0 || idx > har.log.entries.len() {
            bail!(
                "Entry index {} out of range (1-{})",
                idx,
                har.log.entries.len()
            );
        }

        let entry = &har.log.entries[idx - 1];
        self.show_entry_headers(idx, entry, color)
    }

    fn show_entry_headers(&self, index: usize, entry: &crate::har::Entry, color: bool) -> Result<()> {
        let show_request = self.request || (!self.request && !self.response);
        let show_response = self.response || (!self.request && !self.response);

        match self.output {
            OutputFormat::Json => {
                #[derive(serde::Serialize)]
                struct Headers {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    request: Option<Vec<HeaderPair>>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    response: Option<Vec<HeaderPair>>,
                }

                #[derive(serde::Serialize)]
                struct HeaderPair {
                    name: String,
                    value: String,
                }

                let filter_fn = |h: &crate::har::Header| -> bool {
                    self.filter.as_ref().map_or(true, |f| {
                        h.name.to_lowercase().contains(&f.to_lowercase())
                    })
                };

                let headers = Headers {
                    request: if show_request {
                        Some(entry.request.headers.iter()
                            .filter(|h| filter_fn(h))
                            .map(|h| HeaderPair {
                                name: h.name.clone(),
                                value: h.value.clone(),
                            })
                            .collect())
                    } else {
                        None
                    },
                    response: if show_response {
                        Some(entry.response.headers.iter()
                            .filter(|h| filter_fn(h))
                            .map(|h| HeaderPair {
                                name: h.name.clone(),
                                value: h.value.clone(),
                            })
                            .collect())
                    } else {
                        None
                    },
                };

                println!("{}", serde_json::to_string_pretty(&headers)?);
            }
            _ => {
                let label = |s: &str| {
                    if color {
                        s.bold().to_string()
                    } else {
                        s.to_string()
                    }
                };

                println!("{} Entry #{}", label(">>>"), index);
                println!("{} {}", entry.request.method, entry.request.url);
                println!();

                if show_request {
                    println!("{}", label("Request Headers:"));
                    for h in &entry.request.headers {
                        if self.matches_filter(&h.name) {
                            println!("  {}: {}",
                                if color { h.name.cyan().to_string() } else { h.name.clone() },
                                h.value
                            );
                        }
                    }
                    println!();
                }

                if show_response {
                    println!("{}", label("Response Headers:"));
                    for h in &entry.response.headers {
                        if self.matches_filter(&h.name) {
                            println!("  {}: {}",
                                if color { h.name.cyan().to_string() } else { h.name.clone() },
                                h.value
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn show_all_headers(&self, har: &Har, color: bool) -> Result<()> {
        for (i, entry) in har.log.entries.iter().enumerate() {
            self.show_entry_headers(i + 1, entry, color)?;
            println!();
        }
        Ok(())
    }

    fn matches_filter(&self, name: &str) -> bool {
        self.filter.as_ref().map_or(true, |f| {
            name.to_lowercase().contains(&f.to_lowercase())
        })
    }
}
