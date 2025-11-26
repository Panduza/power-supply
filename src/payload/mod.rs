use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Generate a random 5-character PZA ID
pub fn generate_pza_id() -> String {
    pza_toolkit::rand::generate_random_string(5)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub pza_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerState {
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerStatePayload {
    /// PZA identifier
    /// On the command, the client generates this ID
    /// On the response, the server echoes this ID
    pub pza_id: String,
    pub state: PowerState,
}

impl PowerStatePayload {
    /// Create a new PowerStatePayload
    pub fn new(state: PowerState) -> Self {
        Self {
            pza_id: generate_pza_id(),
            state,
        }
    }

    /// Serialize the PowerStatePayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }
}
