use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// List power supply instance names
    #[arg(short = 'l', long)]
    pub list: bool,

    /// Start a control box TUI for the specified instance
    #[arg(short = 't', long, value_name = "INSTANCE_NAME", num_args = 0..=1, default_missing_value = "", require_equals = true)]
    pub tui: Option<String>,
}
