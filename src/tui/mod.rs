/// Terminal User Interface module
///
/// Provides a simple TUI for power supply control and monitoring.
use std::io;
use std::time::Duration;

use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

/// Application state for the TUI
pub struct App {
    /// Whether the application should quit
    should_quit: bool,
    /// Current instance name being controlled
    instance_name: Option<String>,
}

impl App {
    /// Create a new application instance
    pub fn new(instance_name: Option<String>) -> Self {
        Self {
            should_quit: false,
            instance_name,
        }
    }

    // ----------------------------------------------------------------------------

    /// Handle keyboard input events
    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    // ----------------------------------------------------------------------------

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

/// Run the TUI application
pub fn run_tui(instance_name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(instance_name);

    // Main event loop
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let title = match &app.instance_name {
                Some(name) => format!("Power Supply TUI - Instance: {}", name),
                None => "Power Supply TUI - No Instance Selected".to_string(),
            };

            let title_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(title);
            let title_paragraph =
                Paragraph::new("Welcome to the Power Supply TUI").block(title_block);
            f.render_widget(title_paragraph, chunks[0]);

            let main_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Control Panel");
            let main_content =
                Paragraph::new("Controls will be implemented here.\n\nPress 'q' or 'Esc' to quit.")
                    .block(main_block);
            f.render_widget(main_content, chunks[1]);

            let help_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Help");
            let help_paragraph = Paragraph::new("q/Esc: Quit").block(help_block);
            f.render_widget(help_paragraph, chunks[2]);
        })?;

        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_input(key.code);
            }
        }

        if app.should_quit() {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
