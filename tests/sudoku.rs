use std::collections::HashMap;

use wave_function_collapse::*;

pub struct Sudoku {
    layout: Grid<usize>,
}

impl Sudoku {
    pub fn new() -> Self {
        let mut layout = Grid::new(9, 9);

        let mut possibilities = HashMap::new();
        possibilities.insert(1, 1);
        possibilities.insert(2, 1);
        possibilities.insert(3, 1);
        possibilities.insert(4, 1);
        possibilities.insert(5, 1);
        possibilities.insert(6, 1);
        possibilities.insert(7, 1);
        possibilities.insert(8, 1);
        possibilities.insert(9, 1);
        layout.add_possibilities(&possibilities);

        Self { layout }
    }

    pub fn get_layout_mut(&mut self) -> &mut Grid<usize> {
        &mut self.layout
    }
}

impl Wavefunction for Sudoku {
    type V = usize;
    type L = Grid<Self::V>;

    fn get_initial_state(&self) -> &Self::L {
        &self.layout
    }

    fn collapse(
        &mut self,
        layout: &mut Self::L,
        coord: <<Self as Wavefunction>::L as Layout<Self::V>>::Coordinate,
        value: Self::V,
    ) {
        // 1) A cell with the same value cannot be in the same row
        let row = layout.row(coord.y());

        // 2) A cell with the same value cannot be in the same column
        let col = layout.col(coord.x());

        // 3) A cell with the same value cannot be in the same 3x3 subgrid

        // which subgrid rows
        let subgrid_rows = if coord.y() < 3 {
            [0, 1, 2]
        } else if coord.y() < 6 {
            [3, 4, 5]
        } else {
            [7, 8, 9]
        };

        // Which subgrid cols
        let subgrid_cols = if coord.x() < 3 {
            [0, 1, 2]
        } else if coord.x() < 6 {
            [3, 4, 5]
        } else {
            [7, 8, 9]
        };

        let mut subgrid = Vec::new();
        for sub_y in subgrid_rows {
            for sub_x in subgrid_cols {
                subgrid.push(Coord2D::new(sub_x, sub_y));
            }
        }

        let all = [row, col, subgrid].concat();
        layout.remove_cells_possibility(all, &value);
    }

    fn print_layout(&self, layout: &Self::L) {
        for y in 0..9 {
            for x in 0..9 {
                let cell = layout
                    .get_cell(&Coord2D::new(x, y))
                    .expect("board should be 9x9");
                match cell {
                    Cell::Collapsed(value) => print!("{} ", value),
                    Cell::Uncollapsed(possibilites) => {
                        if possibilites.is_empty() {
                            print!("! ");
                        } else {
                            print!("_ ");
                        }
                    }
                }
            }
            println!()
        }
    }
}

#[test]
fn sudoku_easy() {
    let wavefunction = Sudoku::new();

    let mut solver = Solver::new(wavefunction);
    solver.collapse_initial(Coord2D::new(3, 0), 4);
    solver.collapse_initial(Coord2D::new(5, 0), 9);
    solver.collapse_initial(Coord2D::new(6, 0), 8);
    solver.collapse_initial(Coord2D::new(8, 0), 2);

    solver.collapse_initial(Coord2D::new(0, 1), 5);
    solver.collapse_initial(Coord2D::new(1, 1), 7);
    solver.collapse_initial(Coord2D::new(3, 1), 3);
    solver.collapse_initial(Coord2D::new(4, 1), 8);
    solver.collapse_initial(Coord2D::new(8, 1), 4);

    solver.collapse_initial(Coord2D::new(5, 2), 2);
    solver.collapse_initial(Coord2D::new(6, 2), 5);

    solver.collapse_initial(Coord2D::new(0, 3), 3);
    solver.collapse_initial(Coord2D::new(1, 3), 2);
    solver.collapse_initial(Coord2D::new(2, 3), 8);
    solver.collapse_initial(Coord2D::new(4, 3), 1);
    solver.collapse_initial(Coord2D::new(5, 3), 7);
    solver.collapse_initial(Coord2D::new(7, 3), 6);

    solver.collapse_initial(Coord2D::new(1, 4), 5);
    solver.collapse_initial(Coord2D::new(2, 4), 7);
    solver.collapse_initial(Coord2D::new(3, 4), 9);
    solver.collapse_initial(Coord2D::new(4, 4), 3);

    solver.collapse_initial(Coord2D::new(0, 5), 9);
    solver.collapse_initial(Coord2D::new(4, 5), 2);
    solver.collapse_initial(Coord2D::new(6, 5), 7);
    solver.collapse_initial(Coord2D::new(7, 5), 3);

    solver.collapse_initial(Coord2D::new(0, 6), 7);
    solver.collapse_initial(Coord2D::new(1, 6), 8);
    solver.collapse_initial(Coord2D::new(3, 6), 1);

    solver.collapse_initial(Coord2D::new(0, 7), 6);
    solver.collapse_initial(Coord2D::new(2, 7), 5);
    solver.collapse_initial(Coord2D::new(3, 7), 2);
    solver.collapse_initial(Coord2D::new(5, 7), 8);
    solver.collapse_initial(Coord2D::new(8, 7), 7);

    solver.collapse_initial(Coord2D::new(1, 8), 9);
    solver.collapse_initial(Coord2D::new(2, 8), 4);
    solver.collapse_initial(Coord2D::new(4, 8), 7);
    solver.collapse_initial(Coord2D::new(5, 8), 3);
    solver.collapse_initial(Coord2D::new(7, 8), 5);

    println!("Initial State:");
    solver.print_layout();

    println!();

    let output = solver.solve();

    if let Some(ref layout) = output {
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }

    println!("Backtracks: {}", solver.get_backtrack_count());

    assert!(output.is_some());
    // TODO: Assert that this is exactly equal to the correct output
}

#[test]
fn sudoku_hard() {
    let wavefunction = Sudoku::new();

    let mut solver = Solver::new(wavefunction);
    solver.collapse_initial(Coord2D::new(1, 0), 3);
    solver.collapse_initial(Coord2D::new(3, 0), 8);
    solver.collapse_initial(Coord2D::new(5, 0), 2);
    solver.collapse_initial(Coord2D::new(8, 0), 5);

    solver.collapse_initial(Coord2D::new(0, 1), 6);
    solver.collapse_initial(Coord2D::new(5, 1), 9);

    solver.collapse_initial(Coord2D::new(2, 2), 8);
    solver.collapse_initial(Coord2D::new(3, 2), 5);
    solver.collapse_initial(Coord2D::new(7, 2), 4);
    solver.collapse_initial(Coord2D::new(8, 2), 3);

    solver.collapse_initial(Coord2D::new(2, 3), 7);
    solver.collapse_initial(Coord2D::new(8, 3), 1);

    solver.collapse_initial(Coord2D::new(2, 4), 9);
    solver.collapse_initial(Coord2D::new(6, 4), 4);

    solver.collapse_initial(Coord2D::new(0, 5), 5);
    solver.collapse_initial(Coord2D::new(6, 5), 7);

    solver.collapse_initial(Coord2D::new(0, 6), 9);
    solver.collapse_initial(Coord2D::new(1, 6), 6);
    solver.collapse_initial(Coord2D::new(5, 6), 1);
    solver.collapse_initial(Coord2D::new(6, 6), 3);

    solver.collapse_initial(Coord2D::new(3, 7), 6);
    solver.collapse_initial(Coord2D::new(8, 7), 8);

    solver.collapse_initial(Coord2D::new(0, 8), 2);
    solver.collapse_initial(Coord2D::new(3, 8), 9);
    solver.collapse_initial(Coord2D::new(5, 8), 3);
    solver.collapse_initial(Coord2D::new(7, 8), 7);

    println!("Initial State:");
    solver.print_layout();

    println!();

    let output = solver.solve();

    if let Some(ref layout) = output {
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }

    println!("Backtracks: {}", solver.get_backtrack_count());

    assert!(output.is_some());
    // TODO: Assert that this is exactly equal to the correct output
}

#[test]
fn sudoku_invalid() {
    let wavefunction = Sudoku::new();

    let mut solver = Solver::new(wavefunction);
    solver.collapse_initial(Coord2D::new(0, 0), 1);
    solver.collapse_initial(Coord2D::new(2, 0), 4);

    solver.collapse_initial(Coord2D::new(0, 1), 2);
    solver.collapse_initial(Coord2D::new(2, 1), 5);

    solver.collapse_initial(Coord2D::new(0, 2), 3);
    solver.collapse_initial(Coord2D::new(2, 2), 6);

    solver.collapse_initial(Coord2D::new(1, 3), 7);

    solver.collapse_initial(Coord2D::new(1, 4), 8);

    solver.collapse_initial(Coord2D::new(1, 5), 9);

    println!("Initial State:");
    solver.print_layout();

    println!();

    let output = solver.solve();

    if let Some(ref layout) = output {
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }

    println!("Backtracks: {}", solver.get_backtrack_count());

    assert!(output.is_none());
}
