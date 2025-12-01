use pza_power_supply_client::PowerSupplyClient;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// Power Supply Instance Widget
///
/// Manages the display and interaction for a single power supply instance
pub struct PowerSupplyInstanceWidget {
    /// Name of the power supply instance
    pub instance_name: String,
    /// Power supply client for MQTT communication
    pub client: Option<PowerSupplyClient>,
    /// Current power state (output enable)
    pub power_state: bool,
    /// Current voltage value
    pub voltage: String,
    /// Current current value
    pub current: String,
    /// Status message for this instance
    pub status_message: String,
}

impl PowerSupplyInstanceWidget {
    /// Create a new power supply instance widget
    pub fn new(instance_name: String) -> Self {
        Self {
            instance_name,
            client: None,
            power_state: false,
            voltage: "0.00V".to_string(),
            current: "0.00A".to_string(),
            status_message: "Initializing...".to_string(),
        }
    }

    // ------------------------------------------------------------------------------

    /// Set the power supply client
    ///
    /// Assigns an MQTT client to this widget, establishing communication
    /// with the corresponding power supply instance. Updates the status
    /// message to indicate successful connection.
    ///
    /// # Arguments
    ///
    /// * `client` - The PowerSupplyClient for MQTT communication
    pub fn set_client(&mut self, client: PowerSupplyClient) {
        self.client = Some(client);
        self.status_message = "Connected".to_string();
    }

    // ------------------------------------------------------------------------------

    /// Update power supply state from client
    ///
    /// Retrieves the current state information (power output, voltage, current)
    /// from the power supply via MQTT and updates the widget's display values.
    /// This method is called periodically to keep the UI synchronized with
    /// the actual power supply state.
    pub async fn update_state(&mut self) {
        if let Some(ref client) = self.client {
            self.power_state = client.get_oe().await;
            self.voltage = client.get_voltage().await;
            self.current = client.get_current().await;
        }
    }

    // ------------------------------------------------------------------------------

    /// Toggle power state
    ///
    /// Toggles the power output state of this power supply instance.
    /// If currently enabled, it will be disabled, and vice versa.
    /// Updates the status message to reflect the action taken.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the power toggle command was executed successfully
    /// * `Err(Box<dyn std::error::Error>)` - If the MQTT command fails
    ///
    /// # Errors
    ///
    /// Returns an error if the client is not set or if the underlying
    /// enable_output/disable_output command fails.
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

    // ------------------------------------------------------------------------------

    /// Render the widget
    ///
    /// Draws the power supply instance widget in the specified terminal area.
    /// The widget displays the instance name, power state, voltage, current,
    /// and status in a rounded border block with aligned field labels.
    ///
    /// # Arguments
    ///
    /// * `f` - The ratatui Frame for rendering
    /// * `area` - The terminal area where the widget should be drawn
    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Create main block with instance name and rounded borders
        let main_block = Block::default()
            .title(self.instance_name.clone())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White));

        // Power state color and text
        let power_color = if self.power_state {
            Color::Green
        } else {
            Color::Red
        };
        let power_text = if self.power_state { "ON" } else { "OFF" };

        // Create content lines with FIELD_NAME: value format
        // Field names have different color from values and are aligned for consistent spacing
        let content = vec![
            Line::from(""), // Empty line for spacing
            Line::from(vec![
                Span::styled(
                    "Power  : ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    power_text,
                    Style::default()
                        .fg(power_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""), // Empty line for spacing
            Line::from(vec![
                Span::styled(
                    "Voltage: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(&self.voltage, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""), // Empty line for spacing
            Line::from(vec![
                Span::styled(
                    "Current: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(&self.current, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""), // Empty line for spacing
            Line::from(vec![
                Span::styled(
                    "Status : ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(&self.status_message, Style::default().fg(Color::Gray)),
            ]),
        ];

        // Create paragraph with content inside the main block
        let paragraph = Paragraph::new(content)
            .block(main_block)
            .alignment(ratatui::layout::Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}
