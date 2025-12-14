use crate::chrome::ChromeClient;
use crate::config::SourceConfig;
use crate::sources::{HtmlSource, NewsItem};
use crate::state::State;
use anyhow::{Context, Result};

/// Smol.ai issues source implementation.
pub struct SmolAiSource {
    id: String,
    base_url: String,
}

impl SmolAiSource {
    pub fn from_config(config: &SourceConfig) -> Result<Self> {
        let base_url = config.extra.get("base_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'base_url' in source config"))?
            .to_string();

        Ok(Self {
            id: config.id.clone(),
            base_url,
        })
    }
}

impl HtmlSource for SmolAiSource {
    fn id(&self) -> &str {
        &self.id
    }

    async fn fetch_html(&self, chrome: &ChromeClient) -> Result<String> {
        chrome.fetch_html(&self.base_url).await
    }

    fn is_fresh(&self, html: &str, state: &State) -> bool {
        // TODO: Implement freshness check for smol.ai issues
        true
    }

    fn extract_items(&self, html: &str) -> Result<Vec<NewsItem>> {
        // TODO: Implement parsing for smol.ai issues page
        Ok(Vec::new())
    }
}

