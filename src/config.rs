pub mod schema;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use schema::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub scheduler: Option<SchedulerConfig>,
    pub vpn: Option<VpnConfig>,
    pub notifications: NotificationsConfig,
    pub storage: StorageConfig,
    pub sources: Vec<SourceConfig>,
    pub secrets: Option<SecretsConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub run_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    pub process_name: String,
    pub launch_command: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsConfig {
    pub email: Option<EmailConfig>,
    pub shortcut: Option<ShortcutConfig>,
    #[serde(default)]
    pub error_section: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub enabled: bool,
    pub to: String,
    pub smtp_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub enabled: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub source_type: String,
    pub active: bool,
    // Additional fields are stored in extra via serde_json::Value
    // This allows flexible source-specific configuration
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsConfig {
    pub keychain_service_prefix: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::default_path()?;
        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {}", config_path.display()))?;
        
        let config: Config = serde_yaml::from_str(&content)
            .context("Failed to parse config YAML")?;
        
        Ok(config)
    }

    pub fn default_path() -> Result<PathBuf> {
        let home = std::env::var("HOME")?;
        Ok(PathBuf::from(format!("{}/.config/news_agent/config.yaml", home)))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::default_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_yaml::to_string(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}

