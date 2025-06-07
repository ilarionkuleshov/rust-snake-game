use crossterm::event::{self, KeyCode};
use std;

pub enum KeyboardEvent {
    Up,
    Down,
    Left,
    Right,
}

pub fn get_keyboard_event(timeout: u64) -> Option<KeyboardEvent> {
    if event::poll(std::time::Duration::from_millis(timeout)).unwrap() {
        if let event::Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('w') => Some(KeyboardEvent::Up),
                KeyCode::Char('s') => Some(KeyboardEvent::Down),
                KeyCode::Char('a') => Some(KeyboardEvent::Left),
                KeyCode::Char('d') => Some(KeyboardEvent::Right),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
