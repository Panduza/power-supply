//! Path utilities for Panduza standardized file system locations
//!
//! This module provides handy functions to access all standardized paths of Panduza on systems.
//! It works cross-platform (Windows, Linux, Mac).

use pza_power_supply_client::SERVER_TYPE_NAME;
use pza_toolkit::path::server_configs_dir;
use std::path::PathBuf;

/// Get the path to the server configuration file
///
pub fn server_config_file() -> Option<PathBuf> {
    server_configs_dir().map(|root| root.join(format!("pza-{}.json5", SERVER_TYPE_NAME)))
}
