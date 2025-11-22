#[cfg(test)]
mod tests {
    use crate::tui::TuiApp;

    #[test]
    fn test_tui_app_new() {
        let _app = TuiApp::new();
        // Creation should succeed without panic
        // We test behavior through integration tests since fields are private
    }

    #[test]
    fn test_tui_app_default() {
        let _app1 = TuiApp::new();
        let _app2 = TuiApp::default();
        
        // Both should create successfully
        // Behavior equality tested through integration tests
    }
}

#[cfg(test)]
mod state_tests {
    use crate::tui::state::TuiState;

    #[test]
    fn test_tui_state_new() {
        let state = TuiState::new();
        
        assert!(!state.power_on());
        assert_eq!(state.voltage(), 0.0);
        assert_eq!(state.current(), 0.0);
        assert!(!state.is_help_visible());
        assert!(!state.is_connected());
    }

    #[test]
    fn test_tui_state_setters() {
        let mut state = TuiState::new();
        
        state.set_power_on(true);
        state.set_voltage(12.5);
        state.set_current(2.5);
        state.set_connected(true);
        
        assert!(state.power_on());
        assert_eq!(state.voltage(), 12.5);
        assert_eq!(state.current(), 2.5);
        assert!(state.is_connected());
    }

    #[test]
    fn test_tui_state_toggle_help() {
        let mut state = TuiState::new();
        
        assert!(!state.is_help_visible());
        state.toggle_help();
        assert!(state.is_help_visible());
        state.toggle_help();
        assert!(!state.is_help_visible());
    }

    #[test]
    fn test_tui_state_update_readings() {
        let mut state = TuiState::new();
        
        state.update_readings(true, 24.0, 1.5);
        
        assert!(state.power_on());
        assert_eq!(state.voltage(), 24.0);
        assert_eq!(state.current(), 1.5);
    }
}

#[cfg(test)]
mod input_tests {
    use crate::tui::input::{InputHandler, InputAction};
    use crossterm::event::KeyCode;

    #[test]
    fn test_input_handler_exit_keys() {
        let mut handler = InputHandler::new();
        
        assert_eq!(handler.handle_key(KeyCode::Char('q')), Some(InputAction::Exit));
        assert_eq!(handler.handle_key(KeyCode::Esc), Some(InputAction::Exit));
    }

    #[test]
    fn test_input_handler_toggle_power() {
        let mut handler = InputHandler::new();
        
        assert_eq!(handler.handle_key(KeyCode::Char(' ')), Some(InputAction::TogglePower));
    }

    #[test]
    fn test_input_handler_help_toggle() {
        let mut handler = InputHandler::new();
        
        assert!(!handler.is_help_visible());
        assert_eq!(handler.handle_key(KeyCode::Char('?')), Some(InputAction::ToggleHelp));
        assert!(handler.is_help_visible());
        assert_eq!(handler.handle_key(KeyCode::Char('?')), Some(InputAction::ToggleHelp));
        assert!(!handler.is_help_visible());
    }

    #[test]
    fn test_input_handler_unknown_key() {
        let mut handler = InputHandler::new();
        
        assert_eq!(handler.handle_key(KeyCode::Char('x')), None);
        assert_eq!(handler.handle_key(KeyCode::Enter), None);
    }
}