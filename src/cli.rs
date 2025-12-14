pub mod fetch;
pub mod notify;
pub mod summarize;

use anyhow::Result;

/// Orchestrates the full pipeline: fetch → summarize → notify
pub async fn run_all() -> Result<()> {
    fetch::run().await?;
    summarize::run().await?;
    notify::run().await?;
    Ok(())
}

