use crate::config::ServerMainConfig;
use crate::server::factory::Factory;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ServerRuntime {
    /// Factory instance
    pub factory: Arc<Mutex<Factory>>,

    /// Server configuration
    pub server_config: Arc<Mutex<ServerMainConfig>>,
}

impl ServerRuntime {
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

    pub fn start_runtime(&self) {
        // // // Initialize devices
        // // let mut psu_names = Vec::new();
        // // let mut instance_handles = Vec::new();
        // if let Some(devices) = &self.server_config.devices {
        //     for (name, device_config) in devices {
        //         let instance = factory
        //             .instanciate_driver(device_config.clone())
        //             .unwrap_or_else(|err| {
        //                 panic!("Failed to create driver for device '{}': {}", name, err)
        //             });

        //         psu_names.push(name.clone());

        //         let runner = Runner::start(name.clone(), instance);
        //         instance_handles.push(runner);
        //     }
        // }
    }

    pub fn stop_runtime(&self) {}
}
