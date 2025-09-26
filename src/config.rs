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
struct PowerSupplyConfig {
    /// Unique identifier for the power supply
    model: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ApplicationConfig {
    /// MQTT broker configuration
    broker: MqttBrokerConfig,

    /// Power supply configurations, keyed by their unique identifiers
    devices: Option<HashMap<String, PowerSupplyConfig>>,
}

