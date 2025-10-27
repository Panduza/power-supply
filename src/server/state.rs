use crate::server::runtime::ServerRuntime;
use std::sync::Arc;
use tokio::sync::Mutex;

// Global state for sharing data between background services and GUI
#[derive(Clone, Debug)]
pub struct ServerState {
    /// Server configuration
    pub runtime: Arc<Mutex<ServerRuntime>>,
    // Names of available instances
    // pub instance_names: Arc<Mutex<Vec<String>>>,
    // pub broker_config: Arc<Mutex<Option<client::config::MqttBrokerConfig>>>,
}

impl PartialEq for ServerState {
    fn eq(&self, other: &Self) -> bool {
        // // Note: This is a blocking comparison that requires async runtime
        // // In practice, you might want to use try_lock() or implement async comparison
        // let rt = tokio::runtime::Handle::try_current();
        // if let Ok(handle) = rt {
        //     let self_names = handle.block_on(self.instance_names.lock());
        //     let other_names = handle.block_on(other.instance_names.lock());
        //     *self_names == *other_names
        // } else {
        //     // Fallback: compare Arc pointers (same allocation = likely same data)
        //     Arc::ptr_eq(&self.instance_names, &other.instance_names)
        // }

        true
    }
}
