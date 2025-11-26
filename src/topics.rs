use crate::constants::SERVER_TYPE_NAME;

/// Topics used for MQTT communication with the power supply
#[derive(Debug, Clone)]
pub struct Topics {
    // ---
    /// Topic for status updates
    topic_status: String,

    /// Topic for error messages
    /// pza_id match the one from the command that caused the error
    topic_error: String,

    // ---
    /// Topic to send state change commands
    /// /state/cmd
    topic_state_cmd: String,
    /// Topic to receive state updates, acknowledgments from server
    /// /state
    topic_state: String,

    // ---
    /// /voltage/cmd
    topic_voltage_cmd: String,
    /// /voltage
    topic_voltage: String,

    // ---
    /// /current/cmd
    topic_current_cmd: String,
    /// /current
    topic_current: String,
}

impl Topics {
    /// Create a new Topics instance with the given prefix
    pub fn new<A: AsRef<str>>(name: A) -> Self {
        let prefix = format!("{}/{}", SERVER_TYPE_NAME, name.as_ref());
        Self {
            topic_status: format!("{}/status", prefix),
            topic_error: format!("{}/error", prefix),
            topic_state_cmd: format!("{}/state/cmd", prefix),
            topic_state: format!("{}/state", prefix),
            topic_voltage_cmd: format!("{}/voltage/cmd", prefix),
            topic_voltage: format!("{}/voltage", prefix),
            topic_current_cmd: format!("{}/current/cmd", prefix),
            topic_current: format!("{}/current", prefix),
        }
    }
}
