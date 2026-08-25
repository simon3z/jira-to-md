use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub jira: JiraConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JiraConfig {
    pub url: String,
    pub user: String,
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "Configuration file not found at {:?}. Please create it.",
                config_path
            ));
        }

        let content =
            fs::read_to_string(config_path).context("Failed to read configuration file")?;
        let config: Config =
            toml::from_str(&content).context("Failed to parse configuration file")?;
        Ok(config)
    }

    pub fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", "jira-to-md")
            .ok_or_else(|| anyhow::anyhow!("Could not determine configuration directory"))?;

        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    pub fn create_default_config_dir() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", "jira-to-md")
            .ok_or_else(|| anyhow::anyhow!("Could not determine configuration directory"))?;

        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }
        Ok(config_dir.join("config.toml"))
    }
}
