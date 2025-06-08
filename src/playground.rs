use crossterm;
use rand::Rng;

pub struct Playground {
    pub width: i16,
    pub height: i16,
    pub apple_x: Option<i16>,
    pub apple_y: Option<i16>,
}

impl Playground {
    pub fn new() -> Playground {
        let (width, height) = crossterm::terminal::size().unwrap();
        Playground {
            width: width as i16,
            height: (height - 2) as i16,
            apple_x: None,
            apple_y: None,
        }
    }

    pub fn generate_apple(&mut self) {
        self.apple_x = Some(rand::rng().random_range(1..=self.width - 2));
        self.apple_y = Some(rand::rng().random_range(1..=self.height - 2));
    }
}
