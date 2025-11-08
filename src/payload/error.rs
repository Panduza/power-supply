use super::payloads_generated::{Error, ErrorArgs};
use bytes::Bytes;
use std::fmt::Debug;

/// Builder pattern for creating Error payloads
#[derive(Default, Clone, Debug, PartialEq)]
pub struct ErrorBuilder {
    panduza_id: Option<String>,
    message: Option<String>,
}

impl ErrorBuilder {
    /// Set the panduza ID for the error
    pub fn pza_id(mut self, panduza_id: String) -> Self {
        self.panduza_id = Some(panduza_id);
        self
    }

    // ----------------------------------------------------------------------------

    /// Set the error message
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    // ----------------------------------------------------------------------------

    /// Build the error buffer from the configured parameters
    pub fn build(self) -> anyhow::Result<ErrorBuffer> {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let panduza_id = self
            .panduza_id
            .ok_or_else(|| anyhow::anyhow!("panduza_id not provided"))?;
        let message = self
            .message
            .ok_or_else(|| anyhow::anyhow!("message not provided"))?;

        // Create FlatBuffer strings and vectors
        let panduza_id_fb = builder.create_string(&panduza_id);
        let message_fb = builder.create_string(&message);

        let error_args = ErrorArgs {
            pza_id: Some(panduza_id_fb),
            message: Some(message_fb),
        };
        let error = Error::create(&mut builder, &error_args);

        builder.finish(error, None);

        Ok(ErrorBuffer {
            raw_data: Bytes::from(builder.finished_data().to_vec()),
        })
    }
}

// ================================================================================

/// An error buffer for handling error payloads in FlatBuffers format
#[derive(Clone, PartialEq)]
pub struct ErrorBuffer {
    /// The raw data of the buffer, serialized as bytes.
    raw_data: Bytes,
}

// ================================================================================

impl ErrorBuffer {
    /// Create a new error buffer from raw bytes
    pub fn new(raw_data: Bytes) -> Self {
        Self { raw_data }
    }

    // ----------------------------------------------------------------------------

    /// Convert the buffer to an Error FlatBuffer object
    pub fn as_error<'a>(&'a self) -> anyhow::Result<Error<'a>> {
        Ok(flatbuffers::root::<Error>(&self.raw_data)?)
    }

    // ----------------------------------------------------------------------------

    /// Get the raw bytes of the error buffer
    pub fn as_bytes(&self) -> &Bytes {
        &self.raw_data
    }
}
