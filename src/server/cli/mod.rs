use clap::Parser;

/// Command line interface arguments for the power supply application
///
/// This struct defines the CLI interface that allows users to:
/// - List available power supply instances
/// - Disable the TUI for script usage
/// - Disable MCP servers for script usage
/// - Specify an optional instance name for TUI control
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// List power supply instance names
    #[arg(short = 'l', long)]
    pub list: bool,

    /// Disable the TUI and start only server services (for script usage)
    #[arg(long = "disable-tui")]
    pub disable_tui: bool,

    /// Disable MCP servers (for script usage)
    #[arg(long = "disable-mcp")]
    pub disable_mcp: bool,

    /// Optional instance name for TUI control (positional argument)
    /// If not specified, the application will choose the first instance available
    pub instance_name: Option<String>,
}
