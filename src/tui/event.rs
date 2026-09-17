use crate::error::Result;

/// Terminal events.
#[derive(Debug, Clone)]
pub enum Event {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
    Quit,
}

/// Poll for the next event with a timeout.
pub fn poll_event() -> Result<Event> {
    use crossterm::event::{poll, read};
    if poll(std::time::Duration::from_millis(100))? {
        match read()? {
            crossterm::event::Event::Key(key) => Ok(Event::Key(key)),
            crossterm::event::Event::Resize(w, h) => Ok(Event::Resize(w, h)),
            crossterm::event::Event::Mouse(_) => Ok(Event::Quit),
            _ => Ok(Event::Quit),
        }
    } else {
        Ok(Event::Quit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_quit_exists() {
        let event = Event::Quit;
        assert!(matches!(event, Event::Quit));
    }
}