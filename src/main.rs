mod keyboard;
mod playground;
mod screen;
mod snake;
mod vector;

use crossterm::terminal;

fn main() {
    // for i in 0..10 {
    //     println!("Hello, world! {}", i);
    // }
    terminal::enable_raw_mode().unwrap();

    let mut playground = playground::Playground::new();
    let mut snake = snake::Snake::new(&playground);
    playground.generate_apples(&snake.score);

    loop {
        let keyboard_event = keyboard::get_keyboard_event(100);

        snake.update(&playground, keyboard_event);
        playground.update(&snake.positions.last().unwrap(), &snake.score);

        screen::clear();
        screen::draw(&playground, &snake);

        if !snake.is_alive {
            break;
        }
    }
    terminal::disable_raw_mode().unwrap();
    println!();
}
