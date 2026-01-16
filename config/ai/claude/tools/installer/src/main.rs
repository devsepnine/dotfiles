mod app;
mod component;
mod mcp;
mod plugin;
mod fs;
mod ui;

use std::io;
use anyhow::Result;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, poll},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor::MoveTo,
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All), MoveTo(0, 0))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app in background thread while showing loading animation
    use std::sync::{Arc, Mutex};
    use std::thread;

    let app_result: Arc<Mutex<Option<Result<App>>>> = Arc::new(Mutex::new(None));
    let app_result_clone = Arc::clone(&app_result);

    // Spawn background thread to create app
    thread::spawn(move || {
        let result = App::new();
        *app_result_clone.lock().unwrap() = Some(result);
    });

    // Show loading animation while app is being created
    let mut frame_idx = 0;

    loop {
        terminal.draw(|f| {
            use ratatui::{
                layout::{Alignment, Constraint, Direction, Layout},
                style::{Color, Style},
                widgets::{Block, Borders, Paragraph},
            };

            let area = f.area();

            // Center vertically
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Length(7),
                    Constraint::Percentage(40),
                ])
                .split(area);

            // Center horizontally
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ])
                .split(vertical[1]);

            let spinner = ui::get_spinner(frame_idx);
            let loading_text = format!(
                "\n  {}  Loading...\n\n  Scanning components",
                spinner
            );

            let loading = Paragraph::new(loading_text)
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title(" Claude Config Installer ")
                        .title_style(Style::default().fg(Color::White)),
                );

            f.render_widget(loading, horizontal[1]);
        })?;

        // Check if app is ready
        let result_lock = app_result.lock().unwrap();
        if result_lock.is_some() {
            break;
        }
        drop(result_lock);

        // Update animation frame
        frame_idx += 1;
        std::thread::sleep(Duration::from_millis(100));
    }

    // Get app from result
    let mut app = app_result.lock().unwrap().take().unwrap()?;
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    use std::sync::{Arc, Mutex};
    use std::thread;

    type RefreshResult = (Vec<component::Component>, Vec<mcp::McpServer>, Vec<plugin::Plugin>);
    let refresh_result: Arc<Mutex<Option<Result<RefreshResult>>>> = Arc::new(Mutex::new(None));

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        match app.current_view {
            app::View::Installing => {
                // Check for input (non-blocking with short timeout)
                if poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            handle_installing_input(app, key.code)?;
                        }
                    }
                }

                // Update animation
                app.tick();

                // Process current state
                if !app.processing_queue.is_empty() {
                    // Process next item
                    let done = app.process_step()?;
                    if done {
                        app.start_finish_processing();
                    }
                } else if app.needs_refresh && !app.refreshing {
                    // Start background refresh thread
                    app.refreshing = true;
                    let result_clone = Arc::clone(&refresh_result);
                    let source_dir = app.source_dir.clone();
                    let dest_dir = app.dest_dir.clone();

                    thread::spawn(move || {
                        use crate::fs;

                        let result = (|| -> Result<RefreshResult> {
                            let components = fs::scanner::scan_components(&source_dir, &dest_dir)?;
                            let mcp_servers = fs::scanner::scan_mcp_servers(&source_dir)?;
                            let plugins = fs::scanner::scan_plugins(&source_dir)?;
                            Ok((components, mcp_servers, plugins))
                        })();

                        *result_clone.lock().unwrap() = Some(result);
                    });
                } else if app.refreshing {
                    // Check if refresh thread is done
                    let mut result_lock = refresh_result.lock().unwrap();
                    if let Some(result) = result_lock.take() {
                        match result {
                            Ok((components, mcp_servers, plugins)) => {
                                app.apply_refresh_result(components, mcp_servers, plugins);
                            }
                            Err(e) => {
                                app.processing_log.push(format!("[ERROR] Refresh failed: {}", e));
                                app.needs_refresh = false;
                                app.refreshing = false;
                                app.processing_complete = true;
                            }
                        }
                    }
                }
                // else: Installation complete, just wait for user input to close
            }
            app::View::EnvInput => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        handle_env_input(app, key.code)?;
                    }
                }
            }
            app::View::ProjectPath => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        handle_project_path_input(app, key.code);
                    }
                }
            }
            _ => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match app.current_view {
                            app::View::List => handle_list_input(app, key.code, key.modifiers)?,
                            app::View::Diff => handle_diff_input(app, key.code)?,
                            app::View::EnvInput | app::View::ProjectPath | app::View::Installing => {} // Handled above
                        }
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_list_input(app: &mut App, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
    match key {
        KeyCode::Char('q') => app.should_quit = true,
        // Tab navigation: Tab = next, Shift+Tab = prev
        KeyCode::Tab => {
            if modifiers.contains(KeyModifiers::SHIFT) {
                app.prev_tab();
            } else {
                app.next_tab();
            }
        }
        KeyCode::BackTab => app.prev_tab(), // Some terminals send BackTab for Shift+Tab
        // Number keys 1-8 for direct tab selection
        KeyCode::Char('1') => app.set_tab(0),
        KeyCode::Char('2') => app.set_tab(1),
        KeyCode::Char('3') => app.set_tab(2),
        KeyCode::Char('4') => app.set_tab(3),
        KeyCode::Char('5') => app.set_tab(4),
        KeyCode::Char('6') => app.set_tab(5),
        KeyCode::Char('7') => app.set_tab(6),
        KeyCode::Char('8') => app.set_tab(7),
        KeyCode::Char('9') => app.set_tab(8),
        // List navigation
        KeyCode::Down | KeyCode::Char('j') => app.next_item(),
        KeyCode::Up | KeyCode::Char('k') => app.prev_item(),
        // Selection
        KeyCode::Char(' ') => app.toggle_selected(),
        KeyCode::Char('a') => app.select_all(),
        KeyCode::Char('n') => app.deselect_all(),
        // Actions
        KeyCode::Char('d') | KeyCode::Enter => app.show_diff()?,
        KeyCode::Char('i') => app.install_selected()?,
        KeyCode::Char('r') => app.remove_selected()?,
        KeyCode::Char('s') => {
            // 's' sets default for OutputStyles or Statusline tabs
            if app.tab == app::Tab::OutputStyles {
                app.set_default_style()?;
            } else if app.tab == app::Tab::Statusline {
                app.set_statusline()?;
            }
        }
        KeyCode::Char('o') => {
            // 'o' toggles MCP scope (user/local) when on MCP tab
            if app.tab == app::Tab::McpServers {
                app.toggle_mcp_scope();
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_diff_input(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.close_diff(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_diff_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_diff_up(),
        _ => {}
    }
    Ok(())
}

fn handle_installing_input(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
            if app.processing_queue.is_empty() {
                app.close_processing();
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_env_input(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Esc => app.env_input_cancel(),
        KeyCode::Enter => app.env_input_submit()?,
        KeyCode::Backspace => app.env_input_backspace(),
        KeyCode::Char(c) => app.env_input_char(c),
        _ => {}
    }
    Ok(())
}

fn handle_project_path_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.project_path_cancel(),
        KeyCode::Enter => app.project_path_submit(),
        KeyCode::Backspace => app.project_path_backspace(),
        KeyCode::Char(c) => app.project_path_char(c),
        _ => {}
    }
}
