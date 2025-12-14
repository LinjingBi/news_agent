use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Structured summary output from the summarization pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    /// Per-source summaries (source_id -> summary text).
    pub by_source: HashMap<String, String>,
    /// Overall summary or introduction.
    pub overview: Option<String>,
    /// Extracted links (papers, repos, threads) grouped by category.
    #[serde(default)]
    pub links: HashMap<String, Vec<String>>,
    /// Warnings or errors encountered during summarization.
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            by_source: HashMap::new(),
            overview: None,
            links: HashMap::new(),
            warnings: Vec::new(),
        }
    }
}

impl Default for Summary {
    fn default() -> Self {
        Self::new()
    }
}

/// A formatted report ready for notification delivery.
#[derive(Debug, Clone)]
pub struct Report {
    /// The date this report was generated (YYYY-MM-DD).
    pub date: String,
    /// The full Markdown content of the report.
    pub markdown: String,
    /// Metadata about the run (timestamps, source counts, etc.).
    pub metadata: ReportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub fetch_started_at: String,
    pub fetch_completed_at: Option<String>,
    pub summarize_started_at: Option<String>,
    pub summarize_completed_at: Option<String>,
    pub notify_started_at: Option<String>,
    pub notify_completed_at: Option<String>,
    pub source_count: usize,
    pub item_count: usize,
}

impl ReportMetadata {
    pub fn new(fetch_started_at: String) -> Self {
        Self {
            fetch_started_at,
            fetch_completed_at: None,
            summarize_started_at: None,
            summarize_completed_at: None,
            notify_started_at: None,
            notify_completed_at: None,
            source_count: 0,
            item_count: 0,
        }
    }
}

