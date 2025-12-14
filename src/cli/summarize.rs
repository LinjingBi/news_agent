use crate::sources::models::NewsItem;
use crate::summarize::{Report, Summarizer};
use anyhow::{Context, Result};
use serde_json;
use std::path::PathBuf;

/// Runs the summarize subcommand: reads normalized items (from fetch output),
/// calls the configured Summarizer, and writes the Markdown report.
pub async fn run() -> Result<()> {
    // Load cached items from fetch
    let items = load_cached_items().context("Failed to load cached items. Run 'fetch' first.")?;

    if items.is_empty() {
        println!("No items to summarize");
        return Ok(());
    }

    // TODO: Instantiate the configured Summarizer
    // For now, this is a placeholder
    // let summarizer: Box<dyn Summarizer> = match config.summarizer_type.as_str() {
    //     "openrouter" => Box::new(summarize::openrouter::OpenRouterSummarizer::new(&config)?),
    //     "anthropic" => Box::new(summarize::anthropic::AnthropicSummarizer::new(&config)?),
    //     _ => anyhow::bail!("Unknown summarizer type"),
    // };
    //
    // let summary = summarizer.summarize(&items).await?;

    // Generate report from summary
    // For now, create a placeholder report
    let report = generate_report(&items)?;

    // Write report to disk
    let report_path = get_report_path()?;
    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&report_path, &report.markdown)?;

    println!("Generated report: {}", report_path.display());
    Ok(())
}

fn load_cached_items() -> Result<Vec<NewsItem>> {
    let cache_path = get_items_cache_path()?;
    let json = std::fs::read_to_string(&cache_path)?;
    let items: Vec<NewsItem> = serde_json::from_str(&json)?;
    Ok(items)
}

fn get_items_cache_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(format!("{}/.local/share/news_agent/cache/items.json", home)))
}

fn get_report_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    let date = chrono::Local::now().format("%Y-%m-%d");
    Ok(PathBuf::from(format!("{}/.local/share/news_agent/reports/{}.md", home, date)))
}

fn generate_report(items: &[NewsItem]) -> Result<Report> {
    use crate::summarize::models::ReportMetadata;
    use chrono::Local;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let fetch_started_at = Local::now().to_rfc3339();

    let mut markdown = format!("# Daily News Report — {}\n\n", date);
    markdown.push_str("## Sources\n\n");

    // Group items by source
    let mut by_source: std::collections::HashMap<String, Vec<&NewsItem>> = std::collections::HashMap::new();
    for item in items {
        by_source.entry(item.source_id.clone()).or_insert_with(Vec::new).push(item);
    }

    for (source_id, source_items) in &by_source {
        markdown.push_str(&format!("### {}\n", source_id));
        for item in source_items {
            markdown.push_str(&format!("- {}: [{}]({})\n", item.published_at, item.title, item.url));
        }
        markdown.push_str("\n");
    }

    markdown.push_str("## Warnings & Errors\n\n");
    markdown.push_str("## Run Metadata\n\n");
    markdown.push_str(&format!("- Fetch started at {}\n", fetch_started_at));

    let metadata = ReportMetadata::new(fetch_started_at);
    metadata.item_count = items.len();
    metadata.source_count = by_source.len();

    Ok(Report {
        date,
        markdown,
        metadata,
    })
}

