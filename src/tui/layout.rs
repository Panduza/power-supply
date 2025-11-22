//! Layout management for TUI components
//!
//! Handles terminal size validation, responsive layouts, and component positioning.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
};

/// Minimum supported terminal dimensions
pub const MIN_WIDTH: u16 = 80;
pub const MIN_HEIGHT: u16 = 24;

/// Layout manager for TUI components
pub struct LayoutManager;

impl LayoutManager {
    /// Check if terminal size meets minimum requirements
    pub fn validate_terminal_size(area: Rect) -> bool {
        area.width >= MIN_WIDTH && area.height >= MIN_HEIGHT
    }

    /// Create the main control box layout
    pub fn control_box_layout(area: Rect) -> std::rc::Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3), // Power
                Constraint::Length(3), // Voltage
                Constraint::Length(3), // Current
                Constraint::Min(0),    // Remaining space
            ])
            .split(area)
    }

    /// Create warning message layout for small terminals
    pub fn warning_layout(area: Rect) -> Rect {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(50),
                Constraint::Min(0),
            ])
            .split(chunks[1]);

        horizontal[1]
    }

    /// Create help overlay layout
    pub fn help_overlay_layout(area: Rect) -> Rect {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(40),
                Constraint::Min(0),
            ])
            .split(chunks[1]);

        horizontal[1]
    }
}