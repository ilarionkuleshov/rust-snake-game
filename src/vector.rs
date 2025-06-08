#[derive(PartialEq, Eq)]
pub struct Vector {
    pub x: i16,
    pub y: i16,
}

impl Vector {
    pub fn offset(&self, vector: Vector) -> Vector {
        Vector {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}
