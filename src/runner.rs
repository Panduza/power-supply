use crate::drivers::PowerSupplyDriver;

struct Runner {
    name: String,
    driver: Box<dyn PowerSupplyDriver>,
}

impl Runner {
    pub fn new(name: String, driver: Box<dyn PowerSupplyDriver>) -> Self {
        Self { name, driver }
    }

    pub fn start(self) {
        // Start the runner

        // self.driver.enable_output()
    }
}
