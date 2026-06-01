//! Layered configuration management.
//!
//! Configuration is loaded from multiple sources in priority order
//! (lowest to highest):
//! 1. Code defaults
//! 2. XDG config file (`TOML` format)
//! 3. Environment variables (`CLI_` prefix)

use miette::{IntoDiagnostic, Result, WrapErr};
use serde::{Deserialize, Serialize};

/// CLI application configuration loaded from layered sources.
///
/// Fields marked `#[serde(default)]` fall back to [`Default`] when
/// the config file omits them.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CliConfig {
    /// Application name (used to construct the core [`Config`]).
    pub name: String,
    /// Optional description for the current run.
    pub description: Option<String>,
    /// Enable verbose / debug output.
    pub verbose: bool,
    /// Tracing-filter log level (e.g. `info`, `debug`, `trace`).
    pub log_level: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            name: default_name(),
            description: None,
            verbose: false,
            log_level: default_log_level(),
        }
    }
}

impl CliConfig {
    /// Load configuration from all layers.
    ///
    /// Layers are applied in order: code defaults, then `XDG` config
    /// file (`{{ project-name }}/config.toml` under the user's config
    /// directory), and finally environment variables with the `CLI_`
    /// prefix.
    ///
    /// # Errors
    ///
    /// Returns an error if the config file exists but cannot be read
    /// or contains invalid `TOML`.
    pub fn load() -> Result<Self> {
        let mut config = Self::default();

        // Layer 1: XDG config file
        if let Some(config_dir) = dirs::config_dir() {
            let config_file = config_dir
                .join("{{ project-name }}")
                .join("config.toml");

            if config_file.exists() {
                let content = std::fs::read_to_string(&config_file)
                    .into_diagnostic()
                    .wrap_err_with(|| {
                        format!("Failed to read config file: {}", config_file.display())
                    })?;

                if !content.trim().is_empty() {
                    config = toml::from_str(&content)
                        .into_diagnostic()
                        .wrap_err("Failed to parse config file")?;
                }
            }
        }

        // Layer 2: Environment variables (CLI_ prefix)
        if let Ok(val) = std::env::var("CLI_NAME") {
            config.name = val;
        }
        if let Ok(val) = std::env::var("CLI_DESCRIPTION") {
            config.description = Some(val);
        }
        if std::env::var("CLI_VERBOSE").is_ok() {
            config.verbose = true;
        }
        if let Ok(val) = std::env::var("CLI_LOG_LEVEL") {
            config.log_level = val;
        }

        Ok(config)
    }
}

fn default_name() -> String {
    "{{ project-name }}".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}
