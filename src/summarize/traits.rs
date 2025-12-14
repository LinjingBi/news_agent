use crate::sources::models::NewsItem;
use crate::summarize::models::Summary;
use anyhow::Result;

/// Trait for summarization backends (LLM APIs, local models, etc.).
pub trait Summarizer {
    /// Summarizes a collection of news items and returns structured summary data.
    /// The summary can include per-item summaries, grouped sections, or a full report.
    async fn summarize(&self, items: &[NewsItem]) -> Result<Summary>;
}

