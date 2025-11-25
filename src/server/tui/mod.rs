/// Terminal User Interface module
///
/// Provides a simple TUI for power supply control and monitoring.
mod psi_widget;

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
use pza_power_supply_client::PowerSupplyClientBuilder;
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
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

use super::SERVER_STATE_STORAGE;
use psi_widget::PowerSupplyInstanceWidget;

/// Application state for the TUI
pub struct App {
    /// Whether the application should quit
    should_quit: bool,
    /// Power supply instance widgets
    widgets: Vec<PowerSupplyInstanceWidget>,
    /// Currently selected widget index
    selected_widget: usize,
    /// Global status message
    status_message: String,
}

impl App {
    /// Create a new application instance with power supply instances
    pub fn new(instance_names: Vec<String>) -> Self {
        let widgets = instance_names
            .into_iter()
            .map(PowerSupplyInstanceWidget::new)
            .collect();

        Self {
            should_quit: false,
            widgets,
            selected_widget: 0,
            status_message: "Initializing...".to_string(),
        }
    }

    // ------------------------------------------------------------------------------

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    // ------------------------------------------------------------------------------

    /// Handle keyboard input events
    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Toggle power state of selected widget
                if !self.widgets.is_empty() {
                    self.status_message = "Toggling power...".to_string();
                }
            }
            KeyCode::Up => {
                if self.selected_widget > 0 {
                    self.selected_widget -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_widget < self.widgets.len().saturating_sub(1) {
                    self.selected_widget += 1;
                }
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------------------

    /// Initialize clients for all widgets
    pub async fn initialize_clients(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for widget in &mut self.widgets {
            match PowerSupplyClientBuilder::default()
                .with_power_supply_name(widget.instance_name.clone())
                .build()
            {
                Ok(client) => {
                    widget.set_client(client);
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to connect to power supply instance '{}': {}",
                        widget.instance_name, e
                    )
                    .into());
                }
            }
        }
        self.status_message = "Connected to all power supplies".to_string();
        Ok(())
    }

    // ------------------------------------------------------------------------------

    /// Update state for all widgets
    pub async fn update_state(&mut self) {
        for widget in &mut self.widgets {
            widget.update_state().await;
        }
    }

    // ------------------------------------------------------------------------------

    /// Toggle power state of selected widget
    pub async fn toggle_power(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(widget) = self.widgets.get_mut(self.selected_widget) {
            widget.toggle_power().await?;
            self.status_message = format!("Toggled power for {}", widget.instance_name);
        }
        Ok(())
    }
}

/// Run the TUI application
pub async fn run_tui(_instance_name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Get available instances from server state
    let server_state = SERVER_STATE_STORAGE
        .get()
        .ok_or("Server state not initialized")?;

    let available_instances = server_state.instances_names().await;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with all available instances
    let mut app = App::new(available_instances.clone());

    // Initialize clients for all instances (only if instances exist)
    if !available_instances.is_empty() {
        app.initialize_clients().await?;
    }

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
                    Constraint::Length(3), // Title bar
                    Constraint::Min(8),    // Main content
                    Constraint::Length(3), // Status bar
                    Constraint::Length(3), // Help bar
                ])
                .split(f.area());

            // Title bar
            let title_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Panduza Power Supply Controller");
            let title_text = if app.widgets.is_empty() {
                "No Power Supply Instances Available"
            } else {
                "All Power Supply Instances"
            };
            let title_paragraph = Paragraph::new(title_text)
                .block(title_block)
                .alignment(Alignment::Center);
            f.render_widget(title_paragraph, chunks[0]);

            // Main content area
            if app.widgets.is_empty() {
                // Display "no instances available" message
                let no_instances_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Yellow))
                    .title("No Instances Available");

                let message = vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "No power supply instances are configured.",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from("Please configure at least one device in the server"),
                    Line::from("configuration file to use the TUI."),
                ];

                let message_paragraph = Paragraph::new(message)
                    .block(no_instances_block)
                    .alignment(Alignment::Center);
                f.render_widget(message_paragraph, chunks[1]);
            } else {
                // Display all power supply widgets
                let widget_count = app.widgets.len();
                let constraints: Vec<Constraint> = (0..widget_count)
                    .map(|_| Constraint::Percentage(100 / widget_count as u16))
                    .collect();

                let widget_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(chunks[1]);

                // Render each widget
                for (i, widget) in app.widgets.iter().enumerate() {
                    if let Some(area) = widget_chunks.get(i) {
                        // Highlight the selected widget
                        let mut area_to_use = *area;
                        if i == app.selected_widget {
                            // Add highlighting for selected widget
                            let highlight_block = Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Thick)
                                .style(Style::default().fg(Color::Magenta));
                            area_to_use = highlight_block.inner(area_to_use);
                            f.render_widget(highlight_block, *area);
                        }
                        widget.render(f, area_to_use);
                    }
                }
            }

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
            let help_text = if app.widgets.is_empty() {
                "q/Esc: Quit"
            } else {
                "q/Esc: Quit | ↑/↓: Navigate | Space/Enter: Toggle Power"
            };
            let help_paragraph = Paragraph::new(help_text).block(help_block);
            f.render_widget(help_paragraph, chunks[3]);
        })?;

        // Handle events
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        toggle_requested = true;
                    }
                    _ => {
                        app.handle_input(key.code);
                    }
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
