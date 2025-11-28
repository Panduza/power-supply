use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Voltage value payload for power supply measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoltagePayload {
    /// PZA identifier
    /// On the command, the client generates this ID
    /// On the response, the server echoes this ID
    pub pza_id: String,
    /// Voltage value in Volts as string for stability
    pub voltage: String,
}

impl VoltagePayload {
    /// Create a new VoltagePayload
    pub fn new(voltage: String) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            voltage,
        }
    }

    /// Create a new VoltagePayload from f32 value with specified decimal places
    pub fn from_f32(value: f32, decimals: usize) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            voltage: format!("{:.1$}", value, decimals),
        }
    }

    /// Serialize the VoltagePayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }
}
