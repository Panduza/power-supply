pub struct MutableData {
    pub enabled: bool,
    pub voltage: String,
    pub current: String,
}

impl Default for MutableData {
    fn default() -> Self {
        Self {
            enabled: false,
            voltage: "0.00".to_string(),
            current: "0.00".to_string(),
        }
    }
}
