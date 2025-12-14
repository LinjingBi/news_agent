use crate::chrome::ChromeClient;
use crate::config::Config;
use crate::sources::{HtmlSource, NewsItems};
use crate::state::State;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Runs the fetch subcommand: loads config/state, runs active HtmlSource implementations via Chrome,
/// updates state, and emits normalized items (optionally cached on disk for summarize).
pub async fn run() -> Result<()> {
    // Load config and state
    let config = Config::load().context("Failed to load config")?;
    let mut state = State::load().context("Failed to load state")?;

    // Connect to Chrome
    let chrome = ChromeClient::connect(9222)
        .await
        .context("Failed to connect to Chrome DevTools. Make sure Chrome is running with --remote-debugging-port=9222")?;

    // Collect all active sources from config
    let mut all_items = NewsItems::new();

    for source_config in &config.sources {
        if !source_config.active {
            continue;
        }

        // TODO: Instantiate the appropriate HtmlSource implementation based on source_config.type
        // For now, this is a placeholder that shows the structure
        // Each source type (smol_ai, html_scrape, x_api) will implement HtmlSource
        eprintln!("Fetching from source: {}", source_config.id);
        
        // Example structure:
        // let source: Box<dyn HtmlSource> = match source_config.type.as_str() {
        //     "smol_ai_issues" => Box::new(sources::smol_ai::SmolAiSource::from_config(source_config)?),
        //     "html_scrape" => Box::new(sources::html_scrape::HtmlScrapeSource::from_config(source_config)?),
        //     "x_api" => Box::new(sources::x_api::XApiSource::from_config(source_config)?),
        //     _ => anyhow::bail!("Unknown source type: {}", source_config.type),
        // };
        // 
        // let html = source.fetch_html(&chrome).await?;
        // if source.is_fresh(&html, &state) {
        //     let items = source.extract_items(&html)?;
        //     all_items.items.extend(items);
        //     // Update state for this source
        //     state.update_source(&source.id(), &html)?;
        // }
    }

    // Save updated state
    state.save().context("Failed to save state")?;

    // Optionally cache items to disk for summarize subcommand
    let cache_path = get_items_cache_path()?;
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&all_items.items)?;
    std::fs::write(&cache_path, json)?;

    println!("Fetched {} items from {} sources", all_items.items.len(), config.sources.iter().filter(|s| s.active).count());
    Ok(())
}

fn get_items_cache_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(format!("{}/.local/share/news_agent/cache/items.json", home)))
}

