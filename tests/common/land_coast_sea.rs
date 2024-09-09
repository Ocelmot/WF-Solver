use std::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum LandCoastSea {
    Land,
    Coast,
    Sea,
}

impl Display for LandCoastSea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LandCoastSea::Land => write!(f, "L"),
            LandCoastSea::Coast => write!(f, "C"),
            LandCoastSea::Sea => write!(f, "S"),
        }
    }
}
