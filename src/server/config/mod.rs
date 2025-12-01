mod mcp;
mod path;
mod power_supply;
mod tui;

pub use power_supply::PowerSupplyConfig;
pub use tui::TuiConfig;

use crate::server::config::mcp::McpConfig;
use pza_toolkit::config::MqttBrokerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// TUI configuration
    pub tui: tui::TuiConfig,

    /// MCP server configuration
    pub mcp: McpConfig,

    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,

    /// Power supply configurations, keyed by their unique identifiers
    pub runners: Option<HashMap<String, PowerSupplyConfig>>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        // Create a default power supply configuration for an emulator device
        let mut runners = HashMap::new();
        runners.insert(
            "emulator".to_string(),
            PowerSupplyConfig {
                model: "emulator".to_string(),
                description: None,
                security_min_voltage: Some(0.0),
                security_max_voltage: Some(30.0),
                security_min_current: Some(0.0),
                security_max_current: Some(5.0),
            },
        );

        // Return the default global configuration
        Self {
            tui: tui::TuiConfig {
                power_toggle_key: Some("p".to_string()),
            },
            mcp: McpConfig {
                enable: false,
                host: "127.0.0.1".to_string(),
                port: 50051,
            },
            broker: MqttBrokerConfig::default(),
            runners: Some(runners),
        }
    }
}

impl ServerConfig {
    /// Load the global configuration from the configuration file
    ///
    pub fn from_user_file() -> anyhow::Result<Self> {
        let config_path = path::server_config_file()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine server configuration file path"))?;

        pza_toolkit::config::read_config::<ServerConfig>(&config_path)
    }

    /// List MCP server URLs from the configuration
    ///
    pub fn list_mcp_servers_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();

        if let Some(runners) = &self.runners {
            for (name, config) in runners {
                let url = format!(
                    "http://{}:{}/{}/{}",
                    self.mcp.host,
                    self.mcp.port,
                    pza_power_supply_client::SERVER_TYPE_NAME,
                    name
                );
                urls.push(url);
            }
        }

        urls
    }

    /// List MCP server URLs as a JSON string
    pub fn list_mcp_servers_urls_as_json_string(&self) -> String {
        let urls = self.list_mcp_servers_urls();
        serde_json::to_string_pretty(&urls).unwrap_or_else(|_| "[]".to_string())
    }

    /// Apply service overrides from CLI arguments
    ///
    pub fn apply_overrides(&mut self, overrides: &crate::server::cli::ServicesOverrides) {
        // Apply overrides to the configuration as needed
        if overrides.no_mcp {
            self.mcp.enable = false;
        }
        if overrides.no_runners {
            self.runners = None;
        }
    }
}
