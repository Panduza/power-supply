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
        Arc::ptr_eq(&self.factory, &other.factory)
            && Arc::ptr_eq(&self.server_config, &other.server_config)
            && Arc::ptr_eq(&self.instances, &other.instances)
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
