use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "beat")]
#[command(about = "A terminal music player", version)]
pub struct Cli {
    /// Optional starting directory path
    pub path: Option<PathBuf>,
}

impl Cli {
    pub fn parse_args() -> Self { Self::parse() }
}
