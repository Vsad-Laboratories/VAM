use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::tui::app::{App};
use crate::tui::theme::Theme;

/// Render the application state to the terminal.
pub fn render(app: &App, frame: &mut Frame, theme: &Theme) {
    let size = frame.size();
    match app.screen {
        Screen::Main => render_main(frame, theme, size),
        Screen::Help => render_help(frame, theme, size),
    }
}

fn render_main(frame: &mut Frame, theme: &Theme, size: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(size);

    let title = Paragraph::new("VAM — VSAD Arch Manager")
        .style(theme.primary)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Main"));
    frame.render_widget(title, chunks[0]);

    let package_list = Paragraph::new("No packages installed. Use 'vam p-install' to install a package.")
        .style(theme.secondary)
        .alignment(Alignment::Center);
    frame.render_widget(package_list, chunks[1]);

    let status = Paragraph::new("? Help | q Quit")
        .style(theme.status_bar)
        .alignment(Alignment::Center);
    frame.render_widget(status, chunks[2]);
}

fn render_help(frame: &mut Frame, theme: &Theme, size: Rect) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("Help", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("↑/↓ or j/k - Navigate"),
        Line::from("? or h       - Toggle help"),
        Line::from("q/Esc        - Quit"),
        Line::from(""),
        Line::from("TUI Foundation — Goal 011"),
    ];
    let paragraph = Paragraph::new(help_text)
        .style(theme.secondary)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    frame.render_widget(paragraph, size);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::{App, Screen};
    use crate::tui::theme::Theme;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_render_does_not_panic() {
        let app = App::new();
        let theme = Theme::new();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(&app, frame, &theme)).unwrap();
    }

    #[test]
    fn test_render_help_does_not_panic() {
        let mut app = App::new();
        app.toggle_help();
        let theme = Theme::new();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(&app, frame, &theme)).unwrap();
    }

    #[test]
    fn test_render_resize_80x24() {
        let app = App::new();
        let theme = Theme::new();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(&app, frame, &theme)).unwrap();
    }

    #[test]
    fn test_render_resize_120x40() {
        let app = App::new();
        let theme = Theme::new();
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(&app, frame, &theme)).unwrap();
    }
}