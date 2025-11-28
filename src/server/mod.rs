pub mod cli;
pub mod config;
pub mod drivers;
pub mod factory;
pub mod mcp;
pub mod mqtt;
pub mod state;
pub mod tui;

use crate::server::config::ServerMainConfig;
use clap::Parser;
use pza_toolkit::dioxus::logger::LoggerBuilder;
pub use state::ServerState;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, Level};

pub static SERVER_STATE_STORAGE: once_cell::sync::OnceCell<Arc<ServerState>> =
    once_cell::sync::OnceCell::new();

/// Run the power supply server
pub async fn run_server() {
    // Parse CLI arguments first to determine if TUI will be used
    let args = cli::Args::parse();

    // Configure tracing only if TUI is not going to be used
    // This prevents tracing output from interfering with the TUI display
    if args.disable_tui {
        LoggerBuilder::default()
            .with_level(Level::TRACE)
            // .display_target(true)
            .filter_rumqttd()
            .filter_dioxus_core()
            .filter_dioxus_signals()
            .filter_warnings()
            .build()
            .expect("failed to init logger");
    }

    // Ensure user root directory exists
    pza_toolkit::path::ensure_user_root_dir_exists()
        .unwrap_or_else(|err| panic!("Failed to ensure user root directory exists: {}", err));

    // Parse server main config file
    let server_config = ServerMainConfig::from_user_file()
        .unwrap_or_else(|err| panic!("Failed to load server configuration: {}", err));

    // Create factory
    let factory = crate::server::factory::Factory::initialize();

    // Create global app state
    let server_state = ServerState::new(
        Arc::new(Mutex::new(factory)),
        Arc::new(Mutex::new(server_config)),
        args.clone(),
    );

    // Store server state in global storage
    SERVER_STATE_STORAGE
        .set(Arc::new(server_state.clone()))
        .unwrap();

    // Start server services in a separated task
    let services_handle = tokio::spawn(async move {
        SERVER_STATE_STORAGE
            .get()
            .expect("Failed to get server state")
            .clone()
            .start_services()
            .await
            .expect("Server services crash");
    });

    // Start TUI at the end if requested by user
    if !server_state.args.disable_tui {
        // Note: Tracing is not initialized when TUI is enabled to avoid
        // log output interfering with the terminal user interface
        let instance_name = server_state.args.instance_name.filter(|s| !s.is_empty());
        if let Err(e) = tui::run_tui(instance_name).await {
            eprintln!("TUI error: {}", e); // Use eprintln since tracing is not available
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
