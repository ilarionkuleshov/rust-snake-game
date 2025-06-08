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
            } else if x == playground.apple_x.unwrap() && y == playground.apple_y.unwrap() {
                print!("*");
            } else {
                let mut is_snake_part = false;
                for i in 0..snake.x.len() {
                    if x == snake.x[i] && y == snake.y[i] {
                        if snake.is_alive {
                            print!("O");
                        } else {
                            print!("X");
                        }
                        is_snake_part = true;
                        break;
                    }
                }
                if !is_snake_part {
                    print!(" ");
                }
            }
        }
        print!("\r\n");
    }
    io::stdout().flush().unwrap();
}
