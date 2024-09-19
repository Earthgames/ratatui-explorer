use crossterm::event::{Event, KeyCode};

use super::Input;

impl From<&Event> for Input {
    /// Convert crossterm [`Event`](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html) to [`Input`].
    ///
    /// **Note:** This implementation is only available when the `crossterm` feature is enabled.
    fn from(value: &Event) -> Self {
        if let Event::Key(key) = value {
            if matches!(
                key.kind,
                crossterm::event::KeyEventKind::Press | crossterm::event::KeyEventKind::Repeat
            ) {
                let input = match key.code {
                    KeyCode::Char('j') | KeyCode::Down => Input::Down,
                    KeyCode::Char('k') | KeyCode::Up => Input::Up,
                    KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => Input::Left,
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => Input::Right,
                    KeyCode::Char('H') => Input::ToggleHide,
                    _ => Input::None,
                };

                return input;
            }
        }

        Input::None
    }
}
