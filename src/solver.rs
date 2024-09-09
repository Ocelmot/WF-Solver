use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{cell::Cell, Layout, Wavefunction};

pub struct Solver<W: Wavefunction> {
    wavefunction: W,
}

impl<W: Wavefunction> Solver<W> {
    pub fn new(wavefunction: W) -> Self {
        return Self { wavefunction };
    }

    pub fn solve(&mut self) -> Option<W::L> {
        let mut layout = self.wavefunction.get_initial_state().clone();

        // Choose a cell at random to collapse
        let cells_len = layout.cell_count();

        let mut rng = rand::thread_rng();
        let skip = rng.gen_range(0usize..cells_len);
        let Some((coord, _cell)) = layout.cells().skip(skip).next() else {
            return None;
        };
        // Collapse the cell with the wavefunction
        let result = self.collapse(&mut layout, coord);

        return result;
    }

    fn collapse(
        &mut self,
        layout: &mut W::L,
        coord: <W::L as Layout<W::V>>::Coordinate,
    ) -> Option<W::L> {
        // For each possibility in the chosen cell, try solving with that configuration
        let possabilities = layout.get_cell_mut(&coord).unwrap().get_possibilities();
        let mut possabilities: Vec<_> = possabilities.into_iter().collect();
        possabilities.shuffle(&mut thread_rng());
        for possability in possabilities {
            // Clone cells to test possability
            let mut new_layout = layout.clone();

            // Modify cell
            let new_cell = new_layout.get_cell_mut(&coord).unwrap();
            *new_cell = Cell::Collapsed(possability.clone());

            // Propagate this proposed collapse
            // let cells_ref = CellsRef::new(&mut self.layout, &mut new_layout);
            self.wavefunction
                .collapse(&mut new_layout, coord.clone(), possability.clone());

            // Find next index target
            let mut last_coord = None;
            let mut last_possibilities = usize::MAX;
            for (coord, cell) in new_layout.candidates() {
                match cell {
                    Cell::Collapsed(_) => continue,
                    Cell::Uncollapsed(possabilities) => {
                        if possabilities.is_empty() {
                            // Solving is no longer possable
                            return None;
                        }
                        if possabilities.len() < last_possibilities {
                            // Found new, lower possabilities
                            last_coord = Some(coord);
                            last_possibilities = possabilities.len();
                        }
                    }
                }
            }
            // If there is no index, we are done.
            let new_index = match last_coord {
                Some(index) => index,
                None => {
                    // Solving is complete, return
                    return Some(new_layout);
                }
            };

            // Recurse
            let result = self.collapse(&mut new_layout, new_index);
            if result.is_some() {
                return result;
            }
        }

        None
    }
}
