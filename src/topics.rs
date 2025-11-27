use crate::constants::SERVER_TYPE_NAME;

/// Topics used for MQTT communication with the power supply
#[derive(Debug, Clone)]
pub struct Topics {
    // ---
    /// Topic for status updates
    pub status: String,
    /// Topic for error messages
    /// pza_id match the one from the command that caused the error
    pub error: String,
    // ---
    /// Topic to send state change commands
    /// /state/cmd
    pub state_cmd: String,
    /// Topic to receive state updates, acknowledgments from server
    /// /state
    pub state: String,
    // ---
    /// /voltage/cmd
    pub voltage_cmd: String,
    /// /voltage
    pub voltage: String,
    // ---
    /// /current/cmd
    pub current_cmd: String,
    /// /current
    pub current: String,
}

impl Topics {
    /// Create a new Topics instance with the given prefix
    pub fn new<A: AsRef<str>>(name: A) -> Self {
        let prefix = format!("{}/{}", SERVER_TYPE_NAME, name.as_ref());
        Self {
            status: format!("{}/status", prefix),
            error: format!("{}/error", prefix),
            state_cmd: format!("{}/state/cmd", prefix),
            state: format!("{}/state", prefix),
            voltage_cmd: format!("{}/voltage/cmd", prefix),
            voltage: format!("{}/voltage", prefix),
            current_cmd: format!("{}/current/cmd", prefix),
            current: format!("{}/current", prefix),
        }
    }
}
