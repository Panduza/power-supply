use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Power state of a power supply
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PowerState {
    /// Power supply is turned on
    #[serde(rename = "ON")]
    On,
    /// Power supply is turned off
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
    /// Create a new PowerStatePayload from a state
    pub fn from_state(state: PowerState) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            state,
        }
    }

    /// Create a new PowerStatePayload as a response to a command with the given pza_id
    pub fn from_state_as_response(state: PowerState, pza_id: String) -> Self {
        Self { pza_id, state }
    }

    /// Serialize the PowerStatePayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }

    /// Deserialize a PowerStatePayload from JSON bytes
    pub fn from_json_bytes(bytes: Bytes) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(&bytes)?)
    }
}
