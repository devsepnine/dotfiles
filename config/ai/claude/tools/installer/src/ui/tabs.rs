use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs as RataTabs},
    Frame,
};

use crate::app::{App, Tab};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Build tab titles with number prefix: "1:Agents", "2:Commands", etc.
    let titles: Vec<String> = Tab::all()
        .iter()
        .enumerate()
        .map(|(i, tab)| format!("{}:{}", i + 1, tab.display_name()))
        .collect();

    let tabs = RataTabs::new(titles.into_iter().map(Line::from).collect::<Vec<_>>())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Claude Config Installer "),
        )
        .select(app.tab.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}
