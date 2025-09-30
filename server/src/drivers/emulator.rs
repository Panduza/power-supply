use crate::drivers::DriverError;
use crate::{config::PowerSupplyConfig, drivers::PowerSupplyDriver};
use async_trait::async_trait;

pub struct PowerSupplyEmulator {
    state_oe: bool,
    voltage: String,
    current: String,
}

impl PowerSupplyEmulator {
    pub fn new(config: PowerSupplyConfig) -> Self {
        Self {
            state_oe: false,
            voltage: "0".into(),
            current: "0".into(),
        }
    }
}

#[async_trait]
impl PowerSupplyDriver for PowerSupplyEmulator {
    async fn output_enabled(&mut self) -> Result<bool, DriverError> {
        Ok(self.state_oe)
    }
    async fn enable_output(&mut self) -> Result<(), DriverError> {
        self.state_oe = true;
        Ok(())
    }
    async fn disable_output(&mut self) -> Result<(), DriverError> {
        self.state_oe = false;
        Ok(())
    }

    async fn get_voltage(&mut self) -> Result<String, DriverError> {
        Ok(self.voltage.clone())
    }
    async fn set_voltage(&mut self, voltage: String) -> Result<(), DriverError> {
        self.voltage = voltage;
        Ok(())
    }

    async fn get_current(&mut self) -> Result<String, DriverError> {
        Ok(self.current.clone())
    }
    async fn set_current(&mut self, current: String) -> Result<(), DriverError> {
        self.current = current;
        Ok(())
    }

    async fn measure_voltage(&mut self) -> Result<String, DriverError> {
        Ok("0".into())
    }
    async fn measure_current(&mut self) -> Result<String, DriverError> {
        Ok("0".into())
    }
}
