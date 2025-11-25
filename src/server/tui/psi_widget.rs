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

use pza_power_supply_client::PowerSupplyClient;

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
    pub fn set_client(&mut self, client: PowerSupplyClient) {
        self.client = Some(client);
        self.status_message = "Connected".to_string();
    }

    // ------------------------------------------------------------------------------

    /// Update power supply state from client
    pub async fn update_state(&mut self) {
        if let Some(ref client) = self.client {
            self.power_state = client.get_oe().await;
            self.voltage = client.get_voltage().await;
            self.current = client.get_current().await;
        }
    }

    // ------------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------------

    /// Render the widget
    pub fn render(&self, f: &mut Frame, area: Rect) {
        // Create main block with instance name and rounded borders
        let main_block = Block::default()
            .title(self.instance_name.clone())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White));

        // Split the area into sections for power, voltage, current, and status
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .margin(1)
            .constraints([
                ratatui::layout::Constraint::Length(3), // Power state
                ratatui::layout::Constraint::Length(3), // Voltage
                ratatui::layout::Constraint::Length(3), // Current
                ratatui::layout::Constraint::Min(1),    // Status
            ])
            .split(main_block.inner(area));

        // Render the main block
        f.render_widget(main_block, area);

        // Power state display
        let power_color = if self.power_state {
            Color::Green
        } else {
            Color::Red
        };
        let power_text = if self.power_state { "ON" } else { "OFF" };
        let power_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(power_color))
            .title("Power");
        let power_content = vec![Line::from(vec![Span::styled(
            power_text,
            Style::default()
                .fg(power_color)
                .add_modifier(Modifier::BOLD),
        )])];
        let power_paragraph = Paragraph::new(power_content)
            .block(power_block)
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(power_paragraph, chunks[0]);

        // Voltage display
        let voltage_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .title("Voltage");
        let voltage_content = vec![Line::from(vec![Span::styled(
            &self.voltage,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )])];
        let voltage_paragraph = Paragraph::new(voltage_content)
            .block(voltage_block)
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(voltage_paragraph, chunks[1]);

        // Current display
        let current_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan))
            .title("Current");
        let current_content = vec![Line::from(vec![Span::styled(
            &self.current,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )])];
        let current_paragraph = Paragraph::new(current_content)
            .block(current_block)
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(current_paragraph, chunks[2]);

        // Status display
        let status_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Status");
        let status_paragraph = Paragraph::new(self.status_message.as_str()).block(status_block);
        f.render_widget(status_paragraph, chunks[3]);
    }
}
