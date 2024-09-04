
use std::{fmt::Debug, usize};

use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{cell::{Cell, CellsRef}, CellValue, Layout, Wavefunction};

pub struct Solver<L: Layout, V: CellValue, W: Wavefunction<L, V>> {
    cells: Vec<Cell<V>>,
    size: usize,
    layout: L,
    wavefunction: W,
}

impl<L: Layout, V: CellValue + Debug, W: Wavefunction<L, V>> Solver<L, V, W> {
    pub fn new(mut layout: L, mut wavefunction: W) -> Self {
        let size = layout.size();
        let cells = vec![Cell::Uncollapsed(wavefunction.get_possabilities()); size];

        return Self {
            cells,
            size,
            layout,
            wavefunction,
        };
    }

    pub fn solve(&mut self) -> Option<Vec<Cell<V>>>{
        let mut cells = self.cells.clone();

        // Choose a cell at random to collapse
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0usize..self.size);
        // Collapse the cell with the wavefunction
        let result = self.collapse(&mut cells, index);
        

        return result;
    }

    fn collapse(&mut self, cells: &mut Vec<Cell<V>>, index: usize) -> Option<Vec<Cell<V>>> {
        println!("Collapsing index {}", index);
        println!("Cells:");
        for y in 0..10 {
            println!("{:?}, {:?}", cells.get(y*2), cells.get(y*2+1));
        }


        // For each possibility in the chosen cell, try solving with that configuration
        let possabilities = cells.get_mut(index).unwrap().get_possibilities();
        let mut possabilities: Vec<_> = possabilities.into_iter().collect();
        possabilities.shuffle(&mut thread_rng());
        for possability in possabilities {
            // Clone cells to test possability
            let mut new_cells = cells.clone();
            
            // Modify cell
            let new_cell = new_cells.get_mut(index).unwrap();
            *new_cell = Cell::Collapsed(possability.clone());

            // Propagate this proposed collapse
            let coord = self.layout.get_coord(index);
            let cells_ref = CellsRef::new(&mut self.layout, &mut new_cells);
            self.wavefunction.collapse(cells_ref, coord);
            
            // Find next index target
            let mut last_index = None;
            let mut last_possibilities = usize::MAX;
            for (i, cell) in new_cells.iter().enumerate() {
                match cell {
                    Cell::Collapsed(_) => continue,
                    Cell::Uncollapsed(possabilities) => {
                        if possabilities.is_empty() {
                            // Solving is no longer possable
                            return None;
                        }
                        if possabilities.len() < last_possibilities {
                            // Found new, lower possabilities
                            last_index = Some(i);
                            last_possibilities = possabilities.len();
                        }
                    },
                }
            }
            // If there is no index, we are done.
            let new_index = match last_index {
                Some(index) => index,
                None => {
                    // Solving is complete, return
                    return Some(new_cells)
                },
            };

            // Recurse
            let result = self.collapse(&mut new_cells, new_index);
            if result.is_some() {
                return result;
            }
        }

        None
    }
}
