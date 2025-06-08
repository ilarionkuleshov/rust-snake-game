use crate::vector::Vector;
use crossterm;
use rand::Rng;

pub struct Playground {
    pub size: Vector,
    pub apples: Vec<Vector>,
}

impl Playground {
    pub fn new() -> Playground {
        let (width, height) = crossterm::terminal::size().unwrap();
        Playground {
            size: Vector {
                x: width as i16,
                y: (height - 2) as i16,
            },
            apples: Vec::new(),
        }
    }

    pub fn update(&mut self, snake_head_position: &Vector, snake_score: &u32) {
        for idx in (0..self.apples.len()).rev() {
            if self.apples[idx] == *snake_head_position {
                self.apples.remove(idx);
                self.generate_apples(snake_score);
            }
        }
    }

    pub fn generate_apples(&mut self, snake_score: &u32) {
        let num = if *snake_score < 10 {
            2
        } else if *snake_score < 20 {
            3
        } else {
            4
        };

        for _ in 0..num {
            let mut apple: Vector;

            loop {
                apple = Vector {
                    x: rand::rng().random_range(1..=self.size.x - 3),
                    y: rand::rng().random_range(1..=self.size.y - 3),
                };
                if !self.apples.contains(&apple) {
                    break;
                }
            }

            self.apples.push(apple);
        }
    }

    pub fn contains(&self, vector: &Vector) -> bool {
        vector.x > 0 && vector.x < self.size.x - 1 && vector.y > 0 && vector.y < self.size.y - 1
    }
}
