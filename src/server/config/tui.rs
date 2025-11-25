use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TuiConfig {
    /// Keyboard shortcut to toggle power output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_toggle_key: Option<String>,
}
