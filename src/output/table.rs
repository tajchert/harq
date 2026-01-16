use crate::har::Entry;
use crate::output::{format_bytes, format_time, truncate};
use colored::Colorize;
use tabled::{
    settings::Style,
    Table, Tabled,
};

#[derive(Tabled)]
pub struct EntryRow {
    #[tabled(rename = "#")]
    pub index: usize,
    #[tabled(rename = "Method")]
    pub method: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "Time")]
    pub time: String,
    #[tabled(rename = "Size")]
    pub size: String,
    #[tabled(rename = "URL")]
    pub url: String,
}

impl EntryRow {
    pub fn from_entry(index: usize, entry: &Entry, color: bool, max_url_len: usize) -> Self {
        let method = if color {
            colorize_method(&entry.request.method)
        } else {
            entry.request.method.clone()
        };

        let status = if color {
            colorize_status(entry.response.status)
        } else {
            entry.response.status.to_string()
        };

        let url = truncate(&entry.request.url, max_url_len);

        Self {
            index,
            method,
            status,
            time: format_time(entry.time),
            size: format_bytes(entry.response.body_size),
            url,
        }
    }
}

fn colorize_method(method: &str) -> String {
    match method {
        "GET" => method.green().to_string(),
        "POST" => method.blue().to_string(),
        "PUT" => method.yellow().to_string(),
        "DELETE" => method.red().to_string(),
        "PATCH" => method.magenta().to_string(),
        "HEAD" => method.cyan().to_string(),
        "OPTIONS" => method.white().to_string(),
        _ => method.to_string(),
    }
}

fn colorize_status(status: i32) -> String {
    let s = status.to_string();
    match status {
        200..=299 => s.green().to_string(),
        300..=399 => s.yellow().to_string(),
        400..=499 => s.red().to_string(),
        500..=599 => s.red().bold().to_string(),
        _ => s,
    }
}

pub fn print_entries_table(entries: &[(usize, &Entry)], color: bool, max_url_len: usize) {
    if entries.is_empty() {
        println!("No entries found.");
        return;
    }

    let rows: Vec<EntryRow> = entries
        .iter()
        .map(|(i, e)| EntryRow::from_entry(*i, e, color, max_url_len))
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::rounded());

    println!("{}", table);
}

/// Print detailed view of an entry
pub fn print_entry_detail(index: usize, entry: &Entry, color: bool, show_body: bool) {
    let label = |s: &str| {
        if color {
            s.bold().to_string()
        } else {
            s.to_string()
        }
    };

    println!("{}", "=".repeat(60));
    println!("{} Entry #{}", label(">>>"), index);
    println!("{}", "=".repeat(60));

    // Request section
    println!("\n{}", label("REQUEST"));
    println!("  {} {} {}",
        colorize_method(&entry.request.method),
        entry.request.url,
        entry.request.http_version.dimmed()
    );

    if !entry.request.headers.is_empty() {
        println!("\n  {}:", label("Headers"));
        for h in &entry.request.headers {
            println!("    {}: {}", h.name.cyan(), h.value);
        }
    }

    if let Some(ref post_data) = entry.request.post_data {
        println!("\n  {}: {}", label("Content-Type"), post_data.mime_type);
        if show_body {
            if let Some(ref text) = post_data.text {
                println!("  {}:", label("Body"));
                print_body_preview(text, 500);
            }
        }
    }

    // Response section
    println!("\n{}", label("RESPONSE"));
    println!("  {} {} {}",
        colorize_status(entry.response.status),
        entry.response.status_text,
        entry.response.http_version.dimmed()
    );

    if !entry.response.headers.is_empty() {
        println!("\n  {}:", label("Headers"));
        for h in &entry.response.headers {
            println!("    {}: {}", h.name.cyan(), h.value);
        }
    }

    if show_body {
        if let Some(text) = entry.response.content.text_content() {
            println!("\n  {}:", label("Body"));
            print_body_preview(&text, 1000);
        }
    }

    // Timing section
    println!("\n{}", label("TIMING"));
    println!("  Total: {}", format_time(entry.time).yellow());
    print_timing_detail(&entry.timings);

    // Metadata
    if let Some(ref ip) = entry.server_ip_address {
        println!("\n{}: {}", label("Server IP"), ip);
    }
    println!("{}: {}", label("Started"), entry.started_date_time);
    println!();
}

fn print_body_preview(text: &str, max_len: usize) {
    let preview = if text.len() > max_len {
        format!("{}... ({} bytes total)", &text[..max_len], text.len())
    } else {
        text.to_string()
    };

    // Try to pretty-print JSON
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&preview) {
        if let Ok(pretty) = serde_json::to_string_pretty(&json) {
            for line in pretty.lines().take(30) {
                println!("    {}", line);
            }
            return;
        }
    }

    for line in preview.lines().take(30) {
        println!("    {}", line);
    }
}

fn print_timing_detail(timings: &crate::har::Timings) {
    let fmt = |v: Option<f64>| -> String {
        v.map(|t| format_time(t)).unwrap_or_else(|| "-".to_string())
    };

    println!("  blocked: {} | dns: {} | connect: {} | ssl: {}",
        fmt(timings.blocked),
        fmt(timings.dns),
        fmt(timings.connect),
        fmt(timings.ssl)
    );
    println!("  send: {} | wait: {} | receive: {}",
        fmt(timings.send),
        fmt(timings.wait),
        fmt(timings.receive)
    );
}
