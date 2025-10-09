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
    /// Configuration for the power supply
    config: PowerSupplyConfig,

    /// The underlying driver instance
    driver: Option<Arc<Mutex<Ka3005p>>>,
}

impl Kd3005pDriver {
    /// Create a new power supply emulator instance
    pub fn new(config: PowerSupplyConfig) -> Self {
        Self {
            config,
            driver: None,
        }
    }

    //--------------------------------------------------------------------------

    /// Get the manifest information for this driver
    pub fn manifest() -> serde_json::Value {
        serde_json::json!({
            "model": "kd3005p",
            "description": "A simple power supply from Korad",
            "security_min_voltage": Some(0.0_f32),
            "security_max_voltage": Some(30.0_f32),
            "security_min_current": Some(0.0_f32),
            "security_max_current": Some(3.0_f32),
        })
    }
}

#[async_trait]
impl PowerSupplyDriver for Kd3005pDriver {
    /// Initialize the driver
    async fn initialize(&mut self) -> Result<(), DriverError> {
        info!("Kd3005p Driver: initialize");
        let mut dev = ka3005p::find_serial_port().unwrap();

        dev.execute(Command::Ovp(Switch::On)).unwrap();
        dev.execute(Command::Ocp(Switch::On)).unwrap();

        self.driver = Some(Arc::new(Mutex::new(dev)));

        Ok(())
    }
    /// Shutdown the driver
    async fn shutdown(&mut self) -> Result<(), DriverError> {
        info!("Emulator Driver: shutdown");
        Ok(())
    }

    /// Get the output enabled state
    async fn output_enabled(&mut self) -> Result<bool, DriverError> {
        let state_oe = self
            .driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .read_output_enable()
            .unwrap();
        info!("Kd3005p Driver: output_enabled = {}", state_oe);
        Ok(state_oe)
    }

    //--------------------------------------------------------------------------

    /// Enable the output
    async fn enable_output(&mut self) -> Result<(), DriverError> {
        info!("Kd3005p Driver: enable_output");
        self.driver
            .as_ref()
            .expect("Driver not initialized")
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
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .execute(Command::Power(Switch::Off))
            .unwrap();

        // Save the settings to the device's memory
        // Important to avoid bad config after power cycle
        self.driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .execute(Command::Save(1))
            .map_err(|e| DriverError::Generic(format!("Failed to save: {:?}", e)))?;

        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Get the voltage
    async fn get_voltage(&mut self) -> Result<String, DriverError> {
        let voltage = self
            .driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .read_set_voltage()
            .unwrap();
        info!("Kd3005p Driver: get_voltage = {}", voltage);
        Ok(voltage.to_string())
    }

    //--------------------------------------------------------------------------

    /// Set the voltage
    async fn set_voltage(&mut self, voltage: String) -> Result<(), DriverError> {
        info!("Kd3005p Driver: set_voltage = {}", voltage);

        // Parse voltage value
        let voltage_value: f32 = voltage
            .parse()
            .map_err(|_| DriverError::Generic(format!("Invalid voltage format: {}", voltage)))?;

        // Check security minimum voltage
        if let Some(min_voltage) = self.config.security_min_voltage {
            if voltage_value < min_voltage {
                return Err(DriverError::VoltageSecurityLimitExceeded(format!(
                    "Voltage {} is below minimum security limit of {}",
                    voltage_value, min_voltage
                )));
            }
        }

        // Check security maximum voltage
        if let Some(max_voltage) = self.config.security_max_voltage {
            if voltage_value > max_voltage {
                return Err(DriverError::VoltageSecurityLimitExceeded(format!(
                    "Voltage {} exceeds maximum security limit of {}",
                    voltage_value, max_voltage
                )));
            }
        }

        self.driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .execute(Command::Voltage(voltage_value))
            .map_err(|e| DriverError::Generic(format!("Failed to set voltage: {:?}", e)))?;

        // Save the settings to the device's memory
        // Important to avoid bad config after power cycle
        self.driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .execute(Command::Save(1))
            .map_err(|e| DriverError::Generic(format!("Failed to save: {:?}", e)))?;

        Ok(())
    }

    /// Get the security minimum voltage
    fn security_min_voltage(&self) -> Option<f32> {
        self.config.security_min_voltage
    }

    /// Get the security maximum voltage
    fn security_max_voltage(&self) -> Option<f32> {
        self.config.security_max_voltage
    }

    //--------------------------------------------------------------------------

    /// Get the current
    async fn get_current(&mut self) -> Result<String, DriverError> {
        let current = self
            .driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .read_set_current()
            .unwrap();
        info!("Kd3005p Driver: get_current = {}", current);
        Ok(current.to_string())
    }

    //--------------------------------------------------------------------------

    /// Set the current
    async fn set_current(&mut self, current: String) -> Result<(), DriverError> {
        info!("Kd3005p Driver: set_current = {}", current);

        // Parse current value
        let current_value: f32 = current
            .parse()
            .map_err(|_| DriverError::Generic(format!("Invalid current format: {}", current)))?;

        // Check security minimum current
        if let Some(min_current) = self.config.security_min_current {
            if current_value < min_current {
                return Err(DriverError::CurrentSecurityLimitExceeded(format!(
                    "Current {} is below minimum security limit of {}",
                    current_value, min_current
                )));
            }
        }

        // Check security maximum current
        if let Some(max_current) = self.config.security_max_current {
            if current_value > max_current {
                return Err(DriverError::CurrentSecurityLimitExceeded(format!(
                    "Current {} exceeds maximum security limit of {}",
                    current_value, max_current
                )));
            }
        }

        self.driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .execute(Command::Current(current_value))
            .map_err(|e| DriverError::Generic(format!("Failed to set current: {:?}", e)))?;

        // Save the settings to the device's memory
        // Important to avoid bad config after power cycle
        self.driver
            .as_ref()
            .expect("Driver not initialized")
            .lock()
            .await
            .execute(Command::Save(1))
            .map_err(|e| DriverError::Generic(format!("Failed to save: {:?}", e)))?;

        Ok(())
    }

    /// Get the security minimum current
    fn security_min_current(&self) -> Option<f32> {
        self.config.security_min_current
    }
    /// Get the security maximum current
    fn security_max_current(&self) -> Option<f32> {
        self.config.security_max_current
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
