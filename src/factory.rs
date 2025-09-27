use std::hash::Hash;





struct Factory {

    /// This map store Driver generators.
    /// Generator are function that return a PowerSupplyDriver
    map: HashMap<String, fn(PowerSupplyConfig) -> Box<dyn PowerSupplyDriver>>,

}


impl Factory {
    /// Create a new empty Factory
    pub fn new() -> Self {

        
        let self = Self {
            map: HashMap::new(),
        };


        self.register_driver("emulator", |config| {
            Box::new(emulator::PowerSupplyEmulator::new(config))
        });


        self
    }

    /// Register a new Driver generator
    pub fn register_driver<A: Into<String>>(&mut self, model: A, generator: fn(PowerSupplyConfig) -> Box<dyn PowerSupplyDriver>) {
        self.map.insert(model.into(), generator);
    }

    /// Create a new Driver instance based on the model
    pub fn create_driver(&self, config: PowerSupplyConfig) -> Result<Box<dyn PowerSupplyDriver>, DriverError> {
        if let Some(generator) = self.map.get(&config.model) {
            Ok(generator(config))
        } else {
            Err(DriverError::Generic(format!("No driver found for model: {}", config.model)))
        }
    }
    
}

