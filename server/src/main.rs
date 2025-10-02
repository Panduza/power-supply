mod broker;
mod config;
mod drivers;
mod factory;
mod gui;
mod mcp;
mod path;
mod runner;

use dioxus::prelude::*;
use runner::Runner;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, Level};

// Global state for sharing data between background services and GUI
#[derive(Clone, Debug)]
pub struct AppState {
    pub psu_names: Arc<Mutex<Vec<String>>>,
    pub broker_config: Arc<Mutex<Option<panduza_power_supply_client::config::MqttBrokerConfig>>>,
}

// Static storage for app state
static APP_STATE_STORAGE: std::sync::OnceLock<AppState> = std::sync::OnceLock::new();

// App component that provides context
fn app_component() -> Element {
    // Get the app state from the static storage
    let app_state = APP_STATE_STORAGE.get().unwrap().clone();

    use_context_provider(|| app_state);

    rsx! {
        gui::Gui {}
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    // Create global app state
    let app_state = AppState {
        psu_names: Arc::new(Mutex::new(Vec::new())),
        broker_config: Arc::new(Mutex::new(None)),
    };

    // Create a dedicated Tokio runtime for background tasks
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    // Store runtime and instances in Arc for sharing between threads
    let runtime = Arc::new(rt);
    let instances = Arc::new(Mutex::new(Vec::new()));

    // Clone for the background task
    let runtime_clone = Arc::clone(&runtime);
    let instances_clone = Arc::clone(&instances);
    let app_state_clone = app_state.clone();

    // Spawn background initialization and management task
    std::thread::spawn(move || {
        runtime_clone.block_on(async {
            initialize_background_services(instances_clone, app_state_clone).await;
        });
    });

    // Store app_state globally for the Dioxus app
    APP_STATE_STORAGE.set(app_state).unwrap();

    // Launch Dioxus app on the main thread
    dioxus::launch(app_component);
}

async fn initialize_background_services(
    instances: Arc<Mutex<Vec<runner::RunnerHandler>>>,
    app_state: AppState,
) {
    // Get user configuration
    let config = config::GlobalConfig::from_user_file();
    debug!("Loaded configuration: {:?}", config);

    // Update broker config in app state
    {
        let mut broker_config = app_state.broker_config.lock().await;
        *broker_config = Some(config.broker.clone());
    }

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

    // Update PSU names in app state
    {
        let mut names = app_state.psu_names.lock().await;
        *names = psu_names.clone();
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
