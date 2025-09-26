use std::collections::HashMap;
use serde::{Deserialize, Serialize};



#[derive(Clone, Serialize, Deserialize)]
struct MqttBrokerConfig {
    /// Bind address of the MQTT broker
    host: String,
    /// Port of the MQTT broker
    port: u16,
}


#[derive(Clone, Serialize, Deserialize)]
struct GuiConfig {
    /// Enable or disable the GUI
    enable: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct McpServerConfig {
    /// Enable or disable the MCP server
    enable: bool,
    /// Bind address of the MCP server
    host: String,
    /// Port of the MCP server
    port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
struct PowerSupplyConfig {
    /// Unique identifier for the power supply
    model: String,
}



#[derive(Clone, Serialize, Deserialize)]
struct GlobalConfig {
    /// GUI configuration
    gui: GuiConfig,

    /// MCP server configuration
    mcp: McpServerConfig,

    /// MQTT broker configuration
    broker: MqttBrokerConfig,

    /// Power supply configurations, keyed by their unique identifiers
    devices: Option<HashMap<String, PowerSupplyConfig>>,
}

