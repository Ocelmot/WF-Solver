use std::fmt::{Debug, Display};

use rand::{seq::SliceRandom, thread_rng};

use crate::{cell::Cell, weighted_iterator::WeightedIterator, Layout, Wavefunction};

pub struct Solver<W: Wavefunction>
where
    W::L: Debug + Display,
{
    wavefunction: W,
    initial_state: W::L,
}

impl<W: Wavefunction> Solver<W>
where
    W::L: Debug + Display,
{
    pub fn new(wavefunction: W) -> Self {
        let layout = wavefunction.get_initial_state().clone();
        return Self {
            wavefunction,
            initial_state: layout,
        };
    }

    pub fn print_layout(&self) {
        self.wavefunction.print_layout(&self.initial_state);
    }

    pub fn collapse_initial(&mut self, coord: <W::L as Layout<W::V>>::Coordinate, value: W::V) {
        self.initial_state.collapse(&coord, value);
        self.wavefunction
            .collapse(&mut self.initial_state, coord, value);
    }

    pub fn solve(&mut self) -> Option<W::L> {
        let mut layout = self.initial_state.clone();

        // Choose a cell at random to collapse
        let new_coord = match self.next_coord(&mut layout) {
            Some(value) => value,
            None => return Some(layout),
        };

        // Collapse the cell with the wavefunction
        let result = self.collapse(&mut layout, &new_coord);

        return result;
    }

    fn collapse(
        &mut self,
        layout: &mut W::L,
        coord: &<W::L as Layout<W::V>>::Coordinate,
    ) -> Option<W::L> {
        // For each possibility in the chosen cell, try solving with that configuration
        let possibilities = layout.get_cell_mut(&coord).unwrap().get_possibilities();
        for possibility in WeightedIterator::new(possibilities) {
            // Clone cells to test possability
            let mut new_layout = layout.clone();

            // Modify cell
            let new_cell = new_layout.get_cell_mut(&coord).unwrap();
            *new_cell = Cell::Collapsed(possibility.clone());

            // Propagate this proposed collapse
            self.wavefunction
                .collapse(&mut new_layout, coord.clone(), possibility.clone());

            let new_coord = match self.next_coord(&mut new_layout) {
                Some(value) => value,
                None => return Some(new_layout),
            };

            // Recurse
            let result = self.collapse(&mut new_layout, &new_coord);
            if result.is_some() {
                return result;
            }
        }

        None
    }

    fn next_coord(
        &self,
        layout: &mut <W as Wavefunction>::L,
    ) -> Option<<<W as Wavefunction>::L as Layout<<W as Wavefunction>::V>>::Coordinate> {
        let mut last_coords = Vec::new();
        let mut last_entropy = f64::MAX;
        for (coord, cell) in layout.candidates() {
            let entropy = cell.entropy();
            if entropy == last_entropy {
                last_coords.push(coord.clone());
            }
            if entropy < last_entropy {
                last_coords = vec![coord.clone()];
                last_entropy = entropy;
            }
        }

        // Choose a possible item, or None if the list is empty
        last_coords.choose(&mut thread_rng()).cloned()
    }
}
