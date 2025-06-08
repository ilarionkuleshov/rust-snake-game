use crossterm::event::{self, KeyCode};
use std::time;

#[derive(PartialEq, Eq)]
pub enum KeyboardEvent {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

pub fn get_keyboard_event() -> Option<KeyboardEvent> {
    if event::poll(time::Duration::from_millis(100)).unwrap() {
        if let event::Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('w') | KeyCode::Up => Some(KeyboardEvent::Up),
                KeyCode::Char('s') | KeyCode::Down => Some(KeyboardEvent::Down),
                KeyCode::Char('a') | KeyCode::Left => Some(KeyboardEvent::Left),
                KeyCode::Char('d') | KeyCode::Right => Some(KeyboardEvent::Right),
                KeyCode::Char('q') | KeyCode::Char('Q') => Some(KeyboardEvent::Quit),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
