use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

use ka3005p::Command;
use ka3005p::Switch;
use tracing::info;

use crate::config::PowerSupplyConfig;
use crate::drivers::DriverError;
use crate::drivers::PowerSupplyDriver;

use ka3005p::Ka3005p;

/// A power supply emulator for testing and development purposes
pub struct Kd3005pDriver {
    driver: Arc<Mutex<Ka3005p>>,
}

impl Kd3005pDriver {
    /// Create a new power supply emulator instance
    pub fn new(_config: PowerSupplyConfig) -> Self {
        let dev = ka3005p::find_serial_port().unwrap();
        Self {
            driver: Arc::new(Mutex::new(dev)),
        }
    }

    //--------------------------------------------------------------------------

    /// Get the manifest information for this driver
    pub fn manifest() -> serde_json::Value {
        serde_json::json!({
            "model": "kd3005p",
            "description": "A simple power supply from Korad",
        })
    }
}

#[async_trait]
impl PowerSupplyDriver for Kd3005pDriver {
    /// Get the output enabled state
    async fn output_enabled(&mut self) -> Result<bool, DriverError> {
        let state_oe = self.driver.lock().await.read_output_enable().unwrap();
        info!("Kd3005p Driver: output_enabled = {}", state_oe);
        Ok(state_oe)
    }

    //--------------------------------------------------------------------------

    /// Enable the output
    async fn enable_output(&mut self) -> Result<(), DriverError> {
        info!("Kd3005p Driver: enable_output");
        self.driver
            .lock()
            .await
            .execute(Command::Power(Switch::On))
            .unwrap();
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Disable the output
    async fn disable_output(&mut self) -> Result<(), DriverError> {
        info!("Kd3005p Driver: disable_output");
        self.driver
            .lock()
            .await
            .execute(Command::Power(Switch::Off))
            .unwrap();
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Get the voltage
    async fn get_voltage(&mut self) -> Result<String, DriverError> {
        let voltage = self.driver.lock().await.read_set_voltage().unwrap();
        info!("Kd3005p Driver: get_voltage = {}", voltage);
        Ok(voltage.to_string())
    }

    //--------------------------------------------------------------------------

    /// Set the voltage
    async fn set_voltage(&mut self, voltage: String) -> Result<(), DriverError> {
        info!("Kd3005p Driver: set_voltage = {}", voltage);
        self.driver
            .lock()
            .await
            .execute(Command::Voltage(voltage.parse().unwrap()))
            .unwrap();
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Get the current
    async fn get_current(&mut self) -> Result<String, DriverError> {
        let current = self.driver.lock().await.read_set_current().unwrap();
        info!("Kd3005p Driver: get_current = {}", current);
        Ok(current.to_string())
    }

    //--------------------------------------------------------------------------

    /// Set the current
    async fn set_current(&mut self, current: String) -> Result<(), DriverError> {
        info!("Kd3005p Driver: set_current = {}", current);
        self.driver
            .lock()
            .await
            .execute(Command::Current(current.parse().unwrap()))
            .unwrap();
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Measure the voltage
    async fn measure_voltage(&mut self) -> Result<String, DriverError> {
        info!("Kd3005p Driver: measure_voltage");
        Ok("0".into())
    }

    //--------------------------------------------------------------------------

    /// Measure the current
    async fn measure_current(&mut self) -> Result<String, DriverError> {
        info!("Kd3005p Driver: measure_current");
        Ok("0".into())
    }
}
