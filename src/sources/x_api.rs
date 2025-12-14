use crate::chrome::ChromeClient;
use crate::config::SourceConfig;
use crate::sources::{HtmlSource, NewsItem};
use crate::state::State;
use anyhow::{Context, Result};

/// X/Twitter API source implementation (uses Chrome to access X.com).
pub struct XApiSource {
    id: String,
    handle: String,
    max_results: Option<usize>,
}

impl XApiSource {
    pub fn from_config(config: &SourceConfig) -> Result<Self> {
        let handle = config.extra.get("handle")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'handle' in source config"))?
            .to_string();
        
        let max_results = config.extra.get("max_results")
            .and_then(|v| v.as_u64())
            .map(|n| n as usize);

        Ok(Self {
            id: config.id.clone(),
            handle,
            max_results,
        })
    }
}

impl HtmlSource for XApiSource {
    fn id(&self) -> &str {
        &self.id
    }

    async fn fetch_html(&self, chrome: &ChromeClient) -> Result<String> {
        let url = format!("https://x.com/{}", self.handle);
        chrome.fetch_html(&url).await
    }

    fn is_fresh(&self, html: &str, state: &State) -> bool {
        // TODO: Implement freshness check for X/Twitter timeline
        true
    }

    fn extract_items(&self, html: &str) -> Result<Vec<NewsItem>> {
        // TODO: Implement parsing for X/Twitter timeline HTML
        Ok(Vec::new())
    }
}

