use rand::{distributions::Alphanumeric, Rng};

/// Generate a random string of the specified length using alphanumeric characters
pub fn generate_random_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(|c| c as char)
        .collect()
}

/// Generate MQTT topic for a given power supply and suffix
pub fn psu_topic<A: Into<String>, B: Into<String>>(name: A, suffix: B) -> String {
    format!("power-supply/{}/{}", name.into(), suffix.into())
}
