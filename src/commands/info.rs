use crate::har::Har;
use crate::output::OutputFormat;
use anyhow::Result;
use clap::Args;
use colored::Colorize;

#[derive(Debug, Args)]
pub struct InfoCmd {
    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,
}

impl InfoCmd {
    pub fn run(&self, har: &Har, color: bool) -> Result<()> {
        match self.output {
            OutputFormat::Json => self.print_json(har),
            _ => self.print_table(har, color),
        }
    }

    fn print_table(&self, har: &Har, color: bool) -> Result<()> {
        let label = |s: &str| {
            if color {
                s.bold().to_string()
            } else {
                s.to_string()
            }
        };

        println!("{}", label("HAR File Information"));
        println!("{}", "─".repeat(40));

        // Version
        println!("{}: {}", label("Version"), har.log.version);

        // Creator
        println!("{}: {} v{}",
            label("Creator"),
            har.log.creator.name,
            har.log.creator.version
        );

        // Browser (if present)
        if let Some(ref browser) = har.log.browser {
            println!("{}: {} v{}",
                label("Browser"),
                browser.name,
                browser.version
            );
        }

        // Pages
        if let Some(ref pages) = har.log.pages {
            println!("{}: {}", label("Pages"), pages.len());
            for page in pages.iter().take(5) {
                println!("  - {} ({})", page.title, page.id);
            }
            if pages.len() > 5 {
                println!("  ... and {} more", pages.len() - 5);
            }
        }

        // Entries summary
        println!("{}: {}", label("Entries"), har.log.entries.len());

        // Method breakdown
        let mut methods: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for entry in &har.log.entries {
            *methods.entry(&entry.request.method).or_insert(0) += 1;
        }

        if !methods.is_empty() {
            println!("{}: ", label("Methods"));
            let mut methods: Vec<_> = methods.into_iter().collect();
            methods.sort_by(|a, b| b.1.cmp(&a.1));
            for (method, count) in methods {
                println!("  {}: {}", method, count);
            }
        }

        // Status code breakdown
        let mut statuses: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        for entry in &har.log.entries {
            *statuses.entry(entry.response.status).or_insert(0) += 1;
        }

        if !statuses.is_empty() {
            println!("{}: ", label("Status Codes"));
            let mut statuses: Vec<_> = statuses.into_iter().collect();
            statuses.sort_by(|a, b| a.0.cmp(&b.0));
            for (status, count) in statuses {
                let status_str = if color {
                    match status {
                        200..=299 => status.to_string().green().to_string(),
                        300..=399 => status.to_string().yellow().to_string(),
                        400..=599 => status.to_string().red().to_string(),
                        _ => status.to_string(),
                    }
                } else {
                    status.to_string()
                };
                println!("  {}: {}", status_str, count);
            }
        }

        // Timing summary
        if !har.log.entries.is_empty() {
            let times: Vec<f64> = har.log.entries.iter().map(|e| e.time).collect();
            let total: f64 = times.iter().sum();
            let avg = total / times.len() as f64;
            let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min = times.iter().cloned().fold(f64::INFINITY, f64::min);

            println!("{}: ", label("Timing"));
            println!("  Total: {:.0}ms", total);
            println!("  Average: {:.0}ms", avg);
            println!("  Min: {:.0}ms, Max: {:.0}ms", min, max);
        }

        Ok(())
    }

    fn print_json(&self, har: &Har) -> Result<()> {
        #[derive(serde::Serialize)]
        struct Info {
            version: String,
            creator: CreatorInfo,
            browser: Option<CreatorInfo>,
            pages_count: usize,
            entries_count: usize,
            methods: std::collections::HashMap<String, usize>,
            status_codes: std::collections::HashMap<i32, usize>,
        }

        #[derive(serde::Serialize)]
        struct CreatorInfo {
            name: String,
            version: String,
        }

        let mut methods: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for entry in &har.log.entries {
            *methods.entry(entry.request.method.clone()).or_insert(0) += 1;
        }

        let mut status_codes: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        for entry in &har.log.entries {
            *status_codes.entry(entry.response.status).or_insert(0) += 1;
        }

        let info = Info {
            version: har.log.version.clone(),
            creator: CreatorInfo {
                name: har.log.creator.name.clone(),
                version: har.log.creator.version.clone(),
            },
            browser: har.log.browser.as_ref().map(|b| CreatorInfo {
                name: b.name.clone(),
                version: b.version.clone(),
            }),
            pages_count: har.log.pages.as_ref().map(|p| p.len()).unwrap_or(0),
            entries_count: har.log.entries.len(),
            methods,
            status_codes,
        };

        println!("{}", serde_json::to_string_pretty(&info)?);
        Ok(())
    }
}
