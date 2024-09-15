use std::fmt::Display;

use crate::CellValue;


/// Tile2D implements [CellValue], and contains a small grid of CellValues
/// 
/// This is used for Wavefunctions that work with a tiling structure.
/// Tile2D can be instantiated to any height and width, the default is 2x2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile2D<V: CellValue, const WIDTH: usize=2, const HEIGHT: usize=WIDTH> {
    pub(crate) contents: [[V; WIDTH]; HEIGHT],
}

impl<V: CellValue, const WIDTH: usize, const HEIGHT: usize> Tile2D<V, WIDTH, HEIGHT> {
    /// Create a new Tile2D from an array of values.
    pub fn new(contents: [[V; WIDTH]; HEIGHT]) -> Self {
        Self { contents }
    }

    /// The width of the tile
    pub fn x(&self) -> usize {
        WIDTH
    }

    /// The height of the tile
    pub fn y(&self) -> usize {
        HEIGHT
    }

    /// Get a value from the tile
    /// 
    /// None if out of bounds
    pub fn get(&self, x: usize, y: usize) -> Option<&V>{
        self.contents.get(y)?.get(x)
    }

    /// Iterates through the values of the tile, yeilding a tuple of (x, y,
    /// value)
    pub fn values(&self) -> impl Iterator<Item = (usize, usize, &V)> + '_ {
        self.contents.iter().enumerate().flat_map(|(y, row)|{
            row.iter().enumerate().map(move |(x, value)|{
                (x, y, value)
            })
        })
    }
}

impl<V: CellValue + Display> Display for Tile2D<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.contents {
            let vals: Vec<_> = row.iter().map(|e| e.to_string()).collect();
            writeln!(f, "{}", vals.join(", "))?;
        }
        Ok(())
    }
}
