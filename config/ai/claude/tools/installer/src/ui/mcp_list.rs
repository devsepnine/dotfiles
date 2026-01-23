use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;
use crate::mcp::McpStatus;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if app.mcp_servers.is_empty() {
        let message = "No MCP servers found. Create mcps/mcps.yaml to add servers.";

        let empty = List::new(vec![ListItem::new(Line::from(vec![Span::styled(
            message,
            Style::default().fg(Color::Gray),
        )]))])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" MCP Servers "),
        );
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .mcp_servers
        .iter()
        .map(|m| {
            let checkbox = if m.selected {
                "[x]"
            } else if m.status == McpStatus::Installed {
                "[*]"
            } else {
                "[ ]"
            };

            let status_style = match m.status {
                McpStatus::Installed => Style::default().fg(Color::Green),
                McpStatus::NotInstalled => Style::default().fg(Color::Gray),
            };

            // First line: checkbox, name, status, category, env warning
            let line1 = Line::from(vec![
                Span::raw(format!("{} ", checkbox)),
                Span::styled(
                    format!("{:<24}", m.def.name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("({:^13})", m.status.display()),
                    status_style,
                ),
                Span::styled(
                    format!(" [{}]", m.def.category),
                    Style::default().fg(Color::Blue),
                ),
                if !m.def.env.is_empty() {
                    Span::styled(" ⚠ env", Style::default().fg(Color::Yellow))
                } else {
                    Span::raw("")
                },
            ]);

            // Second line: description (indented)
            let line2 = Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    m.def.description.clone(),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(vec![line1, line2])
        })
        .collect();

    let title = format!(" MCP Servers (scope: {}) ", app.mcp_scope.display());
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
    state.select(Some(app.mcp_index));

    f.render_stateful_widget(list, area, &mut state);
}
