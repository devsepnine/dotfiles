use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    // Split into three sections: title, options, help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Title
            Constraint::Length(10), // Options
            Constraint::Min(1),     // Help text
        ])
        .split(area);

    // Title
    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Config Installer",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Select target CLI",
            Style::default().fg(Color::White),
        )),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);

    // Options
    let options = vec![
        ListItem::new(vec![
            Line::from(Span::styled(
                "1. Claude Code",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "   Anthropic's official CLI for Claude (~/.claude)",
                Style::default().fg(Color::DarkGray),
            )),
        ]),
        Line::from("").into(),
        ListItem::new(vec![
            Line::from(Span::styled(
                "2. Codex CLI",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "   OpenAI's ChatGPT-based CLI (~/.codex)",
                Style::default().fg(Color::DarkGray),
            )),
        ]),
    ];

    let list = List::new(options)
        .block(Block::default().borders(Borders::ALL).title(" Options "));

    f.render_widget(list, chunks[1]);

    // Help text
    let help = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Press 1 or 2 to select CLI",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "Press q to quit",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title(" Help "));

    f.render_widget(help, chunks[2]);
}
