use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single news item extracted from a source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    /// The source ID that produced this item (e.g., "smol_ai", "anthropic_blog").
    pub source_id: String,
    /// The title or headline of the item.
    pub title: String,
    /// The URL to the full content.
    pub url: String,
    /// The publication or discovery timestamp (ISO 8601).
    pub published_at: String,
    /// Optional HTML snippet or text excerpt for LLM processing.
    pub content: Option<String>,
    /// Additional metadata (e.g., author, tags, thread ID).
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl NewsItem {
    pub fn new(source_id: String, title: String, url: String, published_at: String) -> Self {
        Self {
            source_id,
            title,
            url,
            published_at,
            content: None,
            metadata: HashMap::new(),
        }
    }
}

/// Collection of news items grouped by source for processing.
#[derive(Debug, Clone)]
pub struct NewsItems {
    pub items: Vec<NewsItem>,
}

impl NewsItems {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_items(items: Vec<NewsItem>) -> Self {
        Self { items }
    }

    pub fn group_by_source(&self) -> HashMap<String, Vec<NewsItem>> {
        let mut grouped = HashMap::new();
        for item in &self.items {
            grouped
                .entry(item.source_id.clone())
                .or_insert_with(Vec::new)
                .push(item.clone());
        }
        grouped
    }
}

impl Default for NewsItems {
    fn default() -> Self {
        Self::new()
    }
}

