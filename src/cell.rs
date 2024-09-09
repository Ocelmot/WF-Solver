use std::{collections::HashSet, fmt::{Debug, Display}, hash::Hash};

pub trait CellValue: Copy + PartialEq + Eq + Hash {}
impl<T: Copy + PartialEq + Eq + Hash> CellValue for T {}

#[derive(Clone)]
pub enum Cell<V: CellValue> {
    Collapsed(V),
    Uncollapsed(HashSet<V>),
}

impl<V: CellValue> Cell<V> {
    pub fn is_collapsed(&self) -> bool {
        if let Cell::Collapsed(_) = self {
            true
        } else {
            false
        }
    }

    pub fn collapse(&mut self, value: V) -> bool {
        match self {
            Cell::Collapsed(old_value) => {
                if *old_value == value {
                    true
                } else {
                    *self = Cell::Collapsed(value);
                    false
                }
            }
            Cell::Uncollapsed(values) => {
                let contained = values.contains(&value);
                *self = Cell::Collapsed(value);
                contained
            }
        }
    }

    pub fn get_possibilities(&mut self) -> HashSet<V> {
        match self {
            Cell::Collapsed(_) => HashSet::new(),
            Cell::Uncollapsed(possibilities) => possibilities.clone(),
        }
    }

    pub fn set_possibilities(&mut self, possibilities: HashSet<V>) {
        *self = Cell::Uncollapsed(possibilities);
    }

    pub fn add_possibility(&mut self, possibility: &V) {
        if let Self::Uncollapsed(values) = self {
            values.insert(*possibility);
        }
    }

    pub fn add_possibilities(&mut self, possibilities: &HashSet<V>) {
        if let Self::Uncollapsed(values) = self {
            *values = &*values | possibilities;
        }
    }

    pub fn remove_possibility(&mut self, possibility: &V) {
        if let Self::Uncollapsed(values) = self {
            values.remove(possibility);
        }
    }

    pub fn remove_possibilities(&mut self, possibilities: &HashSet<V>) {
        if let Self::Uncollapsed(values) = self {
            *values = &*values - possibilities;
        }
    }
}


impl<V> Debug for Cell<V> where V: Debug + CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Collapsed(value) => {
                write!(f, "C{{{:?}}}", value)?
            },
            Cell::Uncollapsed(values) => {
                write!(f, "U{{")?;
                let total = values.len();
                for (index, value) in values.iter().enumerate() {
                    write!(f, "{:?}", value)?;
                    if index < total - 1 {
                        write!(f, "|")?;
                    }
                }
                write!(f, "}}")?;
            },
        }
        Ok(())
    }
}

impl<V> Display for Cell<V> where V: Display + CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Collapsed(value) => {
                write!(f, "{}", value)
            },
            Cell::Uncollapsed(values) => {
                let vals: Vec<_> = values.iter().map(|e|{e.to_string()}).collect();
                write!(f, "{{{}}}", vals.join("|"))
            },
        }
    }
}