use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List power supply instance names
    #[arg(short = 'l', long)]
    list: bool,

    /// Start a control box TUI for the specified instance
    #[arg(short = 't', long, value_name = "INSTANCE_NAME")]
    tui: Option<String>,
}
