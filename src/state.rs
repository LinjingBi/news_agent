pub mod models;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub use models::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub sources: HashMap<String, SourceState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceState {
    pub active: bool,
    pub last_checked_at: String,
    pub last_published_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl State {
    pub fn load() -> Result<Self> {
        let state_path = Self::default_path()?;
        
        if !state_path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(&state_path)
            .with_context(|| format!("Failed to read state from {}", state_path.display()))?;
        
        let state: State = serde_json::from_str(&content)
            .context("Failed to parse state JSON")?;
        
        Ok(state)
    }

    pub fn save(&self) -> Result<()> {
        let state_path = Self::default_path()?;
        if let Some(parent) = state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&state_path, content)?;
        Ok(())
    }

    pub fn default_path() -> Result<PathBuf> {
        let home = std::env::var("HOME")?;
        Ok(PathBuf::from(format!("{}/.local/share/news_agent/state.json", home)))
    }

    pub fn update_source(&mut self, source_id: &str, html: &str) -> Result<()> {
        use chrono::Local;
        
        let source_state = self.sources.entry(source_id.to_string()).or_insert_with(|| SourceState {
            active: true,
            last_checked_at: Local::now().to_rfc3339(),
            last_published_at: None,
            extra: HashMap::new(),
        });
        
        source_state.last_checked_at = Local::now().to_rfc3339();
        // TODO: Extract last_published_at from HTML if possible
        
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }
}

