use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{App, Tab};
use crate::component::InstallStatus;
use crate::tree::TreeNode;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Use tree-based rendering for component tabs
    if let Some(tree) = app.get_tree_view() {
        render_tree(f, app, tree, area);
    } else {
        // Fallback to flat list (shouldn't happen for component tabs)
        render_flat(f, app, area);
    }
}

fn render_tree(f: &mut Frame, app: &App, tree: &crate::tree::TreeView, area: Rect) {
    let items: Vec<ListItem> = tree.visible_indices
        .iter()
        .map(|&node_idx| {
            let node = &tree.nodes[node_idx];
            render_tree_node(app, tree, node, node_idx)
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
    if !tree.visible_indices.is_empty() {
        state.select(Some(tree.cursor));
    }

    f.render_stateful_widget(list, area, &mut state);
}

fn render_tree_node(app: &App, tree: &crate::tree::TreeView, node: &TreeNode, node_idx: usize) -> ListItem<'static> {
    let depth = node.depth();
    let indent = "  ".repeat(depth);

    match node {
        TreeNode::Folder { name, expanded, .. } => {
            // Folder icon
            let icon = if *expanded { "v " } else { "> " };

            // Check folder selection state
            let (checkbox, checkbox_style) = if tree.is_folder_all_selected(node_idx, &app.components) {
                ("[x]", Style::default().fg(Color::Green))
            } else if tree.is_folder_any_selected(node_idx, &app.components) {
                ("[-]", Style::default().fg(Color::Yellow))
            } else {
                ("[ ]", Style::default().fg(Color::DarkGray))
            };

            let line = Line::from(vec![
                Span::raw(format!("{}{}", indent, checkbox)),
                Span::styled(" ", checkbox_style),
                Span::styled(
                    icon,
                    Style::default().fg(Color::Blue),
                ),
                Span::styled(
                    format!("{}/", name),
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                ),
            ]);

            ListItem::new(line)
        }
        TreeNode::File { component_idx, .. } => {
            let c = &app.components[*component_idx];
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

            let default_marker = if is_default { " *" } else { "" };

            // Extract just the filename (last part of path)
            let filename = c.name.rsplit('/').next().unwrap_or(&c.name);

            let line = Line::from(vec![
                Span::raw(format!("{}{} ", indent, checkbox)),
                Span::styled(
                    format!("{:<36}", filename),
                    Style::default().fg(Color::White),
                ),
                Span::styled(format!("({:^9})", c.status.display()), status_style),
                Span::styled(default_marker, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]);

            ListItem::new(line)
        }
    }
}

fn render_flat(f: &mut Frame, app: &App, area: Rect) {
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

            let default_marker = if is_default { " *" } else { "" };

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
