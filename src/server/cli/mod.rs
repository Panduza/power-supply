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
        /// Service overrides (flags to disable individual services)
        #[command(flatten)]
        services: ServicesOverrides,
    },
}

/// Grouping for flags that control which services to disable when running.
#[derive(clap::Args, Debug, Clone, PartialEq)]
pub struct ServicesOverrides {
    /// Disable the TUI
    #[arg(long = "no-tui")]
    pub no_tui: bool,

    /// Disable the embedded broker
    #[arg(long = "no-broker")]
    pub no_broker: bool,

    /// Disable MCP servers
    #[arg(long = "no-mcp")]
    pub no_mcp: bool,

    /// Disable runners
    #[arg(long = "no-runners")]
    pub no_runners: bool,

    /// Disable traces
    #[arg(long = "no-traces")]
    pub no_traces: bool,
}
