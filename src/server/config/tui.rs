use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TuiConfig {
    /// Enable or disable the TUI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// Keyboard shortcut to toggle power output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_toggle_key: Option<String>,
}
