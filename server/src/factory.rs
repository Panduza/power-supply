use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use thiserror::Error as ThisError;

use crate::{config::PowerSupplyConfig, drivers::PowerSupplyDriver};

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
}

impl Factory {
    /// Create a new empty Factory
    pub fn new() -> Self {
        let mut factory = Self {
            map: HashMap::new(),
        };

        factory.register_driver("emulator", |config| {
            Arc::new(Mutex::new(
                crate::drivers::emulator::PowerSupplyEmulator::new(config),
            ))
        });

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
