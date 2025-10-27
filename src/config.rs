use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::Path;
use tracing::{error, info};

use crate::client::config::MqttBrokerConfig;

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

    /// Optional description of the power supply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Security limits for voltage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_min_voltage: Option<f32>,
    /// Security limits for voltage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_max_voltage: Option<f32>,
    /// Security limits for current
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_min_current: Option<f32>,
    /// Security limits for current
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_max_current: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerMainConfig {
    /// GUI configuration
    pub gui: GuiConfig,

    /// MCP server configuration
    pub mcp: McpServerConfig,

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
            gui: GuiConfig { enable: true },
            mcp: McpServerConfig {
                enable: false,
                host: "127.0.0.1".to_string(),
                port: 50051,
            },
            broker: MqttBrokerConfig {
                host: "127.0.0.1".to_string(),
                port: 1883,
            },
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
