pub mod emulator;




use async_trait::async_trait;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone)]
pub enum DriverError {
    #[error("An error occurred: {0}")]
    Generic(String),
}

#[async_trait]
pub trait PowerSupplyDriver {

    ///
    async fn output_enabled(&mut self) -> Result<bool, DriverError>;
    async fn enable_output(&mut self) -> Result<(), DriverError>;
    async fn disable_output(&mut self) -> Result<(), DriverError>;
}

