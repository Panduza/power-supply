pub mod cli;
pub mod config;
pub mod drivers;
// pub mod factory;
// pub mod mcp;
// pub mod mqtt;
// mod path;
// pub mod state;
// pub mod tui;
pub mod services;

use clap::Parser;
use config::ServerConfig;

// pub use state::ServerState;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Run the power supply server
pub async fn run_server() {
    // Parse CLI arguments first to determine if TUI will be used
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::List {
            mcps,
            drivers,
            devices,
        } => {
            // Handle the 'list' command
            if mcps {
                ServerConfig::from_user_file()
                    .unwrap_or_else(|err| panic!("Failed to load server configuration: {}", err))
                    .print_mcp_servers_urls();
            }
            if drivers {
                println!("Listing drivers...");
                // Implementation for listing drivers goes here
            }
            if devices {
                println!("Listing devices...");
                // Implementation for listing devices goes here
            }
        }
        cli::Commands::Version => {
            // Display the version from Cargo.toml
            println!("{}", env!("PKG_VERSION"));
        }
        cli::Commands::Run { services } => {
            // Load server configuration
            let server_config = ServerConfig::from_user_file()
                .unwrap_or_else(|err| panic!("Failed to load server configuration: {}", err))
                .apply_overrides(&services)
                .setup_tracing()
                .trace_config();

            // Load driver factory
            let factory = drivers::Factory::initialize();

            // Create Services instance
            let mut services =
                services::Services::new(server_config, Arc::new(Mutex::new(factory)));

            // Start services
            if let Err(e) = services.start().await {
                eprintln!("Failed to start services: {}", e);
            }
        }
    }
}
