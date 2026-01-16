use crate::har::{Entry, Har};
use anyhow::Result;
use serde::Serialize;

/// Output entries as JSON array
pub fn print_entries_json(entries: &[(usize, &Entry)], pretty: bool) -> Result<()> {
    let json: Vec<&Entry> = entries.iter().map(|(_, e)| *e).collect();

    let output = if pretty {
        serde_json::to_string_pretty(&json)?
    } else {
        serde_json::to_string(&json)?
    };

    println!("{}", output);
    Ok(())
}

/// Output single entry as JSON
pub fn print_entry_json(entry: &Entry, pretty: bool) -> Result<()> {
    let output = if pretty {
        serde_json::to_string_pretty(entry)?
    } else {
        serde_json::to_string(entry)?
    };

    println!("{}", output);
    Ok(())
}

/// Output full HAR as JSON (for filtered output)
pub fn print_har_json(har: &Har, pretty: bool) -> Result<()> {
    let output = if pretty {
        serde_json::to_string_pretty(har)?
    } else {
        serde_json::to_string(har)?
    };

    println!("{}", output);
    Ok(())
}

/// Simplified entry summary for compact JSON output
#[derive(Serialize)]
pub struct EntrySummary {
    pub index: usize,
    pub method: String,
    pub url: String,
    pub status: i32,
    pub time_ms: f64,
    pub body_size: i64,
    pub content_type: Option<String>,
}

impl EntrySummary {
    pub fn from_entry(index: usize, entry: &Entry) -> Self {
        Self {
            index,
            method: entry.request.method.clone(),
            url: entry.request.url.clone(),
            status: entry.response.status,
            time_ms: entry.time,
            body_size: entry.response.body_size,
            content_type: entry.content_type().map(|s| s.to_string()),
        }
    }
}

/// Output entry summaries as JSON
pub fn print_summaries_json(entries: &[(usize, &Entry)], pretty: bool) -> Result<()> {
    let summaries: Vec<EntrySummary> = entries
        .iter()
        .map(|(i, e)| EntrySummary::from_entry(*i, e))
        .collect();

    let output = if pretty {
        serde_json::to_string_pretty(&summaries)?
    } else {
        serde_json::to_string(&summaries)?
    };

    println!("{}", output);
    Ok(())
}

/// Create a filtered HAR with only selected entries
pub fn create_filtered_har(har: &Har, entries: &[(usize, &Entry)]) -> Har {
    Har {
        log: crate::har::Log {
            version: har.log.version.clone(),
            creator: har.log.creator.clone(),
            browser: har.log.browser.clone(),
            pages: har.log.pages.clone(),
            entries: entries.iter().map(|(_, e)| (*e).clone()).collect(),
            comment: har.log.comment.clone(),
        },
    }
}
