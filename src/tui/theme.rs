use ratatui::style::{Color, Modifier, Style};

/// Centralized visual styling for the VAM TUI.
#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Style,
    pub secondary: Style,
    pub highlight: Style,
    pub status_bar: Style,
    pub help_text: Style,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            primary: Style::default().fg(Color::White),
            secondary: Style::default().fg(Color::DarkGray),
            highlight: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            status_bar: Style::default().fg(Color::Black).bg(Color::White),
            help_text: Style::default().fg(Color::Cyan),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_primary_has_foreground() {
        let theme = Theme::new();
        assert_eq!(theme.primary.fg, Some(Color::White));
    }

    #[test]
    fn test_theme_highlight_is_bold() {
        let theme = Theme::new();
        assert!(theme.highlight.add_modifier.contains(Modifier::BOLD));
    }
}