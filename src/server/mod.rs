pub mod cli;
pub mod factory;
pub mod mcp;
pub mod mqtt;
pub mod services;
pub mod state;
pub mod tui;

use crate::config::ServerMainConfig;
use crate::server::services::server_services;
use clap::Parser;
use dioxus::prelude::*;
use pza_toolkit::dioxus::logger::LoggerBuilder;
pub use state::ServerState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::Level;

pub static SERVER_STATE_STORAGE: once_cell::sync::OnceCell<Arc<ServerState>> =
    once_cell::sync::OnceCell::new();

/// Run the power supply server
pub fn run_server() {
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

    // Parse CLI arguments
    let args = cli::Args::parse();

    // Handle CLI commands
    if args.list {
        println!("Available power supply instances:");
        println!("(Instance discovery not yet implemented)");
        return;
    }

    // if args.tui.is_some() {
    //     println!("Starting TUI...");
    //     let instance_name = args.tui.filter(|s| !s.is_empty());
    //     if let Err(e) = server::tui::run_tui(instance_name) {
    //         eprintln!("TUI error: {}", e);
    //     }
    //     return;
    // }

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
}
