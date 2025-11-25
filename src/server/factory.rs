use std::{collections::HashMap, sync::Arc};
use thiserror::Error as ThisError;
use tokio::sync::Mutex;
use tracing::error;
use tracing::info;

use crate::path::factory_manifest_file;
use crate::server::{config::PowerSupplyConfig, drivers::PowerSupplyDriver};

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

        // Write factory manifest to file
        if let Err(err) = factory.write_manifest_to_file() {
            error!("Failed to write factory manifest: {}", err);
        } else {
            info!("Factory manifest written successfully");
        }

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

    /// Write the manifest data to the factory manifest file
    pub fn write_manifest_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure the user root directory exists
        pza_toolkit::path::ensure_user_root_dir_exists()?;

        // Get the factory manifest file path
        let manifest_file_path =
            factory_manifest_file().ok_or("Unable to determine factory manifest file path")?;

        info!(
            "Writing factory manifest to: {}",
            manifest_file_path.display()
        );

        // Serialize the manifest data to pretty JSON
        let json_content = serde_json::to_string_pretty(&self.manifest)?;

        // Write to file
        std::fs::write(manifest_file_path, json_content)?;

        Ok(())
    }
}
