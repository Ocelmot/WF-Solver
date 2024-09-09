use crate::{layout, CellValue, Layout};

pub trait Wavefunction {
    type V: CellValue;
    type L: Layout<Self::V>;

    fn get_initial_state(&self) -> &Self::L;

    fn collapse(
        &mut self,
        layout: &mut Self::L,
        coord: <<Self as Wavefunction>::L as layout::Layout<Self::V>>::Coordinate,
        value: Self::V,
    );
}
