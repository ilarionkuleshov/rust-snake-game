use crate::keyboard::KeyboardEvent;
use crate::playground;

pub struct Snake {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub is_alive: bool,
}

impl Snake {
    pub fn new(ground: &playground::Playground) -> Snake {
        Snake {
            x: ground.width / 2,
            y: ground.height / 2,
            dx: 2,
            dy: 0,
            is_alive: true,
        }
    }

    pub fn update(
        &mut self,
        playground: &playground::Playground,
        keyboard_event: Option<KeyboardEvent>,
    ) {
        match keyboard_event {
            Some(KeyboardEvent::Up) => {
                self.dx = 0;
                self.dy = -1;
            }
            Some(KeyboardEvent::Down) => {
                self.dx = 0;
                self.dy = 1;
            }
            Some(KeyboardEvent::Left) => {
                self.dx = -2;
                self.dy = 0;
            }
            Some(KeyboardEvent::Right) => {
                self.dx = 2;
                self.dy = 0;
            }
            _ => {}
        }
        let x = self.x + self.dx;
        let y = self.y + self.dy;

        if x <= 0 || x >= playground.width - 1 || y <= 0 || y >= playground.height - 1 {
            self.is_alive = false;
        } else {
            self.x = x;
            self.y = y;
        }
    }
}
