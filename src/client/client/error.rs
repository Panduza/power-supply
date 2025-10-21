use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone)]
pub enum ClientError {
    #[error("An error occurred: {0}")]
    Generic(String),
    #[error("An error occurred on mqtt communication: {0}")]
    MqttError(String),
}
