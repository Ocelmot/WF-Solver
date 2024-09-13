use std::fmt::Display;

/// A two dimensional coordinate used by the Grid Layout.
/// 
/// Coord2D uses the pixel-coordinate convention where positive y values extend
/// below the axis. This means that the relative coordinate operations that have
/// an upward component like `.up()` will have a smaller y value as they
/// approach the axis.
/// 
/// Additionally, the coordinates wrap when they reach the edges of the allowed
/// range of values. using `.left()` on a coordinate with a 0 x component will
/// result in a coordinate with an x component of `usize::MAX`
#[derive(Debug, Clone, Copy)]
pub struct Coord2D {
    x: usize,
    y: usize,
}

impl Coord2D {
    /// Create a Coord2D with the given x and y values
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// The x value of this Coord2D
    pub fn x(&self) -> usize {
        self.x
    }

    /// The y value of this Coord2D
    pub fn y(&self) -> usize {
        self.y
    }

    /// The Coord2D immediately above the current cell.
    /// 
    /// Since the y axis extends downward, the coordinate will have a smaller y
    /// value except where it wraps from 0 to usize::MAX
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_sub(1),
        }
    }

    /// The Coord2D up and to the left of the current cell.

    pub fn up_left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y.wrapping_sub(1),
        }
    }

    /// The Coord2D up and to the right of the current cell.
    pub fn up_right(&self) -> Self {
        Self {
            x: self.x.wrapping_add(1),
            y: self.y.wrapping_sub(1),
        }
    }

    /// The Coord2D immediately below the current cell.
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_add(1),
        }
    }

    /// The Coord2D down and to the left of the current cell.
    pub fn down_left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y.wrapping_add(1),
        }
    }

    /// The Coord2D down and to the right of the current cell.
    pub fn down_right(&self) -> Self {
        Self {
            x: self.x.wrapping_add(1),
            y: self.y.wrapping_add(1),
        }
    }

    /// The Coord2D immediately to the left of the current cell.
    pub fn left(&self) -> Self {
        Self {
            x: self.x.wrapping_sub(1),
            y: self.y,
        }
    }

    /// The Coord2D immediately to the right of the current cell.
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
