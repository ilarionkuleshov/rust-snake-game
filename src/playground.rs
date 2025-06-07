use crossterm;

pub struct Playground {
    pub width: i16,
    pub height: i16,
}

impl Playground {
    pub fn new() -> Playground {
        let (width, height) = crossterm::terminal::size().unwrap();
        Playground {
            width: width as i16,
            height: (height - 1) as i16,
        }
    }
}
