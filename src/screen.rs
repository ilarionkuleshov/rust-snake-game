use crossterm::{execute, terminal};
use std::io::{self, Write};

use crate::playground;
use crate::snake;

pub fn clear() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

pub fn draw(playground: &playground::Playground, snake: &snake::Snake) {
    for y in 0..playground.height {
        for x in 0..playground.width {
            if y == 0 || y == playground.height - 1 || x == 0 || x == playground.width - 1 {
                print!("#");
            } else if x == snake.x && y == snake.y {
                if snake.is_alive {
                    print!("O");
                } else {
                    print!("X");
                }
            } else {
                print!(" ");
            }
        }
        print!("\r\n");
    }
    io::stdout().flush().unwrap();
}
