use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::{cell::Cell, CellValue};

use super::{Coord2D, Layout};

// GridCoord is used as a common type by Grid for both its own functions as well
// as the associated type for its implementation of Layout.
type GridCoord = Coord2D;

/// A two dimensional grid [Layout] for use by implementors of the trait
/// Wavefunction.
#[derive(Clone)]
pub struct Grid<V: CellValue> {
    x: usize,
    y: usize,
    /// The outer Vec is a Vec of rows. The inner vec is the cell within the row.
    cells: Vec<Vec<Cell<V>>>,
}

impl<V: CellValue> Grid<V> {
    /// Creates a new Grid with size (x, y)
    /// 
    /// Initially filled with uncollapsed, but empty [Cell]s.
    pub fn new(x: usize, y: usize) -> Self {
        let cells = vec![vec![Cell::Uncollapsed(HashMap::new()); x]; y];
        Self { x, y, cells }
    }

    /// The x size of this grid
    pub fn x(&self) -> usize {
        self.x
    }

    /// The y size of this grid
    pub fn y(&self) -> usize {
        self.y
    }

    /// Returns a [`Vec<Coord2D>`] that contains the coordinates for every cell in
    /// the row at position `y`
    pub fn row(&self, y: usize) -> Vec<GridCoord> {
        let mut v = Vec::new();
        for x in 0..self.x {
            v.push(GridCoord::new(x, y));
        }
        v
    }

    /// Returns a [`Vec<Coord2D>`] that contains the coordinates for every cell in
    /// the column at position `x`
    pub fn col(&self, x: usize) -> Vec<GridCoord> {
        let mut v = Vec::new();
        for y in 0..self.y {
            v.push(GridCoord::new(x, y));
        }
        v
    }

    /// Returns a [`Vec<Coord2D>`] that contains the coordinates for the 8 cells
    /// that directly neighbor the cell at `coord`.
    pub fn neighbors(&self, coord: GridCoord) -> Vec<GridCoord> {
        let mut v = Vec::new();
        v.push(GridCoord::new(coord.x().wrapping_sub(1), coord.y().wrapping_sub(1)));
        v.push(GridCoord::new(coord.x(), coord.y().wrapping_sub(1)));
        v.push(GridCoord::new(coord.x() + 1, coord.y().wrapping_sub(1)));

        v.push(GridCoord::new(coord.x().wrapping_sub(1), coord.y()));
        v.push(GridCoord::new(coord.x() + 1, coord.y()));

        v.push(GridCoord::new(coord.x().wrapping_sub(1), coord.y() + 1));
        v.push(GridCoord::new(coord.x(), coord.y() + 1));
        v.push(GridCoord::new(coord.x() + 1, coord.y() + 1));
        v
    }
}

impl<V: CellValue> Layout<V> for Grid<V> {
    type Coordinate = GridCoord;

    fn cells<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Self::Coordinate, &mut Cell<V>)>
    where
        V: 'a,
    {
        self.cells.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, cell)| (Coord2D::new(x, y), cell))
        })
    }

    fn cell_count(&self) -> usize {
        self.x * self.y
    }

    fn get_cell(&self, coord: &Self::Coordinate) -> Option<&Cell<V>> {
        if let Some(row) = self.cells.get(coord.y()) {
            if let Some(cell) = row.get(coord.x()) {
                return Some(cell);
            }
        }
        None
    }

    fn get_cell_mut(&mut self, coord: &Self::Coordinate) -> Option<&mut Cell<V>> {
        if let Some(row) = self.cells.get_mut(coord.y()) {
            if let Some(cell) = row.get_mut(coord.x()) {
                return Some(cell);
            }
        }
        None
    }
}

impl<V: CellValue + Debug> Debug for Grid<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid ({}, {})", self.x, self.y)?;
        for row in &self.cells {
            for (index, cell) in row.iter().enumerate() {
                write!(f, "{:?}", cell)?;
                if index < self.y.saturating_sub(1) {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<V: CellValue + Display> Display for Grid<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            let vals: Vec<_> = row.iter().map(|e| e.to_string()).collect();
            writeln!(f, "{}", vals.join(", "))?;
        }
        Ok(())
    }
}
