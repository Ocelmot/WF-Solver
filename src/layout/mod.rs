
mod grid;
pub use grid::Grid;


pub trait Layout{
    type Coordinate;

    fn size(&mut self) -> usize;
    fn get_coord(&mut self, index: usize) -> Self::Coordinate;
    fn get_index(&mut self, coord: Self::Coordinate) -> usize;
}