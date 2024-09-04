use std::collections::HashSet;

use crate::{cell::CellsRef, CellValue, Layout};

pub mod grid_test;

pub trait Wavefunction<L: Layout, C: CellValue>{
    fn get_possabilities(&mut self) -> HashSet<C>;
    fn collapse(&mut self, cells: CellsRef<L, C>, coord: L::Coordinate);
}

