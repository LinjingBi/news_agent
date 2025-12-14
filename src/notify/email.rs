use crate::config::Config;
use crate::notify::{Notifier, Report};
use anyhow::Result;

/// Email notification backend implementation.
pub struct EmailNotifier {
    // TODO: Add email configuration fields
}

impl EmailNotifier {
    pub fn new(config: &Config) -> Result<Self> {
        // TODO: Initialize email notifier from config
        Ok(Self {})
    }
}

impl Notifier for EmailNotifier {
    async fn send(&self, report: &Report) -> Result<()> {
        // TODO: Implement email sending via SMTP
        println!("Email notification (placeholder): {}", report.date);
        Ok(())
    }
}

