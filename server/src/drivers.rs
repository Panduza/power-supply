pub mod emulator;
pub mod kd3005p;

use async_trait::async_trait;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone)]
pub enum DriverError {
    #[error("An error occurred: {0}")]
    Generic(String),
}

#[async_trait]
pub trait PowerSupplyDriver: Send + Sync {
    ///
    async fn output_enabled(&mut self) -> Result<bool, DriverError>;
    async fn enable_output(&mut self) -> Result<(), DriverError>;
    async fn disable_output(&mut self) -> Result<(), DriverError>;

    //
    async fn get_voltage(&mut self) -> Result<String, DriverError>;
    async fn set_voltage(&mut self, voltage: String) -> Result<(), DriverError>;

    //
    async fn get_current(&mut self) -> Result<String, DriverError>;
    async fn set_current(&mut self, current: String) -> Result<(), DriverError>;

    //
    async fn measure_voltage(&mut self) -> Result<String, DriverError>;
    async fn measure_current(&mut self) -> Result<String, DriverError>;
}
