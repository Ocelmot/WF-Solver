use super::Layout;

#[derive(Debug)]
pub struct Grid {
    x: usize,
    y: usize,
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn row(&self, y: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for x in 0..self.x {
            v.push((x, y));
        }
        v
    }

    pub fn col(&self, x: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for y in 0..self.y {
            v.push((x, y));
        }
        v
    }

    pub fn all(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::with_capacity(self.x * self.y);
        for x in 0..self.x {
            for y in 0..self.y {
                v.push((x, y))
            }
        }
        v
    }

    pub fn neighbors(&self, coord: (usize, usize)) -> Vec<(usize, usize)>{
        let mut v = Vec::new();
        v.push((coord.0.saturating_sub(1), coord.1.saturating_sub(1)));
        v.push((coord.0, coord.1.saturating_sub(1)));
        v.push((coord.0+1, coord.1.saturating_sub(1)));

        v.push((coord.0.saturating_sub(1), coord.1));
        v.push((coord.0+1, coord.1));

        v.push((coord.0.saturating_sub(1), coord.1+1));
        v.push((coord.0, coord.1+1));
        v.push((coord.0+1, coord.1+1));
        v
    }
}

impl Layout for Grid {
    type Coordinate = (usize, usize);

    fn size(&mut self) -> usize {
        self.x * self.y
    }

    fn get_coord(&mut self, index: usize) -> Self::Coordinate {
        (index % self.x, index / self.x)
    }

    fn get_index(&mut self, coord: Self::Coordinate) -> usize {
        coord.1 * self.x + coord.0
    }
}
