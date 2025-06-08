use crate::keyboard::KeyboardEvent;
use crate::playground::Playground;

pub struct Snake {
    pub x: Vec<i16>,
    pub y: Vec<i16>,
    pub dx: i16,
    pub dy: i16,
    pub is_alive: bool,
    pub score: u32,
}

impl Snake {
    pub fn new(playground: &Playground) -> Snake {
        Snake {
            x: vec![playground.width / 2],
            y: vec![playground.height / 2],
            dx: 1,
            dy: 0,
            is_alive: true,
            score: 0,
        }
    }

    pub fn update(
        &mut self,
        playground: &Playground,
        keyboard_event: Option<KeyboardEvent>,
    ) -> bool {
        match keyboard_event {
            Some(KeyboardEvent::Up) => {
                if self.dy != 1 {
                    self.dx = 0;
                    self.dy = -1;
                }
            }
            Some(KeyboardEvent::Down) => {
                if self.dy != -1 {
                    self.dx = 0;
                    self.dy = 1;
                }
            }
            Some(KeyboardEvent::Left) => {
                if self.dx != 1 {
                    self.dx = -1;
                    self.dy = 0;
                }
            }
            Some(KeyboardEvent::Right) => {
                if self.dx != -1 {
                    self.dx = 1;
                    self.dy = 0;
                }
            }
            _ => {}
        }
        let x = self.x.last().unwrap() + self.dx;
        let y = self.y.last().unwrap() + self.dy;

        if x <= 0 || x >= playground.width - 1 || y <= 0 || y >= playground.height - 1 {
            self.is_alive = false;
            false
        } else {
            self.x.push(x);
            self.y.push(y);
            if playground.apple_x == Some(x) && playground.apple_y == Some(y) {
                self.score += 1;
                true
            } else {
                self.x.remove(0);
                self.y.remove(0);
                false
            }
        }
    }
}
