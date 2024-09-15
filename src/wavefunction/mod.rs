mod standard;
pub use standard::Standard2D;

use crate::{layout, CellValue, Layout};

/// Implementors of Wavefunction can be passed to the solver to produce the
/// corresponding output. Different implementors may choose to use different
/// [CellValue]s or [Layout]s in thier definition of Wavefunction.
/// 
/// This is the primary way to add new capability to the solver.
pub trait Wavefunction {
    /// The value of the cells used in the [Layout] and the solution output by
    /// the solver.
    type V: CellValue;

    /// The [Layout] this Wavefunction operates within.
    type L: Layout<Self::V>;

    /// Returns a reference to an initial [Layout] state.
    /// 
    /// The solver needs access to the [Layout] used by the Wavefunction in
    /// order to start the solving process.
    fn get_initial_state(&self) -> &Self::L;

    /// Maintains Wavefunction constraints after a cell is collapsed.
    /// 
    /// Once the solver has collapsed a cell, the Wavefunction may remove
    /// possibilites from other cells to ensure that the rules for the output
    /// are enforced.
    /// 
    /// * `layout`: The current layout at this point in the solving process.
    /// * `coord`: The coordinate of the just collapsed cell.
    /// * `value`: The value of the just collapsed cell.
    fn collapse(
        &mut self,
        layout: &mut Self::L,
        coord: <<Self as Wavefunction>::L as layout::Layout<Self::V>>::Coordinate,
        value: Self::V,
    );

    /// Prints the layout. Sometimes used for debugging the solving process.
    /// 
    /// This only needs to be implemented for wavefunctions used in solver
    /// development
    fn print_layout(&self, _layout: &Self::L) {
        println!("Implement `print_layout(&self)` in the Wavefunction trait to print the layout");
    }
}
