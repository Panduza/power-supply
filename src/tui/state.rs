//! State management for TUI application
//!
//! Handles application state, power supply data, and UI state tracking.

/// Main application state for the TUI
#[derive(Debug, Clone)]
pub struct TuiState {
    /// Power supply power state
    power_on: bool,
    /// Current voltage reading (V)
    voltage: f64,
    /// Current current reading (A)
    current: f64,
    /// Whether help overlay is visible
    help_visible: bool,
    /// Connection status to backend
    connected: bool,
}

impl TuiState {
    /// Create new TUI state with default values
    pub fn new() -> Self {
        Self {
            power_on: false,
            voltage: 0.0,
            current: 0.0,
            help_visible: false,
            connected: false,
        }
    }

    // Getters
    /// Get power state
    pub fn power_on(&self) -> bool {
        self.power_on
    }

    /// Get voltage value
    pub fn voltage(&self) -> f64 {
        self.voltage
    }

    /// Get current value
    pub fn current(&self) -> f64 {
        self.current
    }

    /// Get help visibility state
    pub fn is_help_visible(&self) -> bool {
        self.help_visible
    }

    /// Get connection status
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    // Setters
    /// Set power state
    pub fn set_power_on(&mut self, power_on: bool) {
        self.power_on = power_on;
    }

    /// Set voltage value
    pub fn set_voltage(&mut self, voltage: f64) {
        self.voltage = voltage;
    }

    /// Set current value
    pub fn set_current(&mut self, current: f64) {
        self.current = current;
    }

    /// Toggle help visibility
    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    /// Set connection status
    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
    }

    /// Update all power supply readings at once
    pub fn update_readings(&mut self, power_on: bool, voltage: f64, current: f64) {
        self.power_on = power_on;
        self.voltage = voltage;
        self.current = current;
    }
}

impl Default for TuiState {
    fn default() -> Self {
        Self::new()
    }
}