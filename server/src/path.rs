//! Path utilities for Panduza standardized file system locations
//!
//! This module provides handy functions to access all standardized paths of Panduza on systems.
//! It works cross-platform (Windows, Linux, Mac).

use std::fs;
use std::io;
use std::path::PathBuf;

/// Get the user root directory for Panduza
///
/// Returns the path to the `.panduza` directory inside the user's home directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to `~/.xdoctorwhoz`, or `None` if the home directory cannot be determined.
pub fn user_root_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".xdoctorwhoz"))
}

/// Get the path to the platform configuration file
///
/// Returns the path to `platform.json5` located in the user root directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to the platform configuration file, or `None` if the home directory cannot be determined.
pub fn global_config_file() -> Option<PathBuf> {
    user_root_dir().map(|root| root.join("panduza-power-supply.json5"))
}

// Directory and file management functions

/// Ensure that the user root directory exists
///
/// Creates the `.panduza` directory in the user's home directory if it doesn't exist.
///
/// # Returns
///
/// `Ok(())` if the directory exists or was created successfully, or an `io::Error` if creation failed.
pub fn ensure_user_root_dir_exists() -> io::Result<()> {
    if let Some(dir) = user_root_dir() {
        fs::create_dir_all(dir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to determine home directory",
        ))
    }
}
