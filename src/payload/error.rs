use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Error payload for communicating error messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    /// PZA identifier
    /// On the command, the client generates this ID
    /// On the response, the server echoes this ID
    pub pza_id: String,
    /// Error message description
    pub message: String,
}

impl ErrorPayload {
    /// Create a new ErrorPayload
    pub fn new(message: String) -> Self {
        Self {
            pza_id: super::generate_pza_id(),
            message,
        }
    }

    /// Serialize the ErrorPayload to JSON bytes
    pub fn to_json_bytes(&self) -> anyhow::Result<Bytes> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }
}
