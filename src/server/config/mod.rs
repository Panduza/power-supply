mod mcp;
mod power_supply;
mod tui;

use crate::server::config::mcp::McpConfig;
use crate::server::config::power_supply::PowerSupplyConfig;
use crate::server::config::tui::TuiConfig;
use pza_toolkit::config::MqttBrokerConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerMainConfig {
    /// TUI configuration
    pub tui: TuiConfig,

    /// MCP server configuration
    pub mcp: McpConfig,

    /// MQTT broker configuration
    pub broker: MqttBrokerConfig,

    /// Power supply configurations, keyed by their unique identifiers
    pub devices: Option<HashMap<String, PowerSupplyConfig>>,
}

impl Default for ServerMainConfig {
    fn default() -> Self {
        // Create a default power supply configuration for an emulator device
        let mut devices = HashMap::new();
        devices.insert(
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
            tui: TuiConfig {
                power_toggle_key: Some("p".to_string()),
            },
            mcp: McpConfig {
                enable: false,
                host: "127.0.0.1".to_string(),
                port: 50051,
            },
            broker: MqttBrokerConfig::default(),
            devices: Some(devices),
        }
    }
}

impl ServerMainConfig {
    /// Load the global configuration from the configuration file
    ///
    pub fn from_user_file() -> anyhow::Result<Self> {
        let config_path = crate::path::server_config_file()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine server configuration file path"))?;

        pza_toolkit::config::read_config::<ServerMainConfig>(&config_path)
    }
}
