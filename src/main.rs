mod keyboard;
mod playground;
mod screen;
mod snake;

use crossterm::terminal;

fn main() {
    terminal::enable_raw_mode().unwrap();

    let mut playground = playground::Playground::new();
    playground.generate_apple();

    let mut snake = snake::Snake::new(&playground);

    loop {
        let keyboard_event = keyboard::get_keyboard_event(100);

        let snake_changed = snake.update(&playground, keyboard_event);
        if snake_changed {
            playground.generate_apple();
        }

        screen::clear();
        screen::draw(&playground, &snake);

        if !snake.is_alive {
            break;
        }
    }
    terminal::disable_raw_mode().unwrap();
    println!();
}
