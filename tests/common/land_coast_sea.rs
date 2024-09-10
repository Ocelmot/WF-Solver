use std::fmt::Display;
use colored::Colorize;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum LandCoastSea {
    Land,
    Coast,
    Sea,
}

impl Display for LandCoastSea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LandCoastSea::Land => write!(f, "{}", "L".green()),
            LandCoastSea::Coast => write!(f, "{}", "C".yellow()),
            LandCoastSea::Sea => write!(f, "{}", "S".blue()),
        }
    }
}
