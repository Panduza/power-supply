use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Current value payload for power supply measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentPayload {
    /// PZA identifier
    /// On the command, the client generates this ID
    /// On the response, the server echoes this ID
    pub pza_id: String,
    /// Current value in Amperes as string for stability
    pub current: String,
}

impl CurrentPayload {
    /// Create a new CurrentPayload from a current value
    pub fn from_string(current: String) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            current,
        }
    }

    /// Create a new CurrentPayload from f32 value with specified decimal places
    pub fn from_f32(value: f32, decimals: usize) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            current: format!("{:.1$}", value, decimals),
        }
    }

    /// Create a new CurrentPayload as a response to a command with the given pza_id
    pub fn from_current_as_response(current: String, pza_id: String) -> Self {
        Self { pza_id, current }
    }

    /// Serialize the CurrentPayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }

    /// Deserialize a CurrentPayload from JSON bytes
    pub fn from_json_bytes(bytes: Bytes) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(&bytes)?)
    }
}
