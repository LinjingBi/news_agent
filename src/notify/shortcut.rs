use crate::config::Config;
use crate::notify::{Notifier, Report};
use anyhow::Result;

/// macOS Shortcuts notification backend implementation.
pub struct ShortcutNotifier {
    shortcut_name: String,
}

impl ShortcutNotifier {
    pub fn new(config: &Config) -> Result<Self> {
        let shortcut_name = config.notifications.shortcut
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "Daily News Digest".to_string());
        
        Ok(Self { shortcut_name })
    }
}

impl Notifier for ShortcutNotifier {
    async fn send(&self, report: &Report) -> Result<()> {
        // TODO: Implement macOS Shortcuts integration
        println!("Shortcut notification (placeholder): {} - {}", self.shortcut_name, report.date);
        Ok(())
    }
}

