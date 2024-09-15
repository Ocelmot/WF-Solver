use std::collections::HashMap;

use crate::{cell::{Function, Operation}, layout::grid::Direction, CellValue, Coord2D, Grid, Layout, Tile2D};

use super::Wavefunction;

/// This is an implementation of the standard two dimensional wavefunction
/// collapse algorithm.
/// 
/// This implementation extracts 2x2 tiles from a [Layout] and uses thier
/// adjacency and frequency to build the rules to constrain the output.
pub struct Standard2D<V: CellValue> {
    layout: Grid<Tile2D<V>>,
    /// For a tile type, there is a probability map for each directional
    /// neighbor.
    adj_map: HashMap<Tile2D<V>, HashMap<Direction, HashMap<Tile2D<V>, usize>>>,
}

impl<V: CellValue> Standard2D<V>{
    /// Create a new [Standard2D] on a grid with the given dimensions.
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            layout: Grid::new(x, y),
            adj_map: HashMap::new(),
        }
    }

    /// Build a set of adjacency frequencies from the completed [Grid].
    /// 
    /// This functions could be called multiple times with different grids, as
    /// long as they have the same CellValue type. However, this could cause an
    /// issue if there is no adjacency between tiles of one set and another, as
    /// the solver would have to backtrack to remove all tiles from one set in
    /// order to complete the layout.
    pub fn learn(&mut self, material: &Grid<V>) {
        // Iterate through the material, accumulating tiles for the rules list.
        for x in 0..material.x() {
            for y in 0..material.y() {
                let tile_coord = Coord2D::new(x, y);
                let Some(tile) = material.get_tile(tile_coord) else {
                    continue;
                };

                // get neighboring tiles and add them as possibilities in the map
                for direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
                    let neighbor_coord = tile_coord.get_neighbor_scaled(direction, tile.x(), tile.y());
                    if let Some(adjacent) = material.get_tile(neighbor_coord) {
                        self.add_adjacency(tile, direction, adjacent);
                    }
                }

                // Since we have found a tile, it needs to be added to the board
                // as possibilities.
                self.layout.add_possibility(&tile);
            }
        }
    }

    fn add_adjacency(&mut self, tile: Tile2D<V>, direction: Direction, adjacent: Tile2D<V>) {
        // Get map of direction->valid tiles from th adj map
        let tile_adj = self.adj_map.entry(tile).or_default();

        // Get valid tiles for a direction
        let direction_adj = tile_adj.entry(direction).or_default();

        // Get count for this adjacent tile
        let count = direction_adj.entry(adjacent).or_default();

        // Increase count since we have an observation
        *count = *count + 1;
    }
}

impl<V: CellValue> Wavefunction for Standard2D<V> {
    type V = Tile2D<V>;

    type L = Grid<Tile2D<V>>;

    fn get_initial_state(&self) -> &Self::L {
        &self.layout
    }

    fn collapse(
        &mut self,
        layout: &mut Self::L,
        coord: <<Self as Wavefunction>::L as crate::layout::Layout<Self::V>>::Coordinate,
        value: Self::V,
    ) {
        let tile_adjacencies = match self.adj_map.get(&value) {
            Some(adjacencies) => adjacencies,
            None => {
                // This case represents a cell with no known allowed adjacencies in any direction.
                for neighbor in coord.neighbors() {
                    layout.clear_cell(&neighbor);
                }
                return;
            }
        };

        for (neighbor, direction) in coord.neighbor_directions4() {
            if let Some(neighbor_constraints) = tile_adjacencies.get(&direction) {
                layout.merge_cell_possibilities(&neighbor, Operation::Intersection, Function::Min, neighbor_constraints);
            } else {
                // There are no known allowed adjacencies in this direction.
                layout.clear_cell(&neighbor);
            }
        }
    }
}
