mod keyboard;
mod playground;
mod screen;
mod snake;

use crossterm::terminal;

fn main() {
    terminal::enable_raw_mode().unwrap();

    let playground = playground::Playground::new();
    let mut snake = snake::Snake::new(&playground);

    loop {
        let keyboard_event = keyboard::get_keyboard_event(100);

        snake.update(&playground, keyboard_event);
        screen::clear();
        screen::draw(&playground, &snake);

        if !snake.is_alive {
            print!("Game Over!");
            break;
        }
    }
    terminal::disable_raw_mode().unwrap();
}
