use crate::constants::SERVER_TYPE_NAME;

/// Topics used for MQTT communication with the power supply
#[derive(Debug, Clone)]
pub struct Topics {
    // ---
    /// Topic for status updates
    status: String,

    /// Topic for error messages
    /// pza_id match the one from the command that caused the error
    error: String,

    // ---
    /// Topic to send state change commands
    /// /state/cmd
    state_cmd: String,
    /// Topic to receive state updates, acknowledgments from server
    /// /state
    state: String,

    // ---
    /// /voltage/cmd
    voltage_cmd: String,
    /// /voltage
    voltage: String,

    // ---
    /// /current/cmd
    current_cmd: String,
    /// /current
    current: String,
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
