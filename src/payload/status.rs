use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Status of a power supply instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    /// The instance is starting up
    Initializing,
    /// The instance is operational
    Running,
    /// The instance has encountered a critical error
    Panicking,
}

/// Status payload for communicating power supply status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPayload {
    /// PZA identifier
    /// On the command, the client generates this ID
    /// On the response, the server echoes this ID
    pub pza_id: String,
    /// Current status of the power supply instance
    pub status: Status,
}

impl StatusPayload {
    /// Create a new StatusPayload
    pub fn new(status: Status) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            status,
        }
    }

    /// Serialize the StatusPayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }
}
