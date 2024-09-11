use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Coord2D {
    x: usize,
    y: usize,
}

impl Coord2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_sub(1),
        }
    }

    pub fn up_left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y.wrapping_sub(1),
        }
    }
    pub fn up_right(&self) -> Self {
        Self {
            x: self.x.wrapping_add(1),
            y: self.y.wrapping_sub(1),
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_add(1),
        }
    }
    pub fn down_left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y.wrapping_add(1),
        }
    }
    pub fn down_right(&self) -> Self {
        Self {
            x: self.x.wrapping_add(1),
            y: self.y.wrapping_add(1),
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x.wrapping_add(1),
            y: self.y,
        }
    }
}

impl Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
