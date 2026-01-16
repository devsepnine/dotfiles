use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Progress bar
            Constraint::Min(0),     // Log
        ])
        .split(area);

    // Title with spinner animation or completion status
    let (title_text, title_color) = if app.processing_complete {
        ("✓ Complete".to_string(), Color::Green)
    } else if app.needs_refresh {
        let spinner = super::get_spinner(app.animation_frame);
        (format!("{} Refreshing status...", spinner), Color::Yellow)
    } else {
        let spinner = super::get_spinner(app.animation_frame);
        let text = if app.is_removing {
            format!("{} Removing...", spinner)
        } else {
            format!("{} Installing...", spinner)
        };
        let color = if app.is_removing { Color::Red } else { Color::Cyan };
        (text, color)
    };
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(title, chunks[0]);

    // Progress bar
    let progress = app.processing_progress.unwrap_or(0);
    let total = app.processing_total.unwrap_or(1).max(1);
    let percent = ((progress as f64 / total as f64) * 100.0) as u16;

    let gauge_color = if app.is_removing { Color::Red } else { Color::Green };
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(gauge_color))
        .percent(percent)
        .label(format!("{}/{}", progress, total));
    f.render_widget(gauge, chunks[1]);

    // Log
    let log_items: Vec<ListItem> = app
        .processing_log
        .iter()
        .map(|msg| {
            let style = if msg.starts_with("[OK]") {
                Style::default().fg(Color::Green)
            } else if msg.starts_with("[ERR]") {
                Style::default().fg(Color::Red)
            } else if msg.starts_with("[SKIP]") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(msg.clone(), style)))
        })
        .collect();

    let log_list = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title(" Log "));
    f.render_widget(log_list, chunks[2]);
}
