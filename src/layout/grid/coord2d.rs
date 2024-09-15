use std::fmt::Display;

/// Describes the two dimensional relationship between coordinates.
/// 
/// Used to get a coordinate relative to another coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Up and to the left
    UpLeft,
    /// Up
    Up,
    /// Up and to the right
    UpRight,
    /// Left
    Left,
    /// Right
    Right,
    /// Down and to the left
    DownLeft,
    /// Down
    Down,
    /// Down and to the right
    DownRight,
}

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

    /// Returns a Vec of the coordinates of the eight immediately neighboring
    /// cells.
    pub fn neighbors(&self) -> Vec<Coord2D> {
        let mut v = Vec::new();
        v.push(self.up_left());
        v.push(self.up());
        v.push(self.up_right());

        v.push(self.left());
        v.push(self.right());

        v.push(self.down_left());
        v.push(self.down());
        v.push(self.down_right());
        v
    }

    /// Returns a Vec of a tuple of the coordinates and the [Direction] of the
    /// eight immediately neighboring locations.
    pub fn neighbor_directions(&self) -> Vec<(Coord2D, Direction)> {
        let mut v = Vec::new();
        v.push((self.up_left(), Direction::UpLeft));
        v.push((self.up(), Direction::Up));
        v.push((self.up_right(), Direction::UpRight));

        v.push((self.left(), Direction::Left));
        v.push((self.right(), Direction::Right));

        v.push((self.down_left(), Direction::DownLeft));
        v.push((self.down(), Direction::Down));
        v.push((self.down_right(), Direction::DownRight));
        v
    }

    /// Returns a Vec of a tuple of the coordinates and the [Direction] of the
    /// four orthogonal neighboring locations.
    pub fn neighbor_directions4(&self) -> Vec<(Coord2D, Direction)> {
        let mut v = Vec::new();
        v.push((self.up(), Direction::Up));
        v.push((self.left(), Direction::Left));
        v.push((self.right(), Direction::Right));
        v.push((self.down(), Direction::Down));
        v
    }

    /// Returns the coordinate of the location in the given [Direction].
    pub fn get_neighbor(&self, direction: Direction) -> Self {
        match direction {
            Direction::UpLeft => self.up_left(),
            Direction::Up => self.up(),
            Direction::UpRight => self.up_right(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::DownLeft => self.down_left(),
            Direction::Down => self.down(),
            Direction::DownRight => self.down_right(),
        }
    }

    /// Returns the coordinate of the location in the given [Direction], offset
    /// by x and y.
    ///
    /// Moves along the x and y axes by a multiple of the given x and y values.
    pub fn get_neighbor_scaled(&self, direction: Direction, x: usize, y: usize) -> Self {
        match direction {
            Direction::UpLeft => Self {
                x: self.x.wrapping_sub(x),
                y: self.y.wrapping_sub(y),
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y.wrapping_sub(y),
            },
            Direction::UpRight => Self {
                x: self.x.wrapping_add(x),
                y: self.y.wrapping_sub(y),
            },
            Direction::Left => Self {
                x: self.x.wrapping_sub(x),
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x.wrapping_add(x),
                y: self.y,
            },
            Direction::DownLeft => Self {
                x: self.x.wrapping_sub(x),
                y: self.y.wrapping_add(y),
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y.wrapping_add(y),
            },
            Direction::DownRight => Self {
                x: self.x.wrapping_add(x),
                y: self.y.wrapping_add(y),
            },
        }
    }

    /// Returns the coordinate offset by the given values.
    pub fn offset(&self, x: usize, y: usize) -> Self {
        Self {
            x: self.x.wrapping_add(x),
            y: self.y.wrapping_add(y),
        }
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
