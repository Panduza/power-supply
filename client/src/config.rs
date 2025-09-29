use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MqttBrokerConfig {
    /// Bind address of the MQTT broker
    pub host: String,
    /// Port of the MQTT broker
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuiConfig {
    /// Enable or disable the GUI
    pub enable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Enable or disable the MCP server
    pub enable: bool,
    /// Bind address of the MCP server
    pub host: String,
    /// Port of the MCP server
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerSupplyConfig {
    /// Unique identifier for the power supply
    pub model: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            broker: MqttBrokerConfig {
                host: "127.0.0.1".to_string(),
                port: 1883,
            },
        }
    }
}

impl GlobalConfig {
    /// Load the global configuration from the configuration file
    ///
    /// - The configuration file is expected to be in JSON5 format
    /// - Path to the configuration file is determined by `path::global_config_file()`
    /// - If the file does not exist or cannot be read, generate a default configuration file
    /// - When generating a default configuration file
    ///     - ensure the user root directory exists first
    ///     - write in json5 the content in pretty format
    ///
    /// - If path cannot be read, panic and stop application
    ///
    pub fn from_user_file() -> Self {
        let config_path = crate::path::global_config_file()
            .expect("Could not determine configuration file path. Application cannot continue.");

        info!("Loading configuration from: {}", config_path.display());

        match std::fs::read_to_string(&config_path) {
            Ok(content) => match serde_json5::from_str::<GlobalConfig>(&content) {
                Ok(config) => config,
                Err(err) => {
                    error!(
                        "Failed to parse config file: {}. Generating default configuration.",
                        err
                    );
                    Self::generate_default_config(&config_path)
                }
            },
            Err(_) => {
                // File does not exist or cannot be read, generate default config
                Self::generate_default_config(&config_path)
            }
        }
    }

    /// Generate a default configuration file
    ///
    /// - Ensures the user root directory exists first
    /// - Writes the default configuration in JSON5 format
    /// - Returns the default configuration
    ///
    fn generate_default_config(config_path: &Path) -> Self {
        // Ensure the user root directory exists
        if let Err(err) = crate::path::ensure_user_root_dir_exists() {
            panic!("Failed to create user root directory: {}", err);
        }

        // Create default configuration
        let default_config = Self::default();

        // Serialize to JSON format with pretty printing
        let config_content = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default configuration");

        // Write the configuration file
        if let Err(err) = std::fs::write(config_path, config_content) {
            error!("Failed to write default configuration file: {}", err);
        } else {
            info!(
                "Generated default configuration file at: {}",
                config_path.display()
            );
        }

        default_config
    }
}
