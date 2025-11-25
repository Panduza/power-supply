use serde::{Deserialize, Serialize};

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
