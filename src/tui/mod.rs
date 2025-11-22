//! TUI (Terminal User Interface) module for power supply control
//!
//! Provides a real-time terminal interface for monitoring and controlling
//! power supply devices using ratatui.

pub mod input;
pub mod layout;
pub mod render;
pub mod state;

#[cfg(test)]
mod tests;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Result};
use tracing::{info, trace};

use self::state::TuiState;

/// Main TUI application struct
pub struct TuiApp {
    /// Internal application state
    state: TuiState,
    /// Whether the application should continue running
    running: bool,
}

impl TuiApp {
    /// Create a new TUI application instance
    pub fn new() -> Self {
        Self {
            state: TuiState::default(),
            running: true,
        }
    }

    /// Run the TUI event loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting TUI application");

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main event loop
        while self.running {
            // Render current frame
            terminal.draw(|f| {
                render::draw_frame(f, &self.state);
            })?;

            // Handle events
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key_event(key.code);
                }
            }
        }

        // Cleanup
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        info!("TUI application stopped");
        Ok(())
    }

    /// Handle keyboard input events
    fn handle_key_event(&mut self, key: KeyCode) {
        trace!("Key pressed: {:?}", key);
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            _ => {}
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy run function for backward compatibility
pub async fn run() -> Result<()> {
    let mut app = TuiApp::new();
    app.run().await
}
