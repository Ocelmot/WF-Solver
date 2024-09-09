use std::collections::HashSet;

use wave_function_collapse::*;

mod common;
use common::LandCoastSea;

pub struct GridTest {
    layout: Grid<LandCoastSea>,
}

impl GridTest {
    pub fn new(x: usize, y: usize) -> Self {
        let mut layout = Grid::new(x, y);

        let mut possibilities = HashSet::new();
        possibilities.insert(LandCoastSea::Coast);
        possibilities.insert(LandCoastSea::Sea);
        possibilities.insert(LandCoastSea::Land);
        layout.add_possibilities(&possibilities);

        Self { layout }
    }

    pub fn get_layout_mut(&mut self) -> &mut Grid<LandCoastSea> {
        &mut self.layout
    }
}

impl Wavefunction for GridTest {
    type V = LandCoastSea;

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
        // Simple rules, land cannot be directly next to sea.
        // There must be a coast inbetween
        match value {
            LandCoastSea::Land => {
                let coords = layout.neighbors(coord);
                layout.remove_cells_possibility(coords, &LandCoastSea::Sea);
            }
            LandCoastSea::Coast => {}
            LandCoastSea::Sea => {
                let coords = layout.neighbors(coord);
                layout.remove_cells_possibility(coords, &LandCoastSea::Land)
            }
        }
    }
}

#[test]
fn grid_land_coast_sea() {
    let wavefunction = GridTest::new(10, 15);
    let layout = wavefunction.get_initial_state().clone();
    println!("Initial Configuration:\n{}", layout);

    let mut solver = Solver::new(wavefunction);
    let output = solver.solve();

    if let Some(layout) = output {
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }
}
