use crate::server::config::PowerSupplyConfig;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error as ThisError;
use tokio::sync::Mutex;
use tracing::error;

pub mod emulator;
pub mod kd3005p;

use async_trait::async_trait;

#[async_trait]
pub trait PowerSupplyDriver: Send + Sync {
    // --- Lifecycle management ---

    /// Initialize the driver
    async fn initialize(&mut self) -> anyhow::Result<()>;
    /// Shutdown the driver
    // async fn shutdown(&mut self) -> anyhow::Result<()>;

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

    // Decimals Support
    // Maximum number of decimal places supported for voltage settings
    // fn supported_voltage_decimals(&self) -> usize;

    // Security limits
    fn security_min_voltage(&self) -> Option<f32>;
    fn security_max_voltage(&self) -> Option<f32>;

    /// Get the current setting
    async fn get_current(&mut self) -> anyhow::Result<String>;
    /// Set the current setting
    async fn set_current(&mut self, current: String) -> anyhow::Result<()>;

    // Decimals Support
    // Maximum number of decimal places supported for current settings
    // fn supported_current_decimals(&self) -> usize;

    // Security limits
    fn security_min_current(&self) -> Option<f32>;
    fn security_max_current(&self) -> Option<f32>;
}

#[derive(ThisError, Debug, Clone)]
pub enum FactoryError {
    #[error("No driver found for model: {0}")]
    NoDriver(String),
}

#[derive(Clone, Debug)]
pub struct Factory {
    /// This map store Driver generators.
    /// Generator are function that return a PowerSupplyDriver
    pub map:
        HashMap<String, fn(PowerSupplyConfig) -> Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>>,

    /// The manifest of available power supply devices
    pub manifest: HashMap<String, serde_json::Value>,
}

impl Factory {
    /// Create a new empty Factory
    pub fn initialize() -> Self {
        let mut factory = Self {
            map: HashMap::new(),
            manifest: HashMap::new(),
        };

        // ----------------------------------------------------------
        factory.register_driver("emulator", |config| {
            Arc::new(Mutex::new(
                crate::server::drivers::emulator::PowerSupplyEmulator::new(config),
            ))
        });
        factory.manifest.insert(
            "emulator".to_string(),
            crate::server::drivers::emulator::PowerSupplyEmulator::manifest(),
        );

        // ----------------------------------------------------------

        factory.register_driver("kd3005p", |config| {
            Arc::new(Mutex::new(
                crate::server::drivers::kd3005p::Kd3005pDriver::new(config),
            ))
        });
        factory.manifest.insert(
            "kd3005p".to_string(),
            crate::server::drivers::kd3005p::Kd3005pDriver::manifest(),
        );

        // ----------------------------------------------------------
        factory
    }

    /// Register a new Driver generator
    pub fn register_driver<A: Into<String>>(
        &mut self,
        model: A,
        generator: fn(PowerSupplyConfig) -> Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>,
    ) {
        self.map.insert(model.into(), generator);
    }

    pub fn instanciate_driver(
        &self,
        config: PowerSupplyConfig,
    ) -> Result<Arc<Mutex<dyn PowerSupplyDriver + Send + Sync>>, FactoryError> {
        if let Some(generator) = self.map.get(&config.model) {
            Ok(generator(config))
        } else {
            Err(FactoryError::NoDriver(config.model))
        }
    }
}
