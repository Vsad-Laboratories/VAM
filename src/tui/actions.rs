/// Actions that can be performed by the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Help,
    Next,
    Prev,
    ScrollUp,
    ScrollDown,
    Resize,
}

/// Map a key event to an action.
pub fn map_key(key: crossterm::event::KeyEvent) -> Option<Action> {
    use crossterm::event::KeyCode;
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
        KeyCode::Char('?') | KeyCode::Char('h') => Some(Action::Help),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::Next),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::Prev),
        KeyCode::PageDown => Some(Action::ScrollDown),
        KeyCode::PageUp => Some(Action::ScrollUp),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn test_map_key_quit() {
        let key = crossterm::event::KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::empty());
        assert_eq!(map_key(key), Some(Action::Quit));
    }

    #[test]
    fn test_map_key_escape() {
        let key = crossterm::event::KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::empty());
        assert_eq!(map_key(key), Some(Action::Quit));
    }

    #[test]
    fn test_map_key_help() {
        let key = crossterm::event::KeyEvent::new(KeyCode::Char('?'), crossterm::event::KeyModifiers::empty());
        assert_eq!(map_key(key), Some(Action::Help));
    }

    #[test]
    fn test_map_key_down() {
        let key = crossterm::event::KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::empty());
        assert_eq!(map_key(key), Some(Action::Next));
    }

    #[test]
    fn test_map_key_up() {
        let key = crossterm::event::KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::empty());
        assert_eq!(map_key(key), Some(Action::Prev));
    }

    #[test]
    fn test_map_key_unmapped() {
        let key = crossterm::event::KeyEvent::new(KeyCode::Char('z'), crossterm::event::KeyModifiers::empty());
        assert_eq!(map_key(key), None);
    }
}