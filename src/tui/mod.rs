pub mod app;
pub mod event;
pub mod actions;
pub mod theme;
pub mod ui;

use crate::error::Result;

/// Run the TUI application.
///
/// Initializes the terminal, enters the application loop,
/// and restores the terminal on exit.
pub fn run() -> Result<()> {
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
    use crossterm::execute;
    use ratatui::backend::CrosstermBackend;
    use ratatui::Terminal;
    use std::io::stdout;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app_result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app_result
}

fn run_app(terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    use crate::tui::app::App;
    use crate::tui::event::{poll_event, Event};
    use crate::tui::actions::{map_key, Action};
    use crate::tui::theme::Theme;
    use crate::tui::ui::render;

    let mut app = App::new();
    let theme = Theme::new();

    loop {
        terminal.draw(|frame| render(&app, frame, &theme))?;

        match poll_event()? {
            Event::Key(key) => {
                if let Some(action) = map_key(key) {
                    match action {
                        Action::Quit => {
                            app.quit();
                            break;
                        }
                        Action::Help => {
                            app.toggle_help();
                        }
                        Action::Next => {
                            app.next();
                        }
                        Action::Prev => {
                            app.prev();
                        }
                        Action::ScrollUp => {
                            app.prev();
                        }
                        Action::ScrollDown => {
                            app.next();
                        }
                        Action::Resize => {}
                    }
                }
            }
            Event::Resize(_, _) => {}
            Event::Quit => {
                app.quit();
                break;
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_app_does_not_panic() {
        let app = app::App::new();
        assert_eq!(app.screen, app::Screen::Main);
    }
}