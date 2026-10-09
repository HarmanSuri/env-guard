use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envguard")]
#[command(about = "High-performance developer secret scanner", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan staged git changes for secrets
    Scan {
        /// Custom entropy threshold (default: 4.0)
        #[arg(long, default_value_t = 4.0)]
        entropy_threshold: f64,
    },
    /// Install envguard as a git pre-commit hook
    InstallHook,
}
