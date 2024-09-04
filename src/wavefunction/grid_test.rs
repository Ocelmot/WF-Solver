use std::collections::HashSet;

use crate::{cell::{Cell, CellsRef}, layout, Grid, Layout};

use super::Wavefunction;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TileTypes{
    Land,
    Coast,
    Sea,
}

pub struct GridTest{
    

}

impl GridTest {}

impl Wavefunction<Grid, TileTypes> for GridTest {
    fn get_possabilities(&mut self) -> HashSet<TileTypes> {
        let mut possabilities = HashSet::new();

        possabilities.insert(TileTypes::Coast);
        possabilities.insert(TileTypes::Sea);
        possabilities.insert(TileTypes::Land);

        // possabilities.insert(0);
        // possabilities.insert(1);
        // possabilities.insert(2);
        // possabilities.insert(3);
        // possabilities.insert(4);
        // possabilities.insert(5);
        // possabilities.insert(6);
        // possabilities.insert(7);
        // possabilities.insert(8);
        // possabilities.insert(9);

        // possabilities.insert(10);
        // possabilities.insert(11);
        // possabilities.insert(12);
        // possabilities.insert(13);
        // possabilities.insert(14);
        // possabilities.insert(15);
        // possabilities.insert(16);
        // possabilities.insert(17);
        // possabilities.insert(18);
        // possabilities.insert(19);

        possabilities
    }

    fn collapse(&mut self, mut cells: CellsRef<Grid, TileTypes>, coord: <Grid as layout::Layout>::Coordinate) {
        // Cells above cannot be lower
        if let Some(Cell::Collapsed(value)) = cells.get_cell(coord).cloned(){
            match value {
                TileTypes::Land => {
                    let coords = cells.layout().neighbors(coord);
                    cells.remove_values(coords, &TileTypes::Sea);
                },
                TileTypes::Coast => {},
                TileTypes::Sea => {
                    let coords = cells.layout().neighbors(coord);
                    cells.remove_values(coords, &TileTypes::Land)
                },
            }
        }else{
            eprintln!("Coord: {:?}", coord);
            eprintln!("Index: {:?}", cells.layout().get_index(coord));
            eprintln!("Cells: {:?}", cells);
            panic!("Coord was found to be uncollapsed!")
        }



        // // Cells above cannot be lower
        // if let Some(Cell::Collapsed(value)) = cells.get_cell(coord){
        //     let mut values = HashSet::new();
        //     for value in 0..*value {
        //         values.insert(value);
        //     }

        //     for y in 0..=coord.1 {
        //         let coords =  cells.layout().row(y);
        //         cells.remove_sets(coords, &values);
        //     }
        // }else{
        //     eprintln!("Coord: {:?}", coord);
        //     eprintln!("Index: {:?}", cells.layout().get_index(coord));
        //     eprintln!("Cells: {:?}", cells);
        //     panic!("Coord was found to be uncollapsed!")
        // }

        // // Cells below cannot be higher
        // if let Some(Cell::Collapsed(value)) = cells.get_cell(coord){
        //     let mut values = HashSet::new();
        //     for value in *value+1..20 {
        //         values.insert(value);
        //     }

        //     for y in coord.1..cells.layout().y() {
        //         let coords =  cells.layout().row(y);
        //         cells.remove_sets(coords, &values);
        //     }
        // }else{
        //     eprintln!("Coord: {:?}", coord);
        //     eprintln!("Index: {:?}", cells.layout().get_index(coord));
        //     eprintln!("Cells: {:?}", cells);
        //     panic!("Coord was found to be uncollapsed!")
        // }
        
        
        // Up and to the left cannot be lower than the cell
        // let mut coords = Vec::new();
        // for x in 0..coord.0 {
        //     for y in 0..coord.1 {
        //         coords.push((x,y));
        //     }
        // }
        // let mut values = HashSet::new();
        // if let Some(Cell::Collapsed(value)) = cells.get_cell(coord){
        //     for value in 0..*value {
        //         values.insert(value);
        //     }
        // }else{
        //     eprintln!("Coord: {:?}", coord);
        //     eprintln!("Index: {:?}", cells.layout().get_index(coord));
        //     eprintln!("Cells: {:?}", cells);
        //     panic!("Coord was found to be uncollapsed!")
        // }
        
        // cells.remove_sets(coords, &values);


        // Down and to the right cannot be higher than the cell
        // let mut coords = Vec::new();
        // for x in coord.0..cells.layout().x() {
        //     for y in coord.1..cells.layout().y() {
        //         coords.push((x,y));
        //     }
        // }
        // let mut values = HashSet::new();
        // if let Some(Cell::Collapsed(value)) = cells.get_cell(coord){
        //     for value in *value+1..20 {
        //         values.insert(value);
        //     }
        // }else{
        //     eprintln!("Coord: {:?}", coord);
        //     eprintln!("Index: {:?}", cells.layout().get_index(coord));
        //     eprintln!("Cells: {:?}", cells);
        //     panic!("Coord was found to be uncollapsed!")
        // }
        
        // cells.remove_sets(coords, &values);
        
    }
}
