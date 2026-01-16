use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, Tab};
use crate::component::InstallStatus;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let filtered = app.current_components();

    let items: Vec<ListItem> = filtered
        .iter()
        .map(|(_, c)| {
            let checkbox = if c.selected { "[x]" } else { "[ ]" };

            let status_style = match c.status {
                InstallStatus::New => Style::default().fg(Color::Green),
                InstallStatus::Modified => Style::default().fg(Color::Yellow),
                InstallStatus::Unchanged => Style::default().fg(Color::Gray),
                InstallStatus::Managed => Style::default().fg(Color::Cyan),
            };

            // Check if this is the default item
            let is_default = match app.tab {
                Tab::OutputStyles => {
                    // Compare style name without extension
                    let style_name = c.name.strip_suffix(".md").unwrap_or(&c.name);
                    app.current_output_style.as_ref().map(|s| s.as_str()) == Some(style_name)
                }
                Tab::Statusline => {
                    app.current_statusline.as_ref().map(|s| s.as_str()) == Some(&c.name)
                }
                _ => false,
            };

            let default_marker = if is_default { " ★" } else { "" };

            let line = Line::from(vec![
                Span::raw(format!("{} ", checkbox)),
                Span::styled(
                    format!("{:<40}", c.name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(format!("({:^9})", c.status.display()), status_style),
                Span::styled(default_marker, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]);

            ListItem::new(line)
        })
        .collect();

    // Add hint for setting defaults when no default is set
    let mut title = format!(" {} ", app.tab.display_name());
    if app.tab == Tab::OutputStyles && app.current_output_style.is_none() {
        title = format!("{} [No default - press 's' to set] ", title.trim());
    } else if app.tab == Tab::Statusline && app.current_statusline.is_none() {
        title = format!("{} [No default - press 's' to set] ", title.trim());
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    if !filtered.is_empty() {
        state.select(Some(app.list_index));
    }

    f.render_stateful_widget(list, area, &mut state);
}
