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
    /// Optional panic message if status is Panicking
    pub panic_message: Option<String>,
}

impl StatusPayload {
    /// Create a new StatusPayload
    pub fn from_status(status: Status) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            status,
            panic_message: None,
        }
    }

    /// Set the panic message for Panicking status
    pub fn with_panic_message(mut self, message: String) -> Self {
        self.panic_message = Some(message);
        self
    }

    /// Serialize the StatusPayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }

    /// Deserialize a StatusPayload from JSON bytes
    pub fn from_json_bytes(bytes: Bytes) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(&bytes)?)
    }
}
