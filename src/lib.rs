


mod solver;
pub use solver::Solver;

mod cell;
pub use cell::CellValue;

mod layout;
pub use layout::{Layout, Grid};

mod wavefunction;
pub use wavefunction::Wavefunction;


#[cfg(test)]
mod tests {
    use crate::{cell::Cell, wavefunction::grid_test::{GridTest, TileTypes}, Grid, Solver};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    #[test]
    fn grid_test_results() {
        let x = 10;
        let y = 10;
        let layout = Grid::new(x, y);
        let wavefunction = GridTest{};
        let mut solver = Solver::new(layout, wavefunction);
        let output = solver.solve();
        
        if let Some(cells) = output {
            for i in 0..x {
                for j in 0..y{
                    let cell = cells.get((j*y)+i).unwrap();
                    match cell {
                        Cell::Collapsed(TileTypes::Land) => print!("L"),
                        Cell::Collapsed(TileTypes::Coast) => print!("C"),
                        Cell::Collapsed(TileTypes::Sea) => print!("S"),
                        Cell::Uncollapsed(v) => print!("<Uncollapsed:{:?}>",v),
                    }
                    print!(", ")
                }
                print!("\n");
            }
        }else{
            println!("No solution");
        }
        

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

}
