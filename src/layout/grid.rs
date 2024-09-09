use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use crate::{cell::Cell, CellValue};

use super::Layout;

#[derive(Clone)]
pub struct Grid<V: CellValue> {
    x: usize,
    y: usize,
    /// The outer Vec is a Vec of rows. The inner vec is the cell within the row.
    cells: Vec<Vec<Cell<V>>>,
}

impl<V: CellValue> Grid<V> {
    pub fn new(x: usize, y: usize) -> Self {
        let cells = vec![vec![Cell::Uncollapsed(HashSet::new()); x]; y];
        Self { x, y, cells }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn row(&self, y: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for x in 0..self.x {
            v.push((x, y));
        }
        v
    }

    pub fn col(&self, x: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for y in 0..self.y {
            v.push((x, y));
        }
        v
    }

    pub fn neighbors(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        v.push((coord.0.saturating_sub(1), coord.1.saturating_sub(1)));
        v.push((coord.0, coord.1.saturating_sub(1)));
        v.push((coord.0 + 1, coord.1.saturating_sub(1)));

        v.push((coord.0.saturating_sub(1), coord.1));
        v.push((coord.0 + 1, coord.1));

        v.push((coord.0.saturating_sub(1), coord.1 + 1));
        v.push((coord.0, coord.1 + 1));
        v.push((coord.0 + 1, coord.1 + 1));
        v
    }
}

impl<V: CellValue> Layout<V> for Grid<V> {
    type Coordinate = (usize, usize);

    fn cells<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Self::Coordinate, &mut Cell<V>)>
    where
        V: 'a,
    {
        self.cells.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, cell)| ((x, y), cell))
        })
    }

    fn cell_count(&self) -> usize {
        self.x * self.y
    }

    fn get_cell(&self, coord: &Self::Coordinate) -> Option<&Cell<V>> {
        if let Some(row) = self.cells.get(coord.1) {
            if let Some(cell) = row.get(coord.0) {
                return Some(cell);
            }
        }
        None
    }

    fn get_cell_mut(&mut self, coord: &Self::Coordinate) -> Option<&mut Cell<V>> {
        if let Some(row) = self.cells.get_mut(coord.1) {
            if let Some(cell) = row.get_mut(coord.0) {
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
