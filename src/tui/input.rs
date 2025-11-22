//! Input handling for TUI keyboard events and user interaction
//!
//! Handles key mapping, input validation, and action dispatching.

use crossterm::event::KeyCode;
use tracing::trace;

/// Handles keyboard input events for the TUI
pub struct InputHandler {
    /// Whether help overlay is currently shown
    help_visible: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            help_visible: false,
        }
    }

    /// Process a key event and return the corresponding action
    pub fn handle_key(&mut self, key: KeyCode) -> Option<InputAction> {
        trace!("Processing key: {:?}", key);

        match key {
            KeyCode::Char('q') | KeyCode::Esc => Some(InputAction::Exit),
            KeyCode::Char('?') => {
                self.help_visible = !self.help_visible;
                Some(InputAction::ToggleHelp)
            }
            KeyCode::Char(' ') => Some(InputAction::TogglePower),
            _ => None,
        }
    }

    /// Returns whether help overlay is visible
    pub fn is_help_visible(&self) -> bool {
        self.help_visible
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions that can be triggered by user input
#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    /// Exit the application
    Exit,
    /// Toggle power state
    TogglePower,
    /// Show/hide help overlay
    ToggleHelp,
}
