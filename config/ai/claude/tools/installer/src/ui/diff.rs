use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let content = app.diff_content.as_deref().unwrap_or("No diff available");

    let lines: Vec<Line> = content
        .lines()
        .map(|line| {
            let style = if line.starts_with('+') && !line.starts_with("+++") {
                Style::default().fg(Color::Green)
            } else if line.starts_with('-') && !line.starts_with("---") {
                Style::default().fg(Color::Red)
            } else if line.starts_with("@@") {
                Style::default().fg(Color::Cyan)
            } else if line.starts_with("---") || line.starts_with("+++") {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            Line::from(Span::styled(line, style))
        })
        .collect();

    let title = if let Some(idx) = app.selected_component_index() {
        if let Some(c) = app.components.get(idx) {
            format!(" Diff: {} ", c.display_name())
        } else {
            " Diff ".to_string()
        }
    } else {
        " Diff ".to_string()
    };

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .scroll((app.diff_scroll, 0));

    f.render_widget(paragraph, area);
}
