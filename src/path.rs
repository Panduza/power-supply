use pza_toolkit::path::user_root_dir;
use std::path::PathBuf;

/// Get the path to the server configuration file
///
pub fn server_config_file() -> Option<PathBuf> {
    user_root_dir().map(|root| {
        root.join(format!(
            "{}-{}-server.json5",
            crate::constants::CONFIG_FILE_NAME_PREFIX,
            crate::constants::SERVER_TYPE_NAME
        ))
    })
}

/// Get the path to the factory manifest file
///
pub fn factory_manifest_file() -> Option<PathBuf> {
    user_root_dir().map(|root| {
        root.join(format!(
            "{}-{}-factory.json5",
            crate::constants::CONFIG_FILE_NAME_PREFIX,
            crate::constants::SERVER_TYPE_NAME
        ))
    })
}

/// Get the path to the scan file
///
pub fn scan_file() -> Option<PathBuf> {
    user_root_dir().map(|root| {
        root.join(format!(
            "{}-{}-scan.json5",
            crate::constants::CONFIG_FILE_NAME_PREFIX,
            crate::constants::SERVER_TYPE_NAME
        ))
    })
}
