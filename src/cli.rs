use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// MySQL/MariaDB query tool with structured output
#[derive(Parser)]
#[command(name = "gold_digger")]
#[command(about = "A MySQL/MariaDB query tool that exports results to structured data files")]
#[command(version)]
pub struct Cli {
    /// Database connection URL
    #[arg(long, env = "DATABASE_URL")]
    pub db_url: Option<String>,

    /// SQL query string
    #[arg(long, conflicts_with = "query_file")]
    pub query: Option<String>,

    /// File containing SQL query
    #[arg(long, conflicts_with = "query")]
    pub query_file: Option<PathBuf>,

    /// Output file path
    #[arg(short, long, env = "OUTPUT_FILE")]
    pub output: Option<PathBuf>,

    /// Output format override
    #[arg(long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress all output except errors
    #[arg(long, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Pretty-print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Exit successfully on empty result sets
    #[arg(long)]
    pub allow_empty: bool,

    /// Print current configuration as JSON
    #[arg(long)]
    pub dump_config: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completion scripts
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Csv,
    Json,
    Tsv,
}

impl OutputFormat {
    pub fn from_extension(path: &std::path::Path) -> Self {
        match path.extension().and_then(|s| s.to_str()) {
            Some("csv") => Self::Csv,
            Some("json") => Self::Json,
            _ => Self::Tsv, // Default fallback
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Tsv => "tsv",
        }
    }
}
