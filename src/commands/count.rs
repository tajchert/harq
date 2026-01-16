use crate::har::Har;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CountCmd {
    /// HAR file to analyze (use - for stdin)
    #[arg(default_value = "-")]
    pub file: String,
}

impl CountCmd {
    pub fn run(&self, har: &Har) -> Result<()> {
        println!("{}", har.log.entries.len());
        Ok(())
    }
}
