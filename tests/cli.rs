use clap::Parser;
use pza_power_supply::cli::{Cli, Commands};

#[test]
fn test_cli_help() {
    let result = Cli::try_parse_from(&["pza-power-supply", "--help"]);
    // Help output causes clap to return an error (but it's expected)
    assert!(result.is_err());
}

#[test]
fn test_cli_version() {
    let result = Cli::try_parse_from(&["pza-power-supply", "--version"]);
    // Version output causes clap to return an error (but it's expected)
    assert!(result.is_err());
}

#[test]
fn test_cli_gui_subcommand() {
    let cli = Cli::try_parse_from(&["pza-power-supply", "gui"]).unwrap();
    match cli.command.unwrap() {
        Commands::Gui => (),
        _ => panic!("Expected Gui command"),
    }
}

#[test]
fn test_cli_server_subcommand() {
    let cli = Cli::try_parse_from(&["pza-power-supply", "server"]).unwrap();
    match cli.command.unwrap() {
        Commands::Server => (),
        _ => panic!("Expected Server command"),
    }
}

#[test]
fn test_cli_default_command() {
    let cli = Cli::try_parse_from(&["pza-power-supply"]).unwrap();
    // When no subcommand is provided, command should be None, which defaults to Gui
    match cli.command.unwrap_or_default() {
        Commands::Gui => (),
        _ => panic!("Expected default to be Gui command"),
    }
}