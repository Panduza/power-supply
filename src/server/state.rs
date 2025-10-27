use crate::config::ServerMainConfig;
use crate::server::factory::Factory;
use crate::server::instance::InstanceHandler;
use crate::server::instance::InstanceRunner;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

// Global state for sharing data between background services and GUI
#[derive(Clone, Debug)]
pub struct ServerState {
    /// Factory instance
    pub factory: Arc<Mutex<Factory>>,

    /// Server configuration
    pub server_config: Arc<Mutex<ServerMainConfig>>,

    /// Names of available instances
    pub instances: Arc<Mutex<HashMap<String, InstanceHandler>>>,
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

impl ServerState {
    // pub fn new(server_config: ServerMainConfig) -> Self {
    //     // // Update PSU names in app state
    //     // {
    //     //     let mut names = app_state.psu_names.lock().await;
    //     //     *names = psu_names.clone();
    //     // }

    //     // mcp::McpServer::run(config.clone(), psu_names)
    //     //     .await
    //     //     .unwrap();

    //     ServerRuntime { server_config }
    // }

    /// Start background runtime services
    pub async fn start_runtime(&self) -> anyhow::Result<()> {
        // Create a dedicated Tokio runtime for background tasks
        {
            let mut instances = HashMap::new();
            let factory = self.factory.lock().await;
            info!("Starting server runtime services...");
            if let Some(devices) = &self.server_config.lock().await.devices {
                for (name, device_config) in devices {
                    let instance = factory.instanciate_driver(device_config.clone())?;

                    instances.insert(name.clone(), InstanceRunner::start(name.clone(), instance)?);
                }
            }
            *self.instances.lock().await = instances;
        }

        Ok(())
    }

    pub async fn stop_runtime(&self) {}
}
