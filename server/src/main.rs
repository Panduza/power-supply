mod broker;
mod config;
mod drivers;
mod factory;
mod gui;
mod mcp;
mod path;
mod runner;

use gui::Gui;
use runner::Runner;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    // Create a dedicated Tokio runtime for background tasks
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    // Store runtime and instances in Arc for sharing between threads
    let runtime = Arc::new(rt);
    let instances = Arc::new(Mutex::new(Vec::new()));

    // Clone for the background task
    let runtime_clone = Arc::clone(&runtime);
    let instances_clone = Arc::clone(&instances);

    // Spawn background initialization and management task
    std::thread::spawn(move || {
        runtime_clone.block_on(async {
            initialize_background_services(instances_clone).await;
        });
    });

    // Launch Dioxus app on the main thread
    dioxus::launch(Gui);
}

async fn initialize_background_services(instances: Arc<Mutex<Vec<runner::RunnerHandler>>>) {
    // Get user configuration
    let config = config::GlobalConfig::from_user_file();
    debug!("Loaded configuration: {:?}", config);

    // Create factory
    let factory = factory::Factory::new();
    debug!("Factory initialized with drivers: {:?}", factory.map.keys());

    // Start MQTT broker
    let _broker_handle = broker::start(&config);

    // Initialize devices
    let mut psu_names = Vec::new();
    let mut instance_handles = Vec::new();
    if let Some(devices) = &config.devices {
        for (name, device_config) in devices {
            let instance = factory
                .instanciate_driver(device_config.clone())
                .unwrap_or_else(|err| {
                    panic!("Failed to create driver for device '{}': {}", name, err)
                });

            psu_names.push(name.clone());

            let runner = Runner::start(name.clone(), instance);
            instance_handles.push(runner);
        }
    }

    mcp::McpServer::run(config.clone(), psu_names)
        .await
        .unwrap();

    // Store instances for later management
    let mut locked_instances = instances.lock().await;
    *locked_instances = instance_handles;

    debug!("Background services initialized successfully");

    // Keep the runtime alive for background tasks
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
