mod grid;
use std::collections::HashSet;

pub use grid::Grid;

use crate::{cell::Cell, CellValue};

pub trait Layout<V: CellValue>: Clone {
    type Coordinate: Clone;

    fn get_cell(&self, coord: &Self::Coordinate) -> Option<&Cell<V>>;
    fn get_cell_mut(&mut self, coord: &Self::Coordinate) -> Option<&mut Cell<V>>;

    fn add_cell_possibility(&mut self, coord: &Self::Coordinate, possibility: &V) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.add_possibility(possibility);
        }
    }
    fn add_cell_possibilities(&mut self, coord: &Self::Coordinate, possibilities: &HashSet<V>) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.add_possibilities(possibilities);
        }
    }

    fn add_cells_possibility(&mut self, coords: Vec<Self::Coordinate>, possibility: &V) {
        for coord in coords {
            self.add_cell_possibility(&coord, possibility);
        }
    }
    fn add_cells_possibilities(&mut self, coords: Vec<Self::Coordinate>, possibilities: &HashSet<V>){
        for coord in coords {
            self.add_cell_possibilities(&coord, possibilities);
        }
    }

    fn add_possibility(&mut self, possibility: &V){
        for (_, cell) in self.cells() {
            cell.add_possibility(possibility);
        }
    }
    fn add_possibilities(&mut self, possibilities: &HashSet<V>){
        for (_, cell) in self.cells() {
            cell.add_possibilities(possibilities);
        }
    }

    fn remove_cell_possibility(&mut self, coord: &Self::Coordinate, possibility: &V) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.remove_possibility(possibility);
        }
    }
    fn remove_cell_possibilities(&mut self, coord: &Self::Coordinate, possibilities: &HashSet<V>) {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.remove_possibilities(possibilities);
        }
    }

    fn remove_cells_possibility(&mut self, coords: Vec<Self::Coordinate>, possibility: &V) {
        for coord in coords {
            self.remove_cell_possibility(&coord, possibility);
        }
    }
    fn remove_cells_possibilities(&mut self, coords: Vec<Self::Coordinate>, possibilities: &HashSet<V>){
        for coord in coords {
            self.remove_cell_possibilities(&coord, possibilities);
        }
    }

    fn remove_possibility(&mut self, possibility: &V){
        for (_, cell) in self.cells() {
            cell.remove_possibility(possibility);
        }
    }
    fn remove_possibilities(&mut self, possibilities: &HashSet<V>){
        for (_, cell) in self.cells() {
            cell.remove_possibilities(possibilities);
        }
    }

    fn collapse(&mut self, coord: &Self::Coordinate, value: V) -> bool {
        if let Some(cell) = self.get_cell_mut(coord) {
            cell.collapse(value)
        }else{
            false
        }
    }

    fn cells<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Self::Coordinate, &mut Cell<V>)> where V: 'a;
    fn cell_count(&self) -> usize;
    fn candidates<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Self::Coordinate, &mut Cell<V>)> where V: 'a{
        self.cells().filter(|e|{!e.1.is_collapsed()})
    }
}
