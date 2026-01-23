mod tabs;
pub mod list;
mod mcp_list;
mod plugin_list;
mod diff;
mod env_input;
mod project_path;
mod installing;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::{App, Tab, View};

// Spinner animation frames
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn get_spinner(frame: usize) -> &'static str {
    SPINNER_FRAMES[frame % SPINNER_FRAMES.len()]
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Status bar
        ])
        .split(f.area());

    tabs::render(f, app, chunks[0]);

    match app.current_view {
        View::List => {
            if app.tab == Tab::McpServers {
                mcp_list::render(f, app, chunks[1]);
            } else if app.tab == Tab::Plugins {
                plugin_list::render(f, app, chunks[1]);
            } else {
                list::render(f, app, chunks[1]);
            }
        }
        View::Diff => {
            diff::render(f, app, chunks[1]);
        }
        View::EnvInput => {
            // Show MCP list in background, then overlay env input dialog
            mcp_list::render(f, app, chunks[1]);
            env_input::render(f, app, chunks[1]);
        }
        View::ProjectPath => {
            // Show MCP list in background, then overlay project path dialog
            mcp_list::render(f, app, chunks[1]);
            project_path::render(f, app, chunks[1]);
        }
        View::Installing => {
            installing::render(f, app, chunks[1]);
        }
    }

    render_status_bar(f, app, chunks[2]);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    use ratatui::{
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    };

    let help_text = match app.current_view {
        View::List => {
            if app.tab == Tab::McpServers {
                "[Space] Toggle  [i] Install  [r] Remove  [o] Scope  [Tab/1-0,-] Switch Tab  [q] Quit"
            } else if app.tab == Tab::Plugins {
                "[Space] Toggle  [i] Install  [r] Remove  [Tab/1-0,-] Switch Tab  [q] Quit"
            } else if app.tab == Tab::OutputStyles || app.tab == Tab::Statusline {
                "[Space] Toggle  [i] Install  [r] Remove  [d] Diff  [s] Set Default  [Tab/1-0,-] Switch Tab  [q] Quit"
            } else {
                "[Space] Toggle  [i] Install  [r] Remove  [d] Diff  [h/l/←/→] Folder Nav  [Tab/1-0,-] Switch Tab  [q] Quit"
            }
        }
        View::Diff => "[j/k/↑/↓] Scroll  [q/Esc] Close",
        View::EnvInput => "[Enter] Submit  [Esc] Cancel  [Backspace] Delete",
        View::ProjectPath => "[Enter] Confirm  [Esc] Cancel  [Backspace] Delete",
        View::Installing => {
            if app.processing_complete {
                "[Enter/q] Close"
            } else if app.is_removing {
                "Removing..."
            } else {
                "Installing..."
            }
        }
    };

    let status = app.status_message.as_deref().unwrap_or("");

    let spans = vec![
        Span::styled(help_text, Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled(status, Style::default().fg(Color::Yellow)),
    ];

    let paragraph = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(paragraph, area);
}
