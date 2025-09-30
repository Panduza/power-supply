use crate::{config::PowerSupplyConfig, drivers::PowerSupplyDriver};
use async_trait::async_trait;

pub struct PowerSupplyEmulator {
    state_oe: bool,
}

impl PowerSupplyEmulator {
    pub fn new(config: PowerSupplyConfig) -> Self {
        Self { state_oe: false }
    }
}

#[async_trait]
impl PowerSupplyDriver for PowerSupplyEmulator {
    async fn output_enabled(&mut self) -> Result<bool, crate::drivers::DriverError> {
        Ok(self.state_oe)
    }

    async fn enable_output(&mut self) -> Result<(), crate::drivers::DriverError> {
        self.state_oe = true;
        Ok(())
    }

    async fn disable_output(&mut self) -> Result<(), crate::drivers::DriverError> {
        self.state_oe = false;
        Ok(())
    }
}
