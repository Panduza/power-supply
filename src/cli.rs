use clap::{Parser, Subcommand};

/// Panduza Power Supply - MQTT broker and device management
#[derive(Parser)]
#[command(name = "pza-power-supply")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Panduza Power Supply - MQTT broker and device management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch the graphical user interface (default)
    Gui,
    /// Run as a background server without GUI
    Server,
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Gui
    }
}
