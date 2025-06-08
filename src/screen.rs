use crossterm::{ExecutableCommand, cursor, execute, terminal};
use std::io::{self, Write};

use crate::playground;
use crate::snake;
use crate::vector::Vector;

pub fn clear() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

pub fn draw(playground: &playground::Playground, snake: &snake::Snake) {
    io::stdout().execute(cursor::MoveTo(0, 0)).unwrap();

    for y in 0..playground.size.y {
        for x in 0..playground.size.x {
            let pos = Vector { x, y };
            if !playground.contains(&pos) {
                print!("#");
            } else if snake.positions.contains(&pos) {
                if snake.positions.last().unwrap() == &pos {
                    if snake.direction.x > 0 {
                        print!("▶");
                    } else if snake.direction.x < 0 {
                        print!("◀");
                    } else if snake.direction.y > 0 {
                        print!("▼");
                    } else if snake.direction.y < 0 {
                        print!("▲");
                    }
                } else if snake.is_alive {
                    print!("o");
                } else {
                    print!("x");
                }
            } else if playground.apples.contains(&pos) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        print!("\r\n");
    }
    if snake.is_alive {
        print!("Score {}", snake.score);
    } else {
        print!("Game Over! Final Score: {}", snake.score);
    }
    io::stdout().flush().unwrap();
}
