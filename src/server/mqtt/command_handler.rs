use super::topic_suffix;

/// Enum pour identifier les différents handlers
#[derive(Debug, Clone)]
pub enum CommandHandler {
    OutputSet,
    VoltageSet,
    CurrentSet,
}

impl CommandHandler {
    /// Convertit une chaîne de caractères en un CommandHandler
    pub fn from_str(handler_str: &str) -> Option<Self> {
        match handler_str {
            topic_suffix::CONTROL_OE_CMD => Some(CommandHandler::OutputSet),
            topic_suffix::CONTROL_VOLTAGE_CMD => Some(CommandHandler::VoltageSet),
            topic_suffix::CONTROL_CURRENT_CMD => Some(CommandHandler::CurrentSet),
            _ => None,
        }
    }
}
