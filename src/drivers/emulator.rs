use crate::{config::PowerSupplyConfig, drivers::PowerSupplyDriver};
use async_trait::async_trait;

pub struct PowerSupplyEmulator {}

impl PowerSupplyEmulator {
    pub fn new(config: PowerSupplyConfig) -> Self {
        Self {
            // Initialize with config if needed
        }
    }
}

#[async_trait]
impl PowerSupplyDriver for PowerSupplyEmulator {
    async fn output_enabled(&mut self) -> Result<bool, crate::drivers::DriverError> {
        // Simulate checking if output is enabled
        Ok(true)
    }

    async fn enable_output(&mut self) -> Result<(), crate::drivers::DriverError> {
        // Simulate enabling output
        Ok(())
    }

    async fn disable_output(&mut self) -> Result<(), crate::drivers::DriverError> {
        // Simulate disabling output
        Ok(())
    }
}
