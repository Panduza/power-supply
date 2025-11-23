/// Terminal User Interface module
///
/// Provides a simple TUI for power supply control and monitoring.
use std::io;
use std::sync::Arc;
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
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use tokio::sync::broadcast;
use tokio::sync::Mutex;

use crate::PowerSupplyClient;
use crate::PowerSupplyClientBuilder;

/// Application state for the TUI
pub struct App {
    /// Whether the application should quit
    should_quit: bool,
    /// Current instance name being controlled
    instance_name: Option<String>,
    /// Power supply client for MQTT communication
    client: Option<PowerSupplyClient>,
    /// Current power state (output enable)
    power_state: bool,
    /// Current voltage value
    voltage: String,
    /// Current current value
    current: String,
    /// Status message
    status_message: String,
}

impl App {
    /// Create a new application instance
    pub fn new(instance_name: Option<String>) -> Self {
        Self {
            should_quit: false,
            instance_name,
            client: None,
            power_state: false,
            voltage: "0.00V".to_string(),
            current: "0.00A".to_string(),
            status_message: "Initializing...".to_string(),
        }
    }

    // ----------------------------------------------------------------------------

    /// Handle keyboard input events
    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Toggle power state on space or enter
                if self.client.is_some() {
                    self.status_message = "Toggling power...".to_string();
                }
            }
            _ => {}
        }
    }

    // ----------------------------------------------------------------------------

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    // ----------------------------------------------------------------------------

    /// Initialize the PowerSupply client
    pub async fn initialize_client(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref instance_name) = self.instance_name {
            match PowerSupplyClientBuilder::default()
                .psu_name(instance_name.clone())
                .build()
                .await
            {
                Ok(client) => {
                    self.client = Some(client);
                    self.status_message = "Connected to power supply".to_string();
                }
                Err(e) => {
                    self.status_message = format!("Failed to connect: {}", e);
                }
            }
        } else {
            self.status_message = "No instance specified".to_string();
        }
        Ok(())
    }

    // ----------------------------------------------------------------------------

    /// Update power supply state from client
    pub async fn update_state(&mut self) {
        if let Some(ref client) = self.client {
            self.power_state = client.get_oe().await;
            self.voltage = client.get_voltage().await;
            self.current = client.get_current().await;
        }
    }

    // ----------------------------------------------------------------------------

    /// Toggle power state
    pub async fn toggle_power(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref client) = self.client {
            if self.power_state {
                client.disable_output().await?;
                self.status_message = "Power disabled".to_string();
            } else {
                client.enable_output().await?;
                self.status_message = "Power enabled".to_string();
            }
        }
        Ok(())
    }
}

/// Run the TUI application
pub async fn run_tui(instance_name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(instance_name);

    // Initialize client if instance name is provided
    app.initialize_client().await?;

    let mut last_update = std::time::Instant::now();
    let mut toggle_requested = false;

    // Main event loop
    loop {
        // Update state every 500ms
        if last_update.elapsed() > Duration::from_millis(500) {
            app.update_state().await;
            last_update = std::time::Instant::now();
        }

        // Handle toggle request
        if toggle_requested {
            if let Err(e) = app.toggle_power().await {
                app.status_message = format!("Error toggling power: {}", e);
            }
            toggle_requested = false;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(8),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Title bar
            let title = match &app.instance_name {
                Some(name) => format!("Power Supply TUI - Instance: {}", name),
                None => "Power Supply TUI - No Instance Selected".to_string(),
            };

            let title_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Panduza Power Supply Controller");
            let title_paragraph = Paragraph::new(title)
                .block(title_block)
                .alignment(Alignment::Center);
            f.render_widget(title_paragraph, chunks[0]);

            // Main control panel
            let control_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ])
                .split(chunks[1]);

            // Power state display
            let power_color = if app.power_state {
                Color::Green
            } else {
                Color::Red
            };
            let power_text = if app.power_state { "ON" } else { "OFF" };
            let power_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(power_color))
                .title("Power State");
            let power_content = vec![
                Line::from(vec![Span::styled(
                    power_text,
                    Style::default()
                        .fg(power_color)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                )]),
                Line::from(""),
                Line::from("Press SPACE or"),
                Line::from("ENTER to toggle"),
            ];
            let power_paragraph = Paragraph::new(power_content)
                .block(power_block)
                .alignment(Alignment::Center);
            f.render_widget(power_paragraph, control_chunks[0]);

            // Voltage display
            let voltage_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
                .title("Voltage");
            let voltage_content = vec![Line::from(vec![Span::styled(
                &app.voltage,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )])];
            let voltage_paragraph = Paragraph::new(voltage_content)
                .block(voltage_block)
                .alignment(Alignment::Center);
            f.render_widget(voltage_paragraph, control_chunks[1]);

            // Current display
            let current_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan))
                .title("Current");
            let current_content = vec![Line::from(vec![Span::styled(
                &app.current,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )])];
            let current_paragraph = Paragraph::new(current_content)
                .block(current_block)
                .alignment(Alignment::Center);
            f.render_widget(current_paragraph, control_chunks[2]);

            // Status bar
            let status_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Status");
            let status_paragraph = Paragraph::new(app.status_message.as_str()).block(status_block);
            f.render_widget(status_paragraph, chunks[2]);

            // Help bar
            let help_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Help");
            let help_paragraph =
                Paragraph::new("q/Esc: Quit | Space/Enter: Toggle Power").block(help_block);
            f.render_widget(help_paragraph, chunks[3]);
        })?;

        // Handle events
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        toggle_requested = true;
                    }
                    _ => {}
                }
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
