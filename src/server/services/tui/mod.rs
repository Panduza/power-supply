/// Terminal User Interface module
///
/// Provides a simple TUI for power supply control and monitoring.
mod loading;
mod psi_widget;

use std::any;
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
use pza_power_supply_client::PowerSupplyClient;
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

use loading::LoadingWidget;
use psi_widget::PowerSupplyInstanceWidget;
use tokio::task::JoinHandle;

/// Application state for the TUI
pub struct TuiService {
    /// Whether the application should quit
    should_quit: bool,
    /// Animation frame counter for loading widget
    animation_frame: usize,
    //     /// Power supply instance widgets
    //     widgets: Vec<PowerSupplyInstanceWidget>,
    //     /// Currently selected widget index
    //     selected_widget: usize,
    //     /// TUI configuration
    //     tui_config: TuiConfig,
}

impl TuiService {
    /// Starts the TUI service in a separate task
    pub fn start() -> JoinHandle<Result<(), anyhow::Error>> {
        println!("Starting TUI service...");
        let handle = tokio::spawn(Self::render_loop());

        handle
    }

    // ------------------------------------------------------------------------------

    /// Handle keyboard input events
    ///
    /// Processes keyboard input and updates application state accordingly.
    /// Supports navigation between widgets and power toggle commands.
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard key that was pressed
    ///
    /// # Supported Keys
    ///
    /// * `q` or `Esc` - Quit the application
    /// * `Space` or `Enter` - Toggle power state (default keys)
    /// * Configured power toggle key - Toggle power state (from TUI config)
    /// * `Up Arrow` - Navigate to previous widget
    /// * `Down Arrow` - Navigate to next widget
    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                println!("Quitting TUI...");
                self.should_quit = true;
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------------------

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    async fn render_loop() -> anyhow::Result<()> {
        let mut app = TuiService {
            should_quit: false,
            animation_frame: 0,
        };

        // Setup terminal
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Min(8),    // Main content
                        Constraint::Length(3), // Help bar
                    ])
                    .split(f.area());

                // Render animated loading widget
                let loading_widget =
                    LoadingWidget::new("Please wait, TUI is initializing...", app.animation_frame);
                f.render_widget(loading_widget, chunks[0]);
            })?;

            // Handle events
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        _ => {
                            app.handle_input(key.code);
                        }
                    }
                }
            }

            if app.should_quit() {
                break;
            }

            // Increment animation frame for border animation
            app.animation_frame = app.animation_frame.wrapping_add(1);
        }

        println!("Exiting TUI service...");

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
}
//     /// Create a new application instance with power supply instances
//     pub fn new(instance_names: Vec<String>, tui_config: TuiConfig) -> Self {
//         let widgets = instance_names
//             .into_iter()
//             .map(PowerSupplyInstanceWidget::new)
//             .collect();

//         Self {
//             should_quit: false,
//             widgets,
//             selected_widget: 0,
//             tui_config,
//         }
//     }

//     // ------------------------------------------------------------------------------

//     /// Initialize clients for all widgets
//     ///
//     /// Creates and assigns MQTT clients to each power supply instance widget.
//     /// This establishes the communication channel between the TUI and the power supplies.
//     ///
//     /// # Returns
//     ///
//     /// * `Ok(())` - If all clients are successfully initialized
//     /// * `Err(Box<dyn std::error::Error>)` - If any client fails to connect
//     ///
//     /// # Errors
//     ///
//     /// Returns an error if the PowerSupplyClient cannot be built for any instance,
//     /// typically due to MQTT connection issues.
//     pub async fn initialize_clients(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         for widget in &mut self.widgets {
//             match PowerSupplyClient::builder()
//                 .with_power_supply_name(widget.instance_name.clone())
//                 .build()
//             {
//                 Ok(client) => {
//                     widget.set_client(client);
//                 }
//                 Err(e) => {
//                     return Err(format!(
//                         "Failed to connect to power supply instance '{}': {}",
//                         widget.instance_name, e
//                     )
//                     .into());
//                 }
//             }
//         }
//         Ok(())
//     }

//     // ------------------------------------------------------------------------------

//     /// Update state for all widgets
//     ///
//     /// Refreshes the current state (power, voltage, current) for all power supply
//     /// instance widgets by querying their respective MQTT clients.
//     /// This method is called periodically to keep the display up to date.
//     pub async fn update_state(&mut self) {
//         for widget in &mut self.widgets {
//             widget.update_state().await;
//         }
//     }

//     // ------------------------------------------------------------------------------

//     /// Toggle power state of selected widget
//     ///
//     /// Toggles the power output state of the currently selected power supply instance.
//     /// Updates the status message to reflect the action taken.
//     ///
//     /// # Returns
//     ///
//     /// * `Ok(())` - If the power toggle command was sent successfully
//     /// * `Err(Box<dyn std::error::Error>)` - If the MQTT command fails
//     ///
//     /// # Errors
//     ///
//     /// Returns an error if the underlying power supply client cannot execute
//     /// the enable/disable output command.
//     pub async fn toggle_power(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         if let Some(widget) = self.widgets.get_mut(self.selected_widget) {
//             widget.toggle_power().await?;
//         }
//         Ok(())
//     }
// }

// /// Run the TUI application
// ///
// /// This function initializes and runs the terminal user interface for power supply control.
// /// It waits for the server state to be ready, creates widgets for all available power supply
// /// instances, and handles user input for controlling the power supplies.
// ///
// /// # Arguments
// ///
// /// * `_instance_name` - Optional instance name (currently unused, reserved for future use)
// ///
// /// # Returns
// ///
// /// * `Ok(())` - When the TUI exits normally
// /// * `Err(Box<dyn std::error::Error>)` - If there's an error initializing or running the TUI
// ///
// /// # Errors
// ///
// /// This function can return errors for:
// /// - Terminal setup failures
// /// - Server state initialization issues
// /// - MQTT client connection problems
// pub async fn run_tui(_instance_name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
//     // Get server state reference
//     let server_state = SERVER_STATE_STORAGE
//         .get()
//         .ok_or("Server state not initialized")?;

//     // Wait for state ready signal while showing loading screen
//     let mut ready_receiver = server_state.ready_receiver();
//     loop {
//         // Draw loading screen
//         terminal.draw(|f| {
//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .margin(1)
//                 .constraints([
//                     Constraint::Min(8),    // Main content
//                     Constraint::Length(3), // Help bar
//                 ])
//                 .split(f.area());

//             // Loading message
//             let loading_block = Block::default()
//                 .borders(Borders::ALL)
//                 .border_type(BorderType::Rounded)
//                 .style(Style::default().fg(Color::Cyan))
//                 .title("Loading");

//             let message = vec![
//                 Line::from(""),
//                 Line::from(Span::styled(
//                     "Please wait while the server initializes...",
//                     Style::default()
//                         .fg(Color::Cyan)
//                         .add_modifier(Modifier::BOLD),
//                 )),
//                 Line::from(""),
//                 Line::from("Starting services and connecting to devices..."),
//             ];

//             let loading_paragraph = Paragraph::new(message)
//                 .block(loading_block)
//                 .alignment(Alignment::Center);
//             f.render_widget(loading_paragraph, chunks[0]);

//             // Help bar
//             let help_block = Block::default()
//                 .borders(Borders::ALL)
//                 .style(Style::default().fg(Color::White))
//                 .title("Help");
//             let help_paragraph = Paragraph::new("q/Esc: Quit").block(help_block);
//             f.render_widget(help_paragraph, chunks[1]);
//         })?;

//         // Check for quit input during loading
//         if event::poll(Duration::from_millis(50))? {
//             if let Event::Key(key) = event::read()? {
//                 match key.code {
//                     KeyCode::Char('q') | KeyCode::Esc => {
//                         // Restore terminal
//                         disable_raw_mode()?;
//                         execute!(
//                             terminal.backend_mut(),
//                             LeaveAlternateScreen,
//                             DisableMouseCapture
//                         )?;
//                         terminal.show_cursor()?;
//                         return Ok(());
//                     }
//                     _ => {}
//                 }
//             }
//         }

//         // Check if ready signal has been received
//         if *ready_receiver.borrow() {
//             break;
//         }

//         // Wait for signal change
//         tokio::select! {
//             _ = ready_receiver.changed() => {
//                 if *ready_receiver.borrow() {
//                     break;
//                 }
//             }
//             _ = tokio::time::sleep(Duration::from_millis(100)) => {
//                 // Continue showing loading screen
//             }
//         }
//     }

//     // Now get available instances after state is ready
//     let available_instances = server_state.instances_names().await;

//     // Get TUI configuration from server state
//     let tui_config = {
//         let config_guard = server_state.server_config.lock().await;
//         config_guard.tui.clone()
//     };

//     // Create app state with all available instances and TUI config
//     let mut app = App::new(available_instances.clone(), tui_config);

//     // Initialize clients for all instances (only if instances exist)
//     if !available_instances.is_empty() {
//         app.initialize_clients().await?;
//     }

//     let mut last_update = std::time::Instant::now();
//     let mut toggle_requested = false;

//     // Main event loop
//     loop {
//         // Update state every 500ms
//         if last_update.elapsed() > Duration::from_millis(500) {
//             app.update_state().await;
//             last_update = std::time::Instant::now();
//         }

//         // Handle toggle request
//         if toggle_requested {
//             let _ = app.toggle_power().await;
//             toggle_requested = false;
//         }

//         terminal.draw(|f| {
//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .margin(1)
//                 .constraints([
//                     Constraint::Min(8),    // Main content
//                     Constraint::Length(3), // Help bar
//                 ])
//                 .split(f.area());

//             // Main content area
//             if app.widgets.is_empty() {
//                 // Display "no instances available" message
//                 let no_instances_block = Block::default()
//                     .borders(Borders::ALL)
//                     .border_type(BorderType::Rounded)
//                     .style(Style::default().fg(Color::Yellow))
//                     .title("No Instances Available");

//                 let message = vec![
//                     Line::from(""),
//                     Line::from(Span::styled(
//                         "No power supply instances are configured.",
//                         Style::default()
//                             .fg(Color::Yellow)
//                             .add_modifier(Modifier::BOLD),
//                     )),
//                     Line::from(""),
//                     Line::from("Please configure at least one device in the server"),
//                     Line::from("configuration file to use the TUI."),
//                 ];

//                 let message_paragraph = Paragraph::new(message)
//                     .block(no_instances_block)
//                     .alignment(Alignment::Center);
//                 f.render_widget(message_paragraph, chunks[0]);
//             } else {
//                 // Display all power supply widgets
//                 let widget_count = app.widgets.len();
//                 let constraints: Vec<Constraint> = (0..widget_count)
//                     .map(|_| Constraint::Percentage(100 / widget_count as u16))
//                     .collect();

//                 let widget_chunks = Layout::default()
//                     .direction(Direction::Horizontal)
//                     .constraints(constraints)
//                     .split(chunks[0]);

//                 // Render each widget
//                 for (i, widget) in app.widgets.iter().enumerate() {
//                     if let Some(area) = widget_chunks.get(i) {
//                         // Highlight the selected widget
//                         let mut area_to_use = *area;
//                         if i == app.selected_widget {
//                             // Add highlighting for selected widget
//                             let highlight_block = Block::default()
//                                 .borders(Borders::ALL)
//                                 .border_type(BorderType::Thick)
//                                 .style(Style::default().fg(Color::Magenta));
//                             area_to_use = highlight_block.inner(area_to_use);
//                             f.render_widget(highlight_block, *area);
//                         }
//                         widget.render(f, area_to_use);
//                     }
//                 }
//             }

//             // Help bar
//             let help_block = Block::default()
//                 .borders(Borders::ALL)
//                 .style(Style::default().fg(Color::White))
//                 .title("Help");
//             let help_text = if app.widgets.is_empty() {
//                 "q/Esc: Quit"
//             } else {
//                 "q/Esc: Quit | ↑/↓: Navigate | Space/Enter: Toggle Power"
//             };
//             let help_paragraph = Paragraph::new(help_text).block(help_block);
//             f.render_widget(help_paragraph, chunks[1]);
//         })?;

//         // Handle events
//         if event::poll(Duration::from_millis(50))? {
//             if let Event::Key(key) = event::read()? {
//                 match key.code {
//                     KeyCode::Char(' ') | KeyCode::Enter => {
//                         toggle_requested = true;
//                     }
//                     _ => {
//                         app.handle_input(key.code);
//                     }
//                 }
//             }
//         }

//         if app.should_quit() {
//             break;
//         }
//     }

//     // Restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     Ok(())
// }
