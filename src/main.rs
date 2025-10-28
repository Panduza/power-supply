mod config;
mod drivers;

mod constants;

mod path;
mod server;

mod client;

use crate::server::services::server_services;
use dioxus::prelude::*;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::Level;

use pza_toolkit::dioxus::logger::LoggerBuilder;

use config::ServerMainConfig;
use server::ServerState;

pub static SERVER_STATE_STORAGE: once_cell::sync::OnceCell<Arc<ServerState>> =
    once_cell::sync::OnceCell::new();

fn main() {
    // Init logger
    LoggerBuilder::default()
        .with_level(Level::TRACE)
        // .display_target(true)
        .filter_rumqttd()
        .filter_dioxus_core()
        .filter_dioxus_signals()
        .filter_warnings()
        .build()
        .expect("failed to init logger");

    // Ensure user root directory exists
    pza_toolkit::path::ensure_user_root_dir_exists()
        .unwrap_or_else(|err| panic!("Failed to ensure user root directory exists: {}", err));

    // Get user configuration
    let server_config = ServerMainConfig::from_user_file()
        .unwrap_or_else(|err| panic!("Failed to load server configuration: {}", err));

    // Create factory
    let factory = crate::server::factory::Factory::initialize();

    // Create global app state
    let server_state = ServerState {
        factory: Arc::new(Mutex::new(factory)),
        server_config: Arc::new(Mutex::new(server_config)),
        instances: Arc::new(Mutex::new(HashMap::new())),
    };

    SERVER_STATE_STORAGE
        .set(Arc::new(server_state.clone()))
        .unwrap();

    // Spawn background initialization and management task
    std::thread::spawn(move || {
        // Create a dedicated Tokio runtime for background tasks
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(server_services(
            SERVER_STATE_STORAGE
                .get()
                .expect("Failed to get server state")
                .clone(),
        ))
        .expect("Server services crash");
    });

    // Launch Dioxus app on the main thread
    dioxus::launch(server::Gui);
}

// async fn initialize_background_services(
//     instances: Arc<Mutex<Vec<RunnerHandler>>>,
//     app_state: ServerState,
// ) {

// // Initialize devices
// let mut psu_names = Vec::new();
// let mut instance_handles = Vec::new();
// if let Some(devices) = &config.devices {
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

// // Update PSU names in app state
// {
//     let mut names = app_state.psu_names.lock().await;
//     *names = psu_names.clone();
// }

// mcp::McpServer::run(config.clone(), psu_names)
//     .await
//     .unwrap();

// // Store instances for later management
// let mut locked_instances = instances.lock().await;
// *locked_instances = instance_handles;

// debug!("Background services initialized successfully");

// // Keep the runtime alive for background tasks
// loop {
//     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
// }
// }
