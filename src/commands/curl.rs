use crate::har::Har;
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Args)]
pub struct CurlCmd {
    /// Entry index (1-based)
    #[arg()]
    pub index: usize,

    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,
}

impl CurlCmd {
    pub fn run(&self, har: &Har) -> Result<()> {
        if self.index == 0 || self.index > har.log.entries.len() {
            bail!(
                "Entry index {} out of range (1-{})",
                self.index,
                har.log.entries.len()
            );
        }

        let entry = &har.log.entries[self.index - 1];
        let mut parts = vec![format!(
            "curl -X {} {}",
            entry.request.method,
            shell_quote(&entry.request.url)
        )];

        for header in &entry.request.headers {
            parts.push(format!(
                "-H {}",
                shell_quote(&format!("{}: {}", header.name, header.value))
            ));
        }

        if let Some(post_data) = &entry.request.post_data {
            if let Some(bytes) = post_data.decoded_text() {
                let body = String::from_utf8_lossy(&bytes);
                parts.push(format!("--data-raw {}", shell_quote(&body)));
            }
        }

        println!("{}", join_curl_parts(&parts));
        Ok(())
    }
}

fn join_curl_parts(parts: &[String]) -> String {
    let Some((first, rest)) = parts.split_first() else {
        return String::new();
    };

    if rest.is_empty() {
        return first.clone();
    }

    let mut output = first.clone();
    for part in rest {
        output.push_str(" \\\n  ");
        output.push_str(part);
    }
    output
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
