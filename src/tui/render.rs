//! Rendering logic for TUI widgets and components
//!
//! Handles frame drawing, widget creation, and visual formatting.

use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use super::{layout::LayoutManager, state::TuiState};

/// Draw the main TUI frame
pub fn draw_frame<B: Backend>(f: &mut Frame<B>, state: &TuiState) {
    let area = f.size();

    // Check if terminal is large enough
    if !LayoutManager::validate_terminal_size(area) {
        draw_warning_message(f, area);
        return;
    }

    // Draw main control box
    draw_control_box(f, area, state);

    // Draw help overlay if requested
    if state.is_help_visible() {
        draw_help_overlay(f, area);
    }
}

/// Draw the main control box with power supply data
fn draw_control_box<B: Backend>(f: &mut Frame<B>, area: Rect, state: &TuiState) {
    let chunks = LayoutManager::control_box_layout(area);

    // Power state
    let power_style = if state.power_on() {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    };
    
    let power_text = if state.power_on() { "ON" } else { "OFF" };
    let power = Paragraph::new(Spans::from(vec![Span::styled(
        format!("Power: {}", power_text),
        power_style,
    )]))
    .block(Block::default().borders(Borders::ALL).title("Power Status"));
    f.render_widget(power, chunks[0]);

    // Voltage
    let voltage = Paragraph::new(Spans::from(vec![Span::styled(
        format!("Voltage: {:.2} V", state.voltage()),
        Style::default().fg(Color::Yellow),
    )]))
    .block(Block::default().borders(Borders::ALL).title("Voltage"));
    f.render_widget(voltage, chunks[1]);

    // Current
    let current = Paragraph::new(Spans::from(vec![Span::styled(
        format!("Current: {:.3} A", state.current()),
        Style::default().fg(Color::Cyan),
    )]))
    .block(Block::default().borders(Borders::ALL).title("Current"));
    f.render_widget(current, chunks[2]);
}

/// Draw warning message for undersized terminals
fn draw_warning_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let warning_area = LayoutManager::warning_layout(area);
    
    let warning = Paragraph::new(Spans::from(vec![
        Span::styled(
            format!(
                "Terminal too small! Need {}x{}, got {}x{}",
                super::layout::MIN_WIDTH,
                super::layout::MIN_HEIGHT,
                area.width,
                area.height
            ),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Warning"))
    .style(Style::default().bg(Color::Black));
    
    f.render_widget(warning, warning_area);
}

/// Draw help overlay with keyboard shortcuts
fn draw_help_overlay<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let help_area = LayoutManager::help_overlay_layout(area);
    
    // Clear the background
    f.render_widget(Clear, help_area);
    
    let help_text = vec![
        Spans::from("Keyboard Shortcuts:"),
        Spans::from(""),
        Spans::from("Space  - Toggle Power"),
        Spans::from("?      - Show/Hide Help"),
        Spans::from("q/Esc  - Exit"),
    ];
    
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().bg(Color::Blue).fg(Color::White));
    
    f.render_widget(help, help_area);
}