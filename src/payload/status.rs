use super::payloads_generated::{Status, StatusArgs};
use crate::payload::payloads_generated::StatusCode;
use bytes::Bytes;
use pza_toolkit::rand::generate_random_string;
use std::fmt::Debug;

/// Builder pattern for creating Status payloads
#[derive(Clone, Debug, PartialEq)]
pub struct StatusBuilder {
    panduza_id: Option<String>,
    code: Option<StatusCode>,
    message: Option<String>,
}

impl Default for StatusBuilder {
    fn default() -> Self {
        Self {
            panduza_id: Some(generate_random_string(5)),
            code: None,
            message: None,
        }
    }
}

impl StatusBuilder {
    /// Set the panduza ID for the status
    pub fn pza_id(mut self, panduza_id: String) -> Self {
        self.panduza_id = Some(panduza_id);
        self
    }

    // ----------------------------------------------------------------------------

    /// Set the status code to Booting
    pub fn with_code_booting(mut self) -> Self {
        self.code = Some(StatusCode::Booting);
        self
    }

    // ----------------------------------------------------------------------------

    /// Set the status code to Running
    pub fn with_code_running(mut self) -> Self {
        self.code = Some(StatusCode::Running);
        self
    }

    // ----------------------------------------------------------------------------

    /// Set the status code to Crashed
    pub fn with_code_crashed(mut self) -> Self {
        self.code = Some(StatusCode::Crashed);
        self
    }

    // ----------------------------------------------------------------------------

    /// Set the status message
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    // ----------------------------------------------------------------------------

    /// Build the status buffer from the configured parameters
    pub fn build(self) -> anyhow::Result<StatusBuffer> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let panduza_id = self
            .panduza_id
            .ok_or_else(|| anyhow::anyhow!("panduza_id not provided"))?;
        let message = self
            .message
            .ok_or_else(|| anyhow::anyhow!("message not provided"))?;
        let code = self
            .code
            .ok_or_else(|| anyhow::anyhow!("code not provided"))?;

        // Create FlatBuffer strings and vectors
        let panduza_id_fb = builder.create_string(&panduza_id);
        let message_fb = builder.create_string(&message);

        let status_args = StatusArgs {
            pza_id: Some(panduza_id_fb),
            code,
            message: Some(message_fb),
        };
        let status = Status::create(&mut builder, &status_args);

        builder.finish(status, None);

        Ok(StatusBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

// ================================================================================

/// A status buffer for handling status payloads in FlatBuffers format
#[derive(Clone, PartialEq)]
pub struct StatusBuffer {
    /// The raw data of the buffer, serialized as bytes.
    raw_data: Bytes,
}

// ================================================================================

impl StatusBuffer {
    /// Create a new status buffer from raw bytes
    pub fn new(raw_data: Bytes) -> Self {
        Self { raw_data }
    }

    // ----------------------------------------------------------------------------

    /// Convert the buffer to a Status FlatBuffer object
    pub fn as_status<'a>(&'a self) -> anyhow::Result<Status<'a>> {
        Ok(flatbuffers::root::<Status>(&self.raw_data)?)
    }

    // ----------------------------------------------------------------------------

    /// Get the raw bytes of the status buffer
    pub fn as_bytes(&self) -> &Bytes {
        &self.raw_data
    }
}
