use std::fmt::{Debug, Display};

use rand::{seq::SliceRandom, thread_rng, Rng};

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

    pub fn collapse_initial(&mut self, coord: <W::L as Layout<W::V>>::Coordinate, value: W::V) {
        self.initial_state.collapse(&coord, value);
        self.wavefunction
            .collapse(&mut self.initial_state, coord, value);
    }

    pub fn solve(&mut self) -> Option<W::L> {
        let mut layout = self.initial_state.clone();

        // Choose a cell at random to collapse
        let cells_len = layout.cell_count();

        let mut rng = rand::thread_rng();
        let skip = rng.gen_range(0usize..cells_len);
        let Some((coord, _cell)) = layout.cells().skip(skip).next() else {
            return None;
        };
        // Collapse the cell with the wavefunction
        let result = self.collapse(&mut layout, &coord);

        return result;
    }

    fn collapse(
        &mut self,
        layout: &mut W::L,
        coord: &<W::L as Layout<W::V>>::Coordinate,
    ) -> Option<W::L> {
        // For each possibility in the chosen cell, try solving with that configuration
        let possibilities = layout.get_cell_mut(&coord).unwrap().get_possibilities();
        // let mut possibilities: Vec<_> = possabilities.into_iter().collect();
        // possibilities.shuffle(&mut thread_rng());
        for possibility in WeightedIterator::new(possibilities) {
            // Clone cells to test possability
            let mut new_layout = layout.clone();

            // Modify cell
            let new_cell = new_layout.get_cell_mut(&coord).unwrap();
            *new_cell = Cell::Collapsed(possibility.clone());

            // Propagate this proposed collapse
            // let cells_ref = CellsRef::new(&mut self.layout, &mut new_layout);
            self.wavefunction
                .collapse(&mut new_layout, coord.clone(), possibility.clone());

            // Find next index target
            let mut last_coords = Vec::new();
            let mut last_entropy = f64::MAX;
            for (coord, cell) in new_layout.candidates() {
                let entropy = cell.entropy();
                if entropy == last_entropy {
                    last_coords.push(coord.clone());
                }
                if entropy < last_entropy {
                    last_coords = vec![coord.clone()];
                    last_entropy = entropy;
                }
            }
            // If there are no more candidates, we are done.
            if last_coords.is_empty() {
                return Some(new_layout);
            }
            // Choose coord
            let new_coord = last_coords
                .choose(&mut thread_rng())
                .expect("if list was empty it should have returned");

            // Recurse
            let result = self.collapse(&mut new_layout, new_coord);
            if result.is_some() {
                return result;
            }
        }

        None
    }
}
