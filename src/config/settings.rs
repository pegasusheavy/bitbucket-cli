use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const APP_NAME: &str = "bitbucket";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub display: DisplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub default_workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub workspace: Option<String>,
    pub repository: Option<String>,
    pub branch: Option<String>,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            workspace: None,
            repository: None,
            branch: Some("main".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub color: bool,
    pub pager: bool,
    pub date_format: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            color: true,
            pager: true,
            date_format: "%Y-%m-%d %H:%M".to_string(),
        }
    }
}

impl Config {
    /// Get the configuration directory path
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join(APP_NAME);
        Ok(config_dir)
    }

    /// Get the configuration file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join(CONFIG_FILE))
    }

    /// Load configuration from file, or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;
        }

        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, contents)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }

    /// Set the authenticated username
    pub fn set_username(&mut self, username: &str) {
        self.auth.username = Some(username.to_string());
    }

    /// Get the authenticated username
    pub fn username(&self) -> Option<&str> {
        self.auth.username.as_deref()
    }

    /// Set the default workspace
    pub fn set_default_workspace(&mut self, workspace: &str) {
        self.defaults.workspace = Some(workspace.to_string());
    }

    /// Get the default workspace
    pub fn default_workspace(&self) -> Option<&str> {
        self.defaults.workspace.as_deref()
    }

    /// Clear authentication settings (for logout)
    pub fn clear_auth(&mut self) {
        self.auth.username = None;
        self.auth.default_workspace = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.auth.username.is_none());
        assert!(config.display.color);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(config.display.color, deserialized.display.color);
    }
}
