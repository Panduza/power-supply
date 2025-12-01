/// Loading widget for TUI display
///
/// Displays a simple loading message with animated border while backend starts.
use ratatui::layout::Alignment;
use ratatui::layout::Rect;
use ratatui::prelude::Buffer;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

// ================

/// Widget that displays a loading message with animated border
pub struct LoadingWidget {
    /// Message to display to the user
    message: String,
    /// Current animation frame for the border
    animation_frame: usize,
}

// ================

impl LoadingWidget {
    // ------------------------------------------------------------------------------

    /// Create a new loading widget with custom message and animation frame
    pub fn new(message: impl Into<String>, animation_frame: usize) -> Self {
        Self {
            message: message.into(),
            animation_frame,
        }
    }

    // ------------------------------------------------------------------------------

    /// Create a loading widget with custom message
    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            animation_frame: 0,
        }
    }

    // ------------------------------------------------------------------------------

    /// Set the animation frame for border animation
    pub fn frame(mut self, frame: usize) -> Self {
        self.animation_frame = frame;
        self
    }

    // ------------------------------------------------------------------------------

    /// Set a custom message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    // ------------------------------------------------------------------------------
}

// ================

impl Default for LoadingWidget {
    // ------------------------------------------------------------------------------

    fn default() -> Self {
        Self {
            message: "Please wait, backend is starting...".to_string(),
            animation_frame: 0,
        }
    }

    // ------------------------------------------------------------------------------
}

// ================

impl Widget for LoadingWidget {
    // ------------------------------------------------------------------------------

    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create animated title with dots
        let dots = ".".repeat((self.animation_frame / 10) % 4);
        let title = format!("Loading{}", dots);

        // Create border type based on animation frame
        let border_type = match (self.animation_frame / 5) % 4 {
            0 => BorderType::Plain,
            1 => BorderType::Rounded,
            2 => BorderType::Double,
            _ => BorderType::Thick,
        };

        // Create the block with animated border
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(border_type)
            .style(Style::default().fg(Color::Cyan));

        // Create paragraph with centered message
        let paragraph = Paragraph::new(self.message)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });

        // Render the widget
        paragraph.render(area, buf);
    }

    // ------------------------------------------------------------------------------
}
