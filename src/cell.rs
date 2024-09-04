use std::{collections::HashSet, hash::Hash};

use crate::Layout;

pub trait CellValue: Copy + PartialEq + Eq + Hash  {}
impl<T: Copy + PartialEq + Eq + Hash> CellValue for T {}

#[derive(Clone, Debug)]
pub enum Cell<V: CellValue> {
    Collapsed(V),
    Uncollapsed(HashSet<V>),
}

impl<V: CellValue> Cell<V> {
    pub fn get_possibilities(&mut self) -> HashSet<V> {
        match self {
            Cell::Collapsed(_) => HashSet::new(),
            Cell::Uncollapsed(possibilities) => possibilities.clone(),
        }
    }

    pub fn remove(&mut self, value: &V) {
        if let Self::Uncollapsed(values) = self {
            values.remove(value);
        }
    }
}

#[derive(Debug)]
pub struct CellsRef<'a, L: Layout, C: CellValue>{
    layout: &'a mut L,
    cells: &'a mut Vec<Cell<C>>,
}

impl<'a, L: Layout, C: CellValue> CellsRef<'a, L, C> {
    pub fn new(layout: &'a mut L, cells: &'a mut Vec<Cell<C>>) -> Self{
        Self { layout, cells }
    }

    pub fn layout(&mut self) -> &mut L{
        &mut self.layout
    }

    pub fn get_cell(&mut self, coord: L::Coordinate) -> Option<&Cell<C>> {
        let index = self.layout.get_index(coord);
        self.cells.get(index)
    }

    pub fn remove_value(&mut self, coord: L::Coordinate, value: &C){
        let index = self.layout.get_index(coord);
        if let Some(cell_values) = self.cells.get_mut(index) {
            cell_values.remove(value);
        }
    }

    pub fn remove_values(&mut self, coords: Vec<L::Coordinate>, value: &C){
        for coord in coords {
            let index = self.layout.get_index(coord);
            if let Some(cell_values) = self.cells.get_mut(index) {
                cell_values.remove(value);
            }
        }
    }

    pub fn remove_set(&mut self, coord: L::Coordinate, values: &HashSet<C>) {
        let index = self.layout.get_index(coord);
        if let Some(cell_values) = self.cells.get_mut(index) {
            for value in values{
                cell_values.remove(value);
            }
        }
    }

    pub fn remove_sets(&mut self, coords: Vec<L::Coordinate>, values: &HashSet<C>) {
        for coord in coords {
            let index = self.layout.get_index(coord);
            if let Some(cell_values) = self.cells.get_mut(index) {
                for value in values{
                    cell_values.remove(value);
                }
            }
        }
    }
}
