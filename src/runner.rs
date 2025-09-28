use std::time::Duration;

use rand::{distributions::Alphanumeric, Rng};
use rumqttc::{AsyncClient, MqttOptions};

use crate::drivers::PowerSupplyDriver;

fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect()
}

struct Runner {
    name: String,
    driver: Box<dyn PowerSupplyDriver>,
}

impl Runner {
    pub fn new(name: String, driver: Box<dyn PowerSupplyDriver>) -> Self {
        Self { name, driver }
    }

    pub fn start(self) {
        let mut mqttoptions = MqttOptions::new(
            format!("rumqtt-sync-{}", generate_random_string(5)),
            "localhost",
            1883,
        );
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        // Start the runner

        // self.driver.enable_output()
    }
}
