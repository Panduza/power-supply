use std::sync::Arc;
use tokio::sync::Mutex;

// Global state for sharing data between background services and GUI
#[derive(Clone, Debug)]
pub struct ServerState {
    /// Names of available instances
    pub instances_names: Arc<Mutex<Vec<String>>>,
    // pub broker_config: Arc<Mutex<Option<client::config::MqttBrokerConfig>>>,
}
