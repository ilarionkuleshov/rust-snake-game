mod keyboard;
mod playground;
mod screen;
mod snake;
mod vector;

use crossterm::terminal;

fn main() {
    terminal::enable_raw_mode().unwrap();

    let mut playground = playground::Playground::new();
    let mut snake = snake::Snake::new(&playground);
    playground.generate_apples(&snake.score);

    loop {
        let keyboard_event = keyboard::get_keyboard_event();

        snake.update(&playground, &keyboard_event);
        playground.update(&snake.positions.last().unwrap(), &snake.score);

        screen::clear();
        screen::draw(&playground, &snake);

        if !snake.is_alive || keyboard_event == Some(keyboard::KeyboardEvent::Quit) {
            break;
        }
    }
    terminal::disable_raw_mode().unwrap();
    println!();
}
