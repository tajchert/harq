pub mod table;
pub mod json;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Compact,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ColorWhen {
    #[default]
    Auto,
    Always,
    Never,
}

impl ColorWhen {
    pub fn should_color(&self) -> bool {
        match self {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => atty::is(atty::Stream::Stdout),
        }
    }
}

/// Truncate a string to max length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s.chars().take(max_len).collect()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Format milliseconds as human readable
pub fn format_time(ms: f64) -> String {
    if ms < 0.0 {
        "-".to_string()
    } else if ms < 1000.0 {
        format!("{:.0}ms", ms)
    } else if ms < 60000.0 {
        format!("{:.2}s", ms / 1000.0)
    } else {
        format!("{:.2}m", ms / 60000.0)
    }
}

/// Format bytes as human readable
pub fn format_bytes(bytes: i64) -> String {
    if bytes < 0 {
        "-".to_string()
    } else if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Extract host from URL
pub fn extract_host(url: &str) -> &str {
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or(url)
}

/// Extract path from URL
pub fn extract_path(url: &str) -> &str {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    without_scheme
        .find('/')
        .map(|i| &without_scheme[i..])
        .unwrap_or("/")
}
