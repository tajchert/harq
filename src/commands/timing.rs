use crate::har::Har;
use crate::output::{format_time, extract_host, OutputFormat};
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

#[derive(Debug, Args)]
pub struct TimingCmd {
    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub output: OutputFormat,

    /// Sort by field: time, dns, connect, wait, receive
    #[arg(short, long)]
    pub sort: Option<String>,

    /// Reverse sort order
    #[arg(short = 'R', long)]
    pub reverse: bool,

    /// Show statistics summary
    #[arg(long)]
    pub stats: bool,

    /// Limit output
    #[arg(short, long)]
    pub limit: Option<usize>,
}

#[derive(Tabled)]
struct TimingRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Host")]
    host: String,
    #[tabled(rename = "Total")]
    total: String,
    #[tabled(rename = "Blocked")]
    blocked: String,
    #[tabled(rename = "DNS")]
    dns: String,
    #[tabled(rename = "Connect")]
    connect: String,
    #[tabled(rename = "SSL")]
    ssl: String,
    #[tabled(rename = "Send")]
    send: String,
    #[tabled(rename = "Wait")]
    wait: String,
    #[tabled(rename = "Receive")]
    receive: String,
}

impl TimingCmd {
    pub fn run(&self, har: &Har, color: bool) -> Result<()> {
        if self.stats {
            return self.print_stats(har, color);
        }

        match self.output {
            OutputFormat::Json => self.print_json(har),
            _ => self.print_table(har, color),
        }
    }

    fn print_table(&self, har: &Har, _color: bool) -> Result<()> {
        let mut entries: Vec<(usize, &crate::har::Entry)> = har.log.entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i + 1, e))
            .collect();

        // Sort if requested
        if let Some(ref sort_field) = self.sort {
            entries.sort_by(|a, b| {
                let get_val = |e: &crate::har::Entry| -> f64 {
                    match sort_field.as_str() {
                        "time" | "total" => e.time,
                        "dns" => e.timings.dns.unwrap_or(-1.0),
                        "connect" => e.timings.connect.unwrap_or(-1.0),
                        "ssl" => e.timings.ssl.unwrap_or(-1.0),
                        "wait" => e.timings.wait.unwrap_or(-1.0),
                        "receive" => e.timings.receive.unwrap_or(-1.0),
                        "blocked" => e.timings.blocked.unwrap_or(-1.0),
                        "send" => e.timings.send.unwrap_or(-1.0),
                        _ => e.time,
                    }
                };

                let cmp = get_val(a.1).partial_cmp(&get_val(b.1)).unwrap_or(std::cmp::Ordering::Equal);
                if self.reverse { cmp } else { cmp.reverse() }
            });
        }

        // Apply limit
        if let Some(limit) = self.limit {
            entries.truncate(limit);
        }

        let fmt = |v: Option<f64>| -> String {
            v.filter(|&t| t >= 0.0)
                .map(format_time)
                .unwrap_or_else(|| "-".to_string())
        };

        let rows: Vec<TimingRow> = entries
            .iter()
            .map(|(i, e)| {
                let host = extract_host(&e.request.url);
                TimingRow {
                    index: *i,
                    host: if host.len() > 30 {
                        format!("{}...", &host[..27])
                    } else {
                        host.to_string()
                    },
                    total: format_time(e.time),
                    blocked: fmt(e.timings.blocked),
                    dns: fmt(e.timings.dns),
                    connect: fmt(e.timings.connect),
                    ssl: fmt(e.timings.ssl),
                    send: fmt(e.timings.send),
                    wait: fmt(e.timings.wait),
                    receive: fmt(e.timings.receive),
                }
            })
            .collect();

        let mut table = Table::new(rows);
        table.with(Style::rounded());
        println!("{}", table);

        Ok(())
    }

    fn print_stats(&self, har: &Har, color: bool) -> Result<()> {
        let entries = &har.log.entries;

        if entries.is_empty() {
            println!("No entries.");
            return Ok(());
        }

        let label = |s: &str| {
            if color {
                s.bold().to_string()
            } else {
                s.to_string()
            }
        };

        let times: Vec<f64> = entries.iter().map(|e| e.time).collect();
        let total: f64 = times.iter().sum();
        let avg = total / times.len() as f64;
        let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = times.iter().cloned().fold(f64::INFINITY, f64::min);

        // Find slowest entry
        let (slowest_idx, slowest) = entries
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.time.partial_cmp(&b.1.time).unwrap())
            .unwrap();

        println!("{}", label("Timing Statistics"));
        println!("{}", "─".repeat(40));
        println!("{}: {}", label("Total requests"), entries.len());
        println!("{}: {}", label("Total time"), format_time(total));
        println!("{}: {}", label("Average time"), format_time(avg));
        println!("{}: {}", label("Min time"), format_time(min));
        println!("{}: {}", label("Max time"), format_time(max));
        println!();
        println!("{}: #{} {} ({})",
            label("Slowest request"),
            slowest_idx + 1,
            format_time(slowest.time).yellow(),
            extract_host(&slowest.request.url)
        );

        // Timing breakdown averages
        let mut dns_sum = 0.0;
        let mut dns_count = 0;
        let mut connect_sum = 0.0;
        let mut connect_count = 0;
        let mut wait_sum = 0.0;
        let mut wait_count = 0;

        for e in entries {
            if let Some(dns) = e.timings.dns {
                if dns >= 0.0 {
                    dns_sum += dns;
                    dns_count += 1;
                }
            }
            if let Some(connect) = e.timings.connect {
                if connect >= 0.0 {
                    connect_sum += connect;
                    connect_count += 1;
                }
            }
            if let Some(wait) = e.timings.wait {
                if wait >= 0.0 {
                    wait_sum += wait;
                    wait_count += 1;
                }
            }
        }

        println!();
        println!("{}", label("Average breakdown"));
        if dns_count > 0 {
            println!("  DNS: {}", format_time(dns_sum / dns_count as f64));
        }
        if connect_count > 0 {
            println!("  Connect: {}", format_time(connect_sum / connect_count as f64));
        }
        if wait_count > 0 {
            println!("  Wait: {}", format_time(wait_sum / wait_count as f64));
        }

        Ok(())
    }

    fn print_json(&self, har: &Har) -> Result<()> {
        #[derive(serde::Serialize)]
        struct TimingInfo {
            index: usize,
            url: String,
            total_ms: f64,
            blocked_ms: Option<f64>,
            dns_ms: Option<f64>,
            connect_ms: Option<f64>,
            ssl_ms: Option<f64>,
            send_ms: Option<f64>,
            wait_ms: Option<f64>,
            receive_ms: Option<f64>,
        }

        let timings: Vec<TimingInfo> = har.log.entries
            .iter()
            .enumerate()
            .map(|(i, e)| TimingInfo {
                index: i + 1,
                url: e.request.url.clone(),
                total_ms: e.time,
                blocked_ms: e.timings.blocked,
                dns_ms: e.timings.dns,
                connect_ms: e.timings.connect,
                ssl_ms: e.timings.ssl,
                send_ms: e.timings.send,
                wait_ms: e.timings.wait,
                receive_ms: e.timings.receive,
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&timings)?);
        Ok(())
    }
}
