use std::collections::HashMap;

use wave_function_collapse::*;

mod common;
use common::LandCoastSea;

pub struct GridTest {
    layout: Grid<LandCoastSea>,
}

impl GridTest {
    pub fn new(x: usize, y: usize) -> Self {
        let mut layout = Grid::new(x, y);

        let mut possibilities = HashMap::new();
        possibilities.insert(LandCoastSea::Coast, 5);
        possibilities.insert(LandCoastSea::Sea, 100);
        possibilities.insert(LandCoastSea::Land, 100);
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
    let wavefunction = GridTest::new(50, 20);

    let mut solver = Solver::new(wavefunction);
    solver.collapse_initial(Coord2D::new(0,0), LandCoastSea::Land);
    solver.collapse_initial(Coord2D::new(25, 10), LandCoastSea::Sea);
    let output = solver.solve();

    if let Some(layout) = output {
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }

    println!("Backtracks: {}", solver.get_backtrack_count());

}
