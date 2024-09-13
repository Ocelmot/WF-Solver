
use std::collections::HashMap;

mod grid;
pub use grid::Grid;
mod coord2d;
pub use coord2d::Coord2D;

use crate::{cell::Cell, CellValue};

/// A Layout stores, facillitates modifying, and defines adjacency for a
/// collection of [Cell]s
pub trait Layout<V: CellValue>: Clone {
    /// A coordinate type for this layout to refer to its Cells.
    type Coordinate: Clone;

    /// Get a reference to a [Cell], if that Cell is within bounds.
    fn get_cell(&self, coord: &Self::Coordinate) -> Option<&Cell<V>>;

    /// Get a mutable reference to a [Cell], if that Cell is within bounds.
    fn get_cell_mut(&mut self, coord: &Self::Coordinate) -> Option<&mut Cell<V>>;

    /// Adds a possibility to the [Cell] at the Coordinate, if the Cell is in
    /// bounds.
    /// 
    /// Calls [Cell::add_possibility] on the cell.
    fn add_cell_possibility(&mut self, coord: &Self::Coordinate, possibility: &V) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.add_possibility(possibility);
        }
    }

    /// Increases the weight of the possibility of the [Cell] at the Coordinate,
    /// if the Cell is in bounds.
    /// 
    /// Calls [Cell::add_possibility_count] on the cell.
    fn add_cell_possibility_count(&mut self, coord: &Self::Coordinate, possibility: &V, count: usize) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.add_possibility_count(possibility, count);
        }
    }
    /// Increases the weights of the possibilities of the [Cell] at the
    /// Coordinate, if the Cell is in bounds.
    /// 
    /// Calls [Cell::add_possibilities] on the cell.
    fn add_cell_possibilities(&mut self, coord: &Self::Coordinate, possibilities: &HashMap<V, usize>) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.add_possibilities(possibilities);
        }
    }

    /// Adds a possibility to the [Cell]s at the Coordinates, if the Cells are in
    /// bounds.
    /// 
    /// Calls [Cell::add_possibility] on each cell.
    fn add_cells_possibility(&mut self, coords: Vec<Self::Coordinate>, possibility: &V) {
        for coord in coords {
            self.add_cell_possibility(&coord, possibility);
        }
    }

    /// Increases the weight of the possibility of the [Cell]s at the Coordinates,
    /// if the Cells are in bounds.
    /// 
    /// Calls [Cell::add_possibility_count] on each cell.
    fn add_cells_possibility_count(&mut self, coords: Vec<Self::Coordinate>, possibility: &V, count: usize) {
        for coord in coords {
            self.add_cell_possibility_count(&coord, possibility, count);
        }
    }

    /// Increases the weights of the possibilities of the [Cell] at the
    /// Coordinate, if the Cell is in bounds.
    /// 
    /// Calls [Cell::add_possibilities] on the cell.
    fn add_cells_possibilities(&mut self, coords: Vec<Self::Coordinate>, possibilities: &HashMap<V, usize>){
        for coord in coords {
            self.add_cell_possibilities(&coord, possibilities);
        }
    }

    /// Adds a possibility to all [Cell]s in the [Layout].
    /// 
    /// Calls [Cell::add_possibility] on every cell.
    fn add_possibility(&mut self, possibility: &V){
        for (_, cell) in self.cells() {
            cell.add_possibility(possibility);
        }
    }
    /// Increases the weight of the possibility of every cell [Cell] in the [Layout].
    /// 
    /// Calls [Cell::add_possibility_count] on every cell.
    fn add_possibility_count(&mut self, possibility: &V, count: usize){
        for (_, cell) in self.cells() {
            cell.add_possibility_count(possibility, count);
        }
    }
    /// Increases the weights of the possibilities of every [Cell] in the [Layout]
    /// 
    /// Calls [Cell::add_possibilities] on every cell.
    fn add_possibilities(&mut self, possibilities: &HashMap<V, usize>){
        for (_, cell) in self.cells() {
            cell.add_possibilities(possibilities);
        }
    }

    /// Removes a possibility from the [Cell] at the Coordinate, if the Cell is
    /// in bounds.
    /// 
    /// Calls [Cell::remove_possibility] on the cell.
    fn remove_cell_possibility(&mut self, coord: &Self::Coordinate, possibility: &V) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.remove_possibility(possibility);
        }
    }

    /// Decreases the weight of the possibility of the [Cell] at the Coordinate,
    /// if the Cell is in bounds.
    /// 
    /// Calls [Cell::remove_possibility_count] on the cell.
    fn remove_cell_possibility_count(&mut self, coord: &Self::Coordinate, possibility: &V, count: usize) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.remove_possibility_count(possibility, count);
        }
    }

    /// Decreases the weights of the possibilities of the [Cell] at the
    /// Coordinate, if the Cell is in bounds.
    /// 
    /// Calls [Cell::remove_possibilities] on the cell.
    fn remove_cell_possibilities(&mut self, coord: &Self::Coordinate, possibilities: &HashMap<V, usize>) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.remove_possibilities(possibilities);
        }
    }

    /// Removes a possibility from the [Cell]s at the Coordinates, if the Cells
    /// are in bounds.
    /// 
    /// Calls [Cell::remove_possibility] on each cell.
    fn remove_cells_possibility(&mut self, coords: Vec<Self::Coordinate>, possibility: &V) {
        for coord in coords {
            self.remove_cell_possibility(&coord, possibility);
        }
    }

    /// Decreases the weight of the possibility of the [Cell]s at the
    /// Coordinates, if the Cells are in bounds.
    /// 
    /// Calls [Cell::remove_possibility_count] on each cell.
    fn remove_cells_possibility_count(&mut self, coords: Vec<Self::Coordinate>, possibility: &V, count: usize) {
        for coord in coords {
            self.remove_cell_possibility_count(&coord, possibility, count);
        }
    }

    /// Decreases the weights of the possibilities of the [Cell] at the
    /// Coordinate, if the Cell is in bounds.
    /// 
    /// Calls [Cell::remove_possibilities] on the cell.
    fn remove_cells_possibilities(&mut self, coords: Vec<Self::Coordinate>, possibilities: &HashMap<V, usize>){
        for coord in coords {
            self.remove_cell_possibilities(&coord, possibilities);
        }
    }

    /// Removes a possibility from all [Cell]s in the [Layout].
    /// 
    /// Calls [Cell::remove_possibility] on every cell.
    fn remove_possibility(&mut self, possibility: &V){
        for (_, cell) in self.cells() {
            cell.remove_possibility(possibility);
        }
    }

    /// Decreases the weight of the possibility of every cell [Cell] in the
    /// [Layout].
    /// 
    /// Calls [Cell::remove_possibility_count] on every cell.
    fn remove_possibility_count(&mut self, possibility: &V, count: usize){
        for (_, cell) in self.cells() {
            cell.remove_possibility_count(possibility, count);
        }
    }

    /// Decreases the weights of the possibilities of every [Cell] in the
    /// [Layout]
    /// 
    /// Calls [Cell::remove_possibilities] on every cell.
    fn remove_possibilities(&mut self, possibilities: &HashMap<V, usize>){
        for (_, cell) in self.cells() {
            cell.remove_possibilities(possibilities);
        }
    }

    /// Collapses the [Cell] at the given coordinates to the given value.
    /// 
    /// Since this does not involve the wavefunction, it does not enforce any
    /// rules about regarding the possibilities in any other cell.
    /// 
    /// Returns true if the cell was already collapsed to the given value, or if
    /// the given value was in the uncollapsed set of possibilities. Returns
    /// false otherwise.
    fn collapse(&mut self, coord: &Self::Coordinate, value: V) -> bool {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.collapse(value)
        }else{
            false
        }
    }

    /// Iterates over all cells in the layout. Yielding a 2-tuple of
    /// (Coordinate, Cell)
    /// 
    /// The order is not gaurenteed to be anything in particular.
    fn cells<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Self::Coordinate, &mut Cell<V>)> where V: 'a;
    
    /// The total number of cells in the [Layout]
    fn cell_count(&self) -> usize;

    /// Iterates over all instances of [Cell::Uncollapsed] in the layout.
    /// Yielding a 2-tuple of (Coordinate, Cell)
    /// 
    /// The order is not gaurenteed to be anything in particular.
    /// Default implementation calls and filters [Layout::cells].
    fn candidates<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Self::Coordinate, &mut Cell<V>)> where V: 'a{
        self.cells().filter(|e|{!e.1.is_collapsed()})
    }
}
