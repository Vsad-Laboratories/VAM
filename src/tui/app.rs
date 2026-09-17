

/// Central application state for the TUI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub screen: Screen,
    pub selected: usize,
    pub should_quit: bool,
}

/// Current screen/view in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Main,
    Help,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Main,
            selected: 0,
            should_quit: false,
        }
    }

    pub fn next(&mut self) {
        self.selected = self.selected.saturating_add(1);
    }

    pub fn prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_help(&mut self) {
        self.screen = match self.screen {
            Screen::Main => Screen::Help,
            Screen::Help => Screen::Main,
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_starts_on_main_screen() {
        let app = App::new();
        assert_eq!(app.screen, Screen::Main);
        assert_eq!(app.selected, 0);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_app_next_increments_selected() {
        let mut app = App::new();
        app.next();
        assert_eq!(app.selected, 1);
    }

    #[test]
    fn test_app_prev_decrements_selected() {
        let mut app = App::new();
        app.next();
        app.next();
        app.prev();
        assert_eq!(app.selected, 1);
    }

    #[test]
    fn test_app_prev_does_not_underflow() {
        let mut app = App::new();
        app.prev();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_app_quit_sets_flag() {
        let mut app = App::new();
        app.quit();
        assert!(app.should_quit);
    }

    #[test]
    fn test_app_toggle_help_switches_screen() {
        let mut app = App::new();
        app.toggle_help();
        assert_eq!(app.screen, Screen::Help);
        app.toggle_help();
        assert_eq!(app.screen, Screen::Main);
    }
}