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
        possibilities.insert(LandCoastSea::Coast, 2);
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
        // Additionally, when a cell is next to a coast, it cannot also be on the opposite side of the coast
        // When a coast is placed it must also clear its neighbors to accomodate this.
        match value {
            LandCoastSea::Land => {
                // Sea cannot be next to land
                let coords = layout.neighbors(coord);
                layout.remove_cells_possibility(coords, &LandCoastSea::Sea);

                // for each neighboring coast, remove the possibility of land from its opposite side
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.up_left()) {
                    layout.remove_cell_possibility(&coord.up_left().up_left(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.up()) {
                    layout.remove_cell_possibility(&coord.up().up(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.up_right()) {
                    layout.remove_cell_possibility(&coord.up_right().up_right(), &LandCoastSea::Land);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.left()) {
                    layout.remove_cell_possibility(&coord.left().left(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.right()) {
                    layout.remove_cell_possibility(&coord.right().right(), &LandCoastSea::Land);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.down_left()) {
                    layout.remove_cell_possibility(&coord.down_left().down_left(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.down()) {
                    layout.remove_cell_possibility(&coord.down().down(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.down_right()) {
                    layout.remove_cell_possibility(&coord.down_right().down_right(), &LandCoastSea::Land);
                }
            }
            LandCoastSea::Coast => {
                // Check each neighbor, if a neighbor is land make sure cross neighbor cannot be land. etc.
                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.up_left()) {
                    layout.remove_cell_possibility(&coord.down_right(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.up()) {
                    layout.remove_cell_possibility(&coord.down(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.up_right()) {
                    layout.remove_cell_possibility(&coord.down_left(), &LandCoastSea::Land);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.left()) {
                    layout.remove_cell_possibility(&coord.right(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.right()) {
                    layout.remove_cell_possibility(&coord.left(), &LandCoastSea::Land);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.down_left()) {
                    layout.remove_cell_possibility(&coord.up_right(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.down()) {
                    layout.remove_cell_possibility(&coord.up(), &LandCoastSea::Land);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Land)) = layout.get_cell(&coord.down_right()) {
                    layout.remove_cell_possibility(&coord.up_left(), &LandCoastSea::Land);
                }

                // Check same constraint for Sea
                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.up_left()) {
                    layout.remove_cell_possibility(&coord.down_right(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.up()) {
                    layout.remove_cell_possibility(&coord.down(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.up_right()) {
                    layout.remove_cell_possibility(&coord.down_left(), &LandCoastSea::Sea);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.left()) {
                    layout.remove_cell_possibility(&coord.right(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.right()) {
                    layout.remove_cell_possibility(&coord.left(), &LandCoastSea::Sea);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.down_left()) {
                    layout.remove_cell_possibility(&coord.up_right(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.down()) {
                    layout.remove_cell_possibility(&coord.up(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Sea)) = layout.get_cell(&coord.down_right()) {
                    layout.remove_cell_possibility(&coord.up_left(), &LandCoastSea::Sea);
                }
            }
            LandCoastSea::Sea => {
                // Land cannot be next to sea
                let coords = layout.neighbors(coord);
                layout.remove_cells_possibility(coords, &LandCoastSea::Land);

                // for each neighboring coast, remove the possibility of sea from its opposite side.
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.up_left()) {
                    layout.remove_cell_possibility(&coord.up_left().up_left(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.up()) {
                    layout.remove_cell_possibility(&coord.up().up(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.up_right()) {
                    layout.remove_cell_possibility(&coord.up_right().up_right(), &LandCoastSea::Sea);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.left()) {
                    layout.remove_cell_possibility(&coord.left().left(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.right()) {
                    layout.remove_cell_possibility(&coord.right().right(), &LandCoastSea::Sea);
                }

                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.down_left()) {
                    layout.remove_cell_possibility(&coord.down_left().down_left(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.down()) {
                    layout.remove_cell_possibility(&coord.down().down(), &LandCoastSea::Sea);
                }
                if let Some(Cell::Collapsed(LandCoastSea::Coast)) = layout.get_cell(&coord.down_right()) {
                    layout.remove_cell_possibility(&coord.down_right().down_right(), &LandCoastSea::Sea);
                }
            }
        }
    }
}

#[test]
fn grid_land_coast_sea() {
    let wavefunction = GridTest::new(50, 30);

    let mut solver = Solver::new(wavefunction);
    // solver.collapse_initial(Coord2D::new(0,0), LandCoastSea::Land);
    // solver.collapse_initial(Coord2D::new(25, 10), LandCoastSea::Sea);
    let output = solver.solve();

    if let Some(layout) = output {
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }
}
