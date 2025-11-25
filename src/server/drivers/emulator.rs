use crate::server::config::PowerSupplyConfig;
use crate::server::drivers::PowerSupplyDriver;
use async_trait::async_trait;
use tracing::info;

/// A power supply emulator for testing and development purposes
pub struct PowerSupplyEmulator {
    state_oe: bool,
    #[allow(dead_code)]
    voltage: String,
    #[allow(dead_code)]
    current: String,

    security_min_voltage: Option<f32>,
    security_max_voltage: Option<f32>,
    security_min_current: Option<f32>,
    security_max_current: Option<f32>,
}

impl PowerSupplyEmulator {
    /// Create a new power supply emulator instance
    pub fn new(config: PowerSupplyConfig) -> Self {
        Self {
            state_oe: false,
            voltage: "5.3".into(),
            current: "1.2".into(),
            security_min_voltage: config.security_min_voltage,
            security_max_voltage: config.security_max_voltage,
            security_min_current: config.security_min_current,
            security_max_current: config.security_max_current,
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
    /// Initialize the driver
    async fn initialize(&mut self) -> anyhow::Result<()> {
        info!("Emulator Driver: initialize");
        Ok(())
    }
    /// Shutdown the driver
    async fn shutdown(&mut self) -> anyhow::Result<()> {
        info!("Emulator Driver: shutdown");
        Ok(())
    }

    /// Get the output enabled state
    async fn output_enabled(&mut self) -> anyhow::Result<bool> {
        info!("Emulator Driver: output_enabled = {}", self.state_oe);
        Ok(self.state_oe)
    }

    //--------------------------------------------------------------------------

    /// Enable the output
    async fn enable_output(&mut self) -> anyhow::Result<()> {
        info!("Emulator Driver: enable_output");
        self.state_oe = true;
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Disable the output
    async fn disable_output(&mut self) -> anyhow::Result<()> {
        info!("Emulator Driver: disable_output");
        self.state_oe = false;
        Ok(())
    }

    //--------------------------------------------------------------------------

    /// Get the voltage
    async fn get_voltage(&mut self) -> anyhow::Result<String> {
        info!("Emulator Driver: get_voltage = {}", self.voltage);
        Ok(self.voltage.clone())
    }

    //--------------------------------------------------------------------------

    /// Set the voltage
    async fn set_voltage(&mut self, voltage: String) -> anyhow::Result<()> {
        info!("Emulator Driver: set_voltage = {}", voltage);

        // Parse voltage value
        let voltage_value: f32 = voltage
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid voltage format: {}", voltage))?;

        // Check security minimum voltage
        if let Some(min_voltage) = self.security_min_voltage {
            if voltage_value < min_voltage {
                return Err(anyhow::anyhow!(
                    "Voltage {} is below minimum security limit of {}",
                    voltage_value,
                    min_voltage
                ));
            }
        }

        // Check security maximum voltage
        if let Some(max_voltage) = self.security_max_voltage {
            if voltage_value > max_voltage {
                return Err(anyhow::anyhow!(
                    "Voltage {} exceeds maximum security limit of {}",
                    voltage_value,
                    max_voltage
                ));
            }
        }

        self.voltage = voltage;
        Ok(())
    }

    /// Get the security minimum voltage
    fn security_min_voltage(&self) -> Option<f32> {
        self.security_min_voltage
    }
    fn security_max_voltage(&self) -> Option<f32> {
        self.security_max_voltage
    }

    //--------------------------------------------------------------------------

    /// Get the current
    async fn get_current(&mut self) -> anyhow::Result<String> {
        info!("Emulator Driver: get_current = {}", self.current);
        Ok(self.current.clone())
    }

    //--------------------------------------------------------------------------

    /// Set the current
    async fn set_current(&mut self, current: String) -> anyhow::Result<()> {
        info!("Emulator Driver: set_current = {}", current);

        // Parse current value
        let current_value: f32 = current
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid current format: {}", current))?;

        // Check security minimum current
        if let Some(min_current) = self.security_min_current {
            if current_value < min_current {
                return Err(anyhow::anyhow!(
                    "Current {} is below minimum security limit of {}",
                    current_value,
                    min_current
                ));
            }
        }

        // Check security maximum current
        if let Some(max_current) = self.security_max_current {
            if current_value > max_current {
                return Err(anyhow::anyhow!(
                    "Current {} exceeds maximum security limit of {}",
                    current_value,
                    max_current
                ));
            }
        }

        self.current = current;
        Ok(())
    }

    /// Get the security minimum current
    fn security_min_current(&self) -> Option<f32> {
        self.security_min_current
    }
    fn security_max_current(&self) -> Option<f32> {
        self.security_max_current
    }

    //--------------------------------------------------------------------------

    /// Measure the voltage
    async fn measure_voltage(&mut self) -> anyhow::Result<String> {
        info!("Emulator Driver: measure_voltage");
        Ok("0".into())
    }

    //--------------------------------------------------------------------------

    /// Measure the current
    async fn measure_current(&mut self) -> anyhow::Result<String> {
        info!("Emulator Driver: measure_current");
        Ok("0".into())
    }
}
