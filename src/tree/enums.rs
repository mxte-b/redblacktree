#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RbColor {
    Red,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn invert(&self) -> Direction {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
