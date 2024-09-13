#![deny(missing_docs)]

//! A Wavefunction Collapse Solver
//!
//! This solver takes wavefunction and produces a valid output or indicates that
//! no valid solution exists.
//!
//! The typical usage is to define a [CellValue] type to represent the content
//! of the cells. Then implement the [Wavefunction] trait. This will use the
//! previously defined CellValue, and require choosing a [Layout] to use. An
//! implementation of Layout, [Grid], is included. If the included layout is not
//! sufficient, implement Layout to better represent the arrangement of cells.
//!
//! Finally, instantiate the solver by passing an instance of the wavefunction
//! to it. Before solving, the solver can collapse cells to cause them to be
//! fixed to those values in the final output. To generate a solution, call the
//! solve method on the solver.
//!
//! If the solver was able to find a valid output it will be returned. Otherwise
//! it will return None.
//!
//! # Example
//! Below is a minimal example. Additional examples are included in the tests
//! directory.
//!
//! ```rust
//! use wave_function_collapse::{Wavefunction, Grid, Layout, Solver, Coord2D};
//! use std::collections::HashMap;
//!
//! // Define a cell type
//! #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
//! pub enum LandCoastSea {
//!     Land,
//!     Coast,
//!     Sea,
//! }
//!
//! // Implement a Wavefunction
//! pub struct GridTest {
//!     // Layout is generic over the CellValue type
//!     layout: Grid<LandCoastSea>,
//! }
//!
//! impl GridTest {
//!     pub fn new(x: usize, y: usize) -> Self {
//!         // Uses the built in Layout: Grid
//!         let mut layout = Grid::new(x, y);
//!
//!         // Add the cell possibilities to the layout using the specified
//!         // weights.
//!         let mut possibilities = HashMap::new();
//!         possibilities.insert(LandCoastSea::Coast, 5);
//!         possibilities.insert(LandCoastSea::Sea, 100);
//!         possibilities.insert(LandCoastSea::Land, 100);
//!         layout.add_possibilities(&possibilities);
//!
//!         Self { layout }
//!     }
//! }
//!
//! // Implement Wavefunction trait for our example struct
//! impl Wavefunction for GridTest {
//!     // Specify the CellValue defined earlier
//!     type V = LandCoastSea;
//!     // Specify the type of layout used (pass the cell type to the layout)
//!     type L = Grid<Self::V>;
//!
//!     fn get_initial_state(&self) -> &Self::L {
//!         &self.layout
//!     }
//!
//!     // This function will be called after the solver has collapsed a cell.
//!     // It is this funtion's job to maintain the constraints of the
//!     // wavefunction by removing possibilities from other cells. The
//!     // coordinate of the collapsed cell and the value it was collapsed
//!     // to are provided as a convenience.
//!     fn collapse(
//!         &mut self,
//!         layout: &mut Self::L,
//!         coord: <<Self as Wavefunction>::L as Layout<Self::V>>::Coordinate,
//!         value: Self::V,
//!     ) {
//!         // Simple rules for this example: land cannot be directly next to
//!         // sea.
//!         // More complicated wavefunctions could of course do more.
//!         match value {
//!             LandCoastSea::Land => {
//!                 // The layout provides functions to generate Vecs of
//!                 // coordinates. This allows for easy removal of
//!                 // possibilities.
//!                 let coords = layout.neighbors(coord);
//!                 layout.remove_cells_possibility(coords, &LandCoastSea::Sea);
//!             }
//!             // Coasts are unconstrained.
//!             LandCoastSea::Coast => {}
//!             LandCoastSea::Sea => {
//!                 // Make sure the reciprocal constraint is also maintained.
//!                 let coords = layout.neighbors(coord);
//!                 layout.remove_cells_possibility(coords, &LandCoastSea::Land)
//!             }
//!         }
//!     }
//! }
//!
//! // With the implementation finished, its time to instatiate our
//! // wavefunction and run it through the solver.
//!
//! let wavefunction = GridTest::new(50, 30);
//!
//! // Pass the wavefunction to the solver
//! let mut solver = Solver::new(wavefunction);
//!
//! // Set some initial state as needed
//! solver.collapse_initial(Coord2D::new(25, 10), LandCoastSea::Sea);
//!
//! // Get a result for the given initial conditions.
//! // This can be called multiple times on the same initial conditions and will
//! // generate multiple results, if there are multiple valid solutions.
//! let output = solver.solve();
//!
//! // Print the result to the console if one was found.
//! if let Some(layout) = output {
//!     println!("Solution:\n{:?}", layout);
//! } else {
//!     println!("No solution");
//! }
//! ```

mod solver;
pub use solver::Solver;

mod cell;
pub use cell::{Cell, CellValue};

mod layout;
pub use layout::{Coord2D, Grid, Layout};

mod wavefunction;
pub use wavefunction::Wavefunction;

mod weighted_iterator;
