use crate::chrome::ChromeClient;
use crate::state::State;
use crate::sources::models::NewsItem;
use anyhow::Result;

/// Trait for HTML-based news sources that fetch content via Chrome DevTools.
pub trait HtmlSource {
    /// Returns the unique identifier for this source (e.g., "smol_ai", "anthropic_blog").
    fn id(&self) -> &str;

    /// Fetches the HTML content for this source using Chrome DevTools.
    /// Returns the full HTML string suitable for parsing.
    async fn fetch_html(&self, chrome: &ChromeClient) -> Result<String>;

    /// Checks if the fetched HTML contains fresh content compared to the last run.
    /// Uses per-source state to determine if items are new.
    fn is_fresh(&self, html: &str, state: &State) -> bool;

    /// Extracts structured news items from the HTML.
    /// Returns a vector of NewsItem objects suitable for LLM summarization.
    fn extract_items(&self, html: &str) -> Result<Vec<NewsItem>>;
}

