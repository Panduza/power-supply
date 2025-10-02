use std::{collections::HashMap, sync::Arc};
use thiserror::Error as ThisError;
use tokio::sync::Mutex;
use tracing::info;

use crate::{config::PowerSupplyConfig, drivers::PowerSupplyDriver, path};

#[derive(ThisError, Debug, Clone)]
pub enum FactoryError {
    #[error("No driver found for model: {0}")]
    NoDriver(String),
}

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
    pub fn new() -> Self {
        let mut factory = Self {
            map: HashMap::new(),
            manifest: HashMap::new(),
        };

        factory.register_driver("emulator", |config| {
            Arc::new(Mutex::new(
                crate::drivers::emulator::PowerSupplyEmulator::new(config),
            ))
        });

        factory.manifest.insert(
            "emulator".to_string(),
            crate::drivers::emulator::PowerSupplyEmulator::manifest(),
        );

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
        path::ensure_user_root_dir_exists()?;

        // Get the factory manifest file path
        let manifest_file_path = path::factory_manifest_file()
            .ok_or("Unable to determine factory manifest file path")?;

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
