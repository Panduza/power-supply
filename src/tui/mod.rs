use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Spans};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::{stdout, Stdout};
use std::time::Duration;

fn draw_ui<B: ratatui::backend::Backend>(f: &mut ratatui::frame::Frame<B>) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    // Power state
    let power = Paragraph::new(Spans::from(vec![Span::styled(
        "Power: OFF",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )]))
    .block(Block::default().borders(Borders::ALL).title("Power"));
    f.render_widget(power, chunks[0]);

    // Voltage
    let voltage = Paragraph::new(Spans::from(vec![Span::styled(
        "Voltage: 0.00 V",
        Style::default().fg(Color::Yellow),
    )]))
    .block(Block::default().borders(Borders::ALL).title("Voltage"));
    f.render_widget(voltage, chunks[1]);

    // Current
    let current = Paragraph::new(Spans::from(vec![Span::styled(
        "Current: 0.00 A",
        Style::default().fg(Color::Yellow),
    )]))
    .block(Block::default().borders(Borders::ALL).title("Current"));
    f.render_widget(current, chunks[2]);
}

pub fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial draw
    terminal.draw(|f| draw_ui(f))?;

    // Event loop: wait for 'q' to quit, or redraw on any key; timeout to allow redraw
    loop {
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {
                        terminal.draw(|f| draw_ui(f))?;
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    let mut stdout = terminal.backend_mut();
    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
