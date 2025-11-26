use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    /// The instance is starting up
    Initializing,
    /// The instance is operational
    Running,
    /// The instance has encountered a critical error
    Panicking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPayload {
    pub pza_id: String,
    pub status: Status,
}
