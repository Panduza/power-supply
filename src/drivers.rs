pub mod emulator;
pub mod hm310t;
pub mod kd3005p;

use async_trait::async_trait;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone)]
pub enum DriverError {
    #[error("An error occurred: {0}")]
    Generic(String),
    #[error("Security limit exceeded: {0}")]
    VoltageSecurityLimitExceeded(String),
    #[error("Security limit exceeded: {0}")]
    CurrentSecurityLimitExceeded(String),
}

#[async_trait]
pub trait PowerSupplyDriver: Send + Sync {
    // --- Lifecycle management ---

    /// Initialize the driver
    async fn initialize(&mut self) -> Result<(), DriverError>;
    /// Shutdown the driver
    async fn shutdown(&mut self) -> Result<(), DriverError>;

    // --- Output control ---

    /// Check if output is enabled
    async fn output_enabled(&mut self) -> Result<bool, DriverError>;
    /// Enable or disable output
    async fn enable_output(&mut self) -> Result<(), DriverError>;
    /// Disable output
    async fn disable_output(&mut self) -> Result<(), DriverError>;

    // --- Voltage and current control ---

    /// Get the voltage setting
    async fn get_voltage(&mut self) -> Result<String, DriverError>;
    /// Set the voltage setting
    async fn set_voltage(&mut self, voltage: String) -> Result<(), DriverError>;

    // Security limits
    fn security_min_voltage(&self) -> Option<f32>;
    fn security_max_voltage(&self) -> Option<f32>;

    /// Get the current setting
    async fn get_current(&mut self) -> Result<String, DriverError>;
    /// Set the current setting
    async fn set_current(&mut self, current: String) -> Result<(), DriverError>;

    // Security limits
    fn security_min_current(&self) -> Option<f32>;
    fn security_max_current(&self) -> Option<f32>;

    // --- Measurements ---

    /// Measure the output voltage
    async fn measure_voltage(&mut self) -> Result<String, DriverError>;
    /// Measure the output current
    async fn measure_current(&mut self) -> Result<String, DriverError>;
}
