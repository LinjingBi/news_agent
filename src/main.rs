mod chrome;
mod cli;
mod config;
mod errors;
mod logging;
mod notify;
mod sources;
mod state;
mod summarize;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "news_agent")]
#[command(about = "A Rust-based personal news collector that fetches daily updates from multiple AI-related sources")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch news items from all active sources via Chrome DevTools
    Fetch,
    /// Summarize fetched items using LLM
    Summarize,
    /// Send notifications for the generated report
    Notify,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Fetch) => cli::fetch::run().await,
        Some(Commands::Summarize) => cli::summarize::run().await,
        Some(Commands::Notify) => cli::notify::run().await,
        None => {
            // Root command: run the full pipeline
            cli::run_all().await
        }
    }
}
