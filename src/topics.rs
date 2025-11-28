use crate::constants::SERVER_TYPE_NAME;

pub enum TopicId {
    Status,
    Error,
    StateCmd,
    State,
    VoltageCmd,
    Voltage,
    CurrentCmd,
    Current,
}

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

    /// Get a vector of all client subscription topics
    pub fn vec_sub_client(&self) -> Vec<String> {
        vec![
            self.status.clone(),
            self.error.clone(),
            self.state.clone(),
            self.voltage.clone(),
            self.current.clone(),
        ]
    }

    /// Get a vector of all server subscription topics
    pub fn vec_sub_server(&self) -> Vec<String> {
        vec![
            self.state_cmd.clone(),
            self.voltage_cmd.clone(),
            self.current_cmd.clone(),
        ]
    }

    pub fn topic_to_id(&self, topic: &str) -> Option<TopicId> {
        if topic == self.status {
            Some(TopicId::Status)
        } else if topic == self.error {
            Some(TopicId::Error)
        } else if topic == self.state_cmd {
            Some(TopicId::StateCmd)
        } else if topic == self.state {
            Some(TopicId::State)
        } else if topic == self.voltage_cmd {
            Some(TopicId::VoltageCmd)
        } else if topic == self.voltage {
            Some(TopicId::Voltage)
        } else if topic == self.current_cmd {
            Some(TopicId::CurrentCmd)
        } else if topic == self.current {
            Some(TopicId::Current)
        } else {
            None
        }
    }

    pub fn id_to_topic(&self, id: &TopicId) -> &str {
        match id {
            TopicId::Status => &self.status,
            TopicId::Error => &self.error,
            TopicId::StateCmd => &self.state_cmd,
            TopicId::State => &self.state,
            TopicId::VoltageCmd => &self.voltage_cmd,
            TopicId::Voltage => &self.voltage,
            TopicId::CurrentCmd => &self.current_cmd,
            TopicId::Current => &self.current,
        }
    }
}
