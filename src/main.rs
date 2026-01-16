mod har;
mod commands;
mod filter;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use output::ColorWhen;

#[derive(Parser)]
#[command(name = "harq")]
#[command(author, version, about = "A CLI tool for exploring and filtering HAR files")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Coloring: auto, always, never
    #[arg(long, global = true, default_value = "auto")]
    color: ColorWhen,
}

#[derive(Subcommand)]
enum Commands {
    /// Show HAR file metadata and summary
    Info(commands::InfoCmd),

    /// List entries in the HAR file
    #[command(alias = "ls")]
    List(commands::ListCmd),

    /// Count entries in the HAR file
    Count(commands::CountCmd),

    /// View detailed information about a specific entry
    View(commands::ViewCmd),

    /// Search entries by pattern
    Search(commands::SearchCmd),

    /// Filter entries using expressions
    Filter(commands::FilterCmd),

    /// Extract request or response body
    Body(commands::BodyCmd),

    /// Show timing breakdown for entries
    Timing(commands::TimingCmd),

    /// Show headers for entries
    Headers(commands::HeadersCmd),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let color = cli.color.should_color();

    // Configure colored output
    if !color {
        colored::control::set_override(false);
    }

    match cli.command {
        Commands::Info(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har, color)
        }
        Commands::List(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har, color)
        }
        Commands::Count(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har)
        }
        Commands::View(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har, color)
        }
        Commands::Search(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har, color)
        }
        Commands::Filter(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har)
        }
        Commands::Body(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har)
        }
        Commands::Timing(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har, color)
        }
        Commands::Headers(cmd) => {
            let har = load_har(&cmd.file)?;
            cmd.run(&har, color)
        }
    }
}

fn load_har(path: &str) -> Result<har::Har> {
    if path == "-" {
        har::parse_stdin()
    } else {
        har::parse_file(path)
    }
}
