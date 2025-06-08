use crate::keyboard::KeyboardEvent;
use crate::playground::Playground;
use crate::vector::Vector;

pub struct Snake {
    pub positions: Vec<Vector>,
    pub direction: Vector,
    pub is_alive: bool,
    pub score: u32,
}

impl Snake {
    pub fn new(playground: &Playground) -> Snake {
        let head_position = Vector {
            x: playground.size.x / 2,
            y: playground.size.y / 2,
        };
        Snake {
            positions: vec![
                // head_position.offset(Vector{x:-2, y:0}),
                head_position.offset(Vector { x: -1, y: 0 }),
                head_position,
            ],
            direction: Vector { x: 1, y: 0 },
            is_alive: true,
            score: 0,
        }
    }

    pub fn update(&mut self, playground: &Playground, keyboard_event: &Option<KeyboardEvent>) {
        match keyboard_event {
            Some(KeyboardEvent::Up) => {
                if self.direction.y <= 0 {
                    self.direction.x = 0;
                    self.direction.y = -1;
                }
            }
            Some(KeyboardEvent::Down) => {
                if self.direction.y >= 0 {
                    self.direction.x = 0;
                    self.direction.y = 1;
                }
            }
            Some(KeyboardEvent::Left) => {
                if self.direction.x <= 0 {
                    self.direction.x = -1;
                    self.direction.y = 0;
                }
            }
            Some(KeyboardEvent::Right) => {
                if self.direction.x >= 0 {
                    self.direction.x = 1;
                    self.direction.y = 0;
                }
            }
            _ => {}
        }
        let position = Vector {
            x: self.positions.last().unwrap().x + self.direction.x,
            y: self.positions.last().unwrap().y + self.direction.y,
        };

        if !playground.contains(&position) || self.positions.contains(&position) {
            self.is_alive = false;
        } else {
            if playground.apples.contains(&position) {
                self.score += 1;
            } else {
                self.positions.remove(0);
            }
            self.positions.push(position);
        }
    }
}
