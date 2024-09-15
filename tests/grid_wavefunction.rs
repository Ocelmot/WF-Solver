use wave_function_collapse::*;

mod common;
use common::LandCoastSea;

#[test]
fn grid_land_coast_sea() {
    let mut wavefunction = Standard2D::new(10, 10);

    let mut material = Grid::new(7, 8);
    material.collapse(&Coord2D::new(0, 0), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(1, 0), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(2, 0), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(3, 0), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(4, 0), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(5, 0), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(6, 0), LandCoastSea::Sea);

    material.collapse(&Coord2D::new(0, 1), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(1, 1), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(2, 1), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(3, 1), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(4, 1), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(5, 1), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(6, 1), LandCoastSea::Sea);

    material.collapse(&Coord2D::new(0, 2), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(1, 2), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(2, 2), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(3, 2), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(4, 2), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(5, 2), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(6, 2), LandCoastSea::Sea);

    material.collapse(&Coord2D::new(0, 3), LandCoastSea::Coast);
    material.collapse(&Coord2D::new(1, 3), LandCoastSea::Coast);
    material.collapse(&Coord2D::new(2, 3), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(3, 3), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(4, 3), LandCoastSea::Sea);
    material.collapse(&Coord2D::new(5, 3), LandCoastSea::Coast);
    material.collapse(&Coord2D::new(6, 3), LandCoastSea::Coast);

    material.collapse(&Coord2D::new(0, 4), LandCoastSea::Land);
    material.collapse(&Coord2D::new(1, 4), LandCoastSea::Land);
    material.collapse(&Coord2D::new(2, 4), LandCoastSea::Coast);
    material.collapse(&Coord2D::new(3, 4), LandCoastSea::Coast);
    material.collapse(&Coord2D::new(4, 4), LandCoastSea::Coast);
    material.collapse(&Coord2D::new(5, 4), LandCoastSea::Land);
    material.collapse(&Coord2D::new(6, 4), LandCoastSea::Land);

    material.collapse(&Coord2D::new(0, 5), LandCoastSea::Land);
    material.collapse(&Coord2D::new(1, 5), LandCoastSea::Land);
    material.collapse(&Coord2D::new(2, 5), LandCoastSea::Land);
    material.collapse(&Coord2D::new(3, 5), LandCoastSea::Land);
    material.collapse(&Coord2D::new(4, 5), LandCoastSea::Land);
    material.collapse(&Coord2D::new(5, 5), LandCoastSea::Land);
    material.collapse(&Coord2D::new(6, 5), LandCoastSea::Land);

    material.collapse(&Coord2D::new(0, 6), LandCoastSea::Land);
    material.collapse(&Coord2D::new(1, 6), LandCoastSea::Land);
    material.collapse(&Coord2D::new(2, 6), LandCoastSea::Land);
    material.collapse(&Coord2D::new(3, 6), LandCoastSea::Land);
    material.collapse(&Coord2D::new(4, 6), LandCoastSea::Land);
    material.collapse(&Coord2D::new(5, 6), LandCoastSea::Land);
    material.collapse(&Coord2D::new(6, 6), LandCoastSea::Land);

    material.collapse(&Coord2D::new(0, 7), LandCoastSea::Land);
    material.collapse(&Coord2D::new(1, 7), LandCoastSea::Land);
    material.collapse(&Coord2D::new(2, 7), LandCoastSea::Land);
    material.collapse(&Coord2D::new(3, 7), LandCoastSea::Land);
    material.collapse(&Coord2D::new(4, 7), LandCoastSea::Land);
    material.collapse(&Coord2D::new(5, 7), LandCoastSea::Land);
    material.collapse(&Coord2D::new(6, 7), LandCoastSea::Land);
    
    wavefunction.learn(&material);
    println!("Training material:\n{}", material);

    let mut solver = Solver::new(wavefunction);

    // Uncomment to show intermediate solver steps
    // solver.set_on_tile_placement(|l| println!("Layout:\n {}", l.detile()));

    let mut output = solver.solve();

    if let Some(layout) = output.as_mut() {
        let layout = layout.detile();
        println!("Solution:\n{}", layout);
    } else {
        println!("No solution");
    }

    println!("Backtracks: {}", solver.get_backtrack_count());

    assert!(output.is_some())
}
