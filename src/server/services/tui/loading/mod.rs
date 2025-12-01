/// Loading widget for TUI display
///
/// Displays a loading message with smooth animated effects using tachyonfx.
use ratatui::layout::Alignment;
use ratatui::layout::Rect;
use ratatui::prelude::Buffer;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use tachyonfx::{fx, CellFilter, Duration, Effect, EffectTimer, Interpolation};

// ================

/// Widget that displays a loading message with smooth animations
pub struct LoadingWidget {
    /// Message to display to the user
    message: String,
    /// Start time for animation calculations (milliseconds since epoch)
    start_time: u64,
    /// Current border effect instance
    border_effect: Option<Effect>,
}

// ================

impl LoadingWidget {
    // ------------------------------------------------------------------------------

    /// Create a new loading widget with custom message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            start_time: Self::current_time_ms(),
            border_effect: None,
        }
    }

    // ------------------------------------------------------------------------------

    /// Create a loading widget with custom message  
    pub fn with_message(message: impl Into<String>) -> Self {
        Self::new(message)
    }

    // ------------------------------------------------------------------------------

    /// Set a custom message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Get the current message
    pub fn get_message(&self) -> &str {
        &self.message
    }

    // ------------------------------------------------------------------------------

    /// Get current time in milliseconds
    fn current_time_ms() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    // ------------------------------------------------------------------------------

    /// Create and return glowing border effect
    pub fn create_border_effect(&self) -> Effect {
        // Create glowing border effect that cycles every 2 seconds
        let cycle_duration = 2000; // 2 seconds

        // Create a simple glowing effect - just use a single effect for now
        let glow_effect = fx::hsl_shift_fg(
            [0.0, 0.0, 30.0], // Increase lightness for glow effect
            EffectTimer::from_ms(cycle_duration, Interpolation::SineInOut),
        );

        // Return repeating effect directly (not boxed)
        fx::repeating(glow_effect)
    }

    /// Apply loading animation effects to the rendered buffer
    pub fn apply_effects(&mut self, buf: &mut Buffer, area: Rect) {
        // Create or update the border effect
        if self.border_effect.is_none() {
            self.border_effect = Some(self.create_border_effect());
        }

        // Apply the effect to the buffer with proper duration
        if let Some(effect) = &mut self.border_effect {
            let frame_duration = Duration::from_millis(16); // ~60fps
            effect.process(frame_duration, buf, area);
        }
    }

    // ------------------------------------------------------------------------------
}

// ================

impl Default for LoadingWidget {
    // ------------------------------------------------------------------------------

    fn default() -> Self {
        Self {
            message: "Please wait, backend is starting...".to_string(),
            start_time: Self::current_time_ms(),
            border_effect: None,
        }
    }

    // ------------------------------------------------------------------------------
}

// ================

impl Widget for LoadingWidget {
    // ------------------------------------------------------------------------------

    fn render(self, area: Rect, buf: &mut Buffer) {
        let elapsed = Self::current_time_ms().saturating_sub(self.start_time);

        // Create animated title with smooth dot cycling (every 600ms)
        let dot_cycle = (elapsed / 600) % 4;
        let dots = ".".repeat(dot_cycle as usize);
        let title = format!("Loading{:<3}", dots); // Fixed width for stability

        // Create clean, consistent block design
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        // Create paragraph with centered message
        let paragraph = Paragraph::new(self.message.clone())
            .block(block)
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });

        // Render the base widget first
        paragraph.render(area, buf);

        // Apply post-render effects for animated borders
        // Note: Since we need &mut self, we'll apply effects in the TUI main loop
    }

    // ------------------------------------------------------------------------------
}
