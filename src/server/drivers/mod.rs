pub mod emulator;
pub mod kd3005p;

use async_trait::async_trait;

#[async_trait]
pub trait PowerSupplyDriver: Send + Sync {
    // --- Lifecycle management ---

    /// Initialize the driver
    async fn initialize(&mut self) -> anyhow::Result<()>;
    /// Shutdown the driver
    async fn shutdown(&mut self) -> anyhow::Result<()>;

    // --- Output control ---

    /// Check if output is enabled
    async fn output_enabled(&mut self) -> anyhow::Result<bool>;
    /// Enable or disable output
    async fn enable_output(&mut self) -> anyhow::Result<()>;
    /// Disable output
    async fn disable_output(&mut self) -> anyhow::Result<()>;

    // --- Voltage and current control ---

    /// Get the voltage setting
    async fn get_voltage(&mut self) -> anyhow::Result<String>;
    /// Set the voltage setting
    async fn set_voltage(&mut self, voltage: String) -> anyhow::Result<()>;

    // Security limits
    fn security_min_voltage(&self) -> Option<f32>;
    fn security_max_voltage(&self) -> Option<f32>;

    /// Get the current setting
    async fn get_current(&mut self) -> anyhow::Result<String>;
    /// Set the current setting
    async fn set_current(&mut self, current: String) -> anyhow::Result<()>;

    // Security limits
    fn security_min_current(&self) -> Option<f32>;
    fn security_max_current(&self) -> Option<f32>;

    // --- Measurements ---

    /// Measure the output voltage
    async fn measure_voltage(&mut self) -> anyhow::Result<String>;
    /// Measure the output current
    async fn measure_current(&mut self) -> anyhow::Result<String>;
}
