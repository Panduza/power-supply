pub mod cli;
mod config;
pub mod factory;
pub mod mcp;
pub mod mqtt;
pub mod services;
pub mod state;
pub mod tui;

use crate::config::ServerMainConfig;
use crate::server::services::server_services;
use clap::Parser;
use pza_toolkit::dioxus::logger::LoggerBuilder;
pub use state::ServerState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, Level};

pub static SERVER_STATE_STORAGE: once_cell::sync::OnceCell<Arc<ServerState>> =
    once_cell::sync::OnceCell::new();

/// Run the power supply server
pub async fn run_server() {
    // Configure tracing first to be able to generate logs
    LoggerBuilder::default()
        .with_level(Level::TRACE)
        // .display_target(true)
        .filter_rumqttd()
        .filter_dioxus_core()
        .filter_dioxus_signals()
        .filter_warnings()
        .build()
        .expect("failed to init logger");

    // Parse CLI arguments
    let args = cli::Args::parse();

    // Ensure user root directory exists
    pza_toolkit::path::ensure_user_root_dir_exists()
        .unwrap_or_else(|err| panic!("Failed to ensure user root directory exists: {}", err));

    // Parse server main config file
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

    // Store server state in global storage
    SERVER_STATE_STORAGE
        .set(Arc::new(server_state.clone()))
        .unwrap();

    // Start server services in a separated task
    let services_handle = tokio::spawn(async move {
        server_services(
            SERVER_STATE_STORAGE
                .get()
                .expect("Failed to get server state")
                .clone(),
        )
        .await
        .expect("Server services crash");
    });

    // Start TUI at the end if requested by user
    if !args.disable_tui {
        info!("Starting TUI...");
        let instance_name = args.instance_name.filter(|s| !s.is_empty());
        if let Err(e) = tui::run_tui(instance_name).await {
            error!("TUI error: {}", e);
        }
        // Cancel server services when TUI exits
        services_handle.abort();
    } else {
        info!("Server is running...");
        // Wait for server services to complete
        services_handle
            .await
            .expect("Server services stopped unexpectedly");
    }
}
