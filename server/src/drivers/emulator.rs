use async_trait::async_trait;

use crate::config::PowerSupplyConfig;
use crate::drivers::DriverError;
use crate::drivers::PowerSupplyDriver;

/// A power supply emulator for testing and development purposes
pub struct PowerSupplyEmulator {
    state_oe: bool,
    #[allow(dead_code)]
    voltage: String,
    #[allow(dead_code)]
    current: String,
}

impl PowerSupplyEmulator {
    /// Create a new power supply emulator instance
    pub fn new(_config: PowerSupplyConfig) -> Self {
        Self {
            state_oe: false,
            voltage: "0".into(),
            current: "0".into(),
        }
    }

    //--------------------------------------------------------------------------

    /// Get the manifest information for this driver
    pub fn manifest() -> serde_json::Value {
        serde_json::json!({
            "model": "emulator",
            "description": "A simple power supply emulator for testing and development purposes.",
        })
    }
}

#[async_trait]
impl PowerSupplyDriver for PowerSupplyEmulator {
    /// Get the output enabled state
    async fn output_enabled(&mut self) -> Result<bool, DriverError> {
        println!("Emulator Driver: output_enabled = {}", self.state_oe);
        Ok(self.state_oe)
    }

    //--------------------------------------------------------------------------

    /// Enable the output
    async fn enable_output(&mut self) -> Result<(), DriverError> {
        println!("Emulator Driver: enable_output");
        self.state_oe = true;
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Disable the output
    async fn disable_output(&mut self) -> Result<(), DriverError> {
        println!("Emulator Driver: disable_output");
        self.state_oe = false;
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Get the voltage
    async fn get_voltage(&mut self) -> Result<String, DriverError> {
        println!("Emulator Driver: get_voltage = {}", self.voltage);
        Ok(self.voltage.clone())
    }

    //--------------------------------------------------------------------------

    /// Set the voltage
    async fn set_voltage(&mut self, voltage: String) -> Result<(), DriverError> {
        println!("Emulator Driver: set_voltage = {}", voltage);
        self.voltage = voltage;
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Get the current
    async fn get_current(&mut self) -> Result<String, DriverError> {
        println!("Emulator Driver: get_current = {}", self.current);
        Ok(self.current.clone())
    }

    //--------------------------------------------------------------------------

    /// Set the current
    async fn set_current(&mut self, current: String) -> Result<(), DriverError> {
        println!("Emulator Driver: set_current = {}", current);
        self.current = current;
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Measure the voltage
    async fn measure_voltage(&mut self) -> Result<String, DriverError> {
        println!("Emulator Driver: measure_voltage");
        Ok("0".into())
    }

    //--------------------------------------------------------------------------

    /// Measure the current
    async fn measure_current(&mut self) -> Result<String, DriverError> {
        println!("Emulator Driver: measure_current");
        Ok("0".into())
    }
}
