use crate::summarize::models::Report;
use anyhow::Result;

/// Trait for notification delivery backends (email, Slack, shortcuts, etc.).
pub trait Notifier {
    /// Sends the report via this notification channel.
    /// Returns Ok(()) on success, or an error if delivery fails.
    async fn send(&self, report: &Report) -> Result<()>;
}

