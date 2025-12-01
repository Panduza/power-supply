use clap::{Parser, Subcommand};

/// Command line interface for the power supply application.
///
/// Provides the `list` subcommand to enumerate resources and the `run`
/// subcommand to start the application with optional services disabled.
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Top-level subcommands supported by the CLI
#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// List available resources (mcps, drivers, devices)
    List {
        /// Show MCP servers
        #[arg(long = "mcps")]
        mcps: bool,

        /// Show drivers
        #[arg(long = "drivers")]
        drivers: bool,

        /// Show devices
        #[arg(long = "devices")]
        devices: bool,
    },

    /// Run the power supply application (disable services with flags)
    Run {
        /// Disable the TUI
        #[arg(long = "no-tui")]
        no_tui: bool,

        /// Disable the embedded broker
        #[arg(long = "no-broker")]
        no_broker: bool,

        /// Disable MCP servers
        #[arg(long = "no-mcp")]
        no_mcp: bool,

        /// Disable runners
        #[arg(long = "no-runners")]
        no_runners: bool,

        /// Disable traces
        #[arg(long = "no-traces")]
        no_traces: bool,
    },
}
