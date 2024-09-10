use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait CellValue: Copy + PartialEq + Eq + Hash {}
impl<T: Copy + PartialEq + Eq + Hash> CellValue for T {}

#[derive(Clone)]
pub enum Cell<V: CellValue> {
    Collapsed(V),
    Uncollapsed(HashMap<V, usize>),
}

impl<V: CellValue> Cell<V> {
    pub fn is_collapsed(&self) -> bool {
        if let Cell::Collapsed(_) = self {
            true
        } else {
            false
        }
    }

    pub fn entropy(&self) -> f64 {
        match self {
            Cell::Collapsed(_) => 0.0,
            Cell::Uncollapsed(possibilities) => {
                // possibilities.len() as f64 // Simple unweighted count
                let total = possibilities
                    .iter()
                    .fold(0.0, |acc, (_, chance)| acc + *chance as f64);
                let mut entropy = 0.0;
                for (_possibility, chance) in possibilities {
                    let probability = *chance as f64 / total;
                    entropy += probability * probability.log2();
                }
                -entropy
            }
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
                let contained = values.contains_key(&value);
                *self = Cell::Collapsed(value);
                contained
            }
        }
    }

    pub fn get_possibilities(&mut self) -> HashMap<V, usize> {
        match self {
            Cell::Collapsed(_) => HashMap::new(),
            Cell::Uncollapsed(possibilities) => possibilities.clone(),
        }
    }

    pub fn set_possibilities(&mut self, possibilities: HashMap<V, usize>) {
        *self = Cell::Uncollapsed(possibilities);
    }

    pub fn add_possibility(&mut self, possibility: &V) {
        if let Self::Uncollapsed(values) = self {
            let value = values.entry(*possibility).or_insert(0);
            *value = *value + 1;
        }
    }

    pub fn add_possibility_count(&mut self, possibility: &V, count: usize) {
        if let Self::Uncollapsed(values) = self {
            let value = values.entry(*possibility).or_insert(0);
            *value = *value + count;
        }
    }

    pub fn add_possibilities(&mut self, possibilities: &HashMap<V, usize>) {
        if let Self::Uncollapsed(values) = self {
            for (possibility, count) in possibilities {
                let value = values.entry(*possibility).or_insert(0);
                *value = *value + count;
            }
        }
    }

    pub fn remove_possibility(&mut self, possibility: &V) {
        if let Self::Uncollapsed(values) = self {
            values.remove(possibility);
        }
    }

    pub fn remove_possibility_count(&mut self, possibility: &V, count: usize) {
        if let Self::Uncollapsed(values) = self {
            if let Some(value) = values.get_mut(possibility) {
                *value = value.saturating_sub(count);
                if *value == 0 {
                    values.remove(possibility);
                }
            }
        }
    }

    pub fn remove_possibilities(&mut self, possibilities: &HashMap<V, usize>) {
        if let Self::Uncollapsed(values) = self {
            for (possibility, count) in possibilities {
                if let Some(value) = values.get_mut(possibility) {
                    *value = value.saturating_sub(*count);
                    if *value == 0 {
                        values.remove(possibility);
                    }
                }
            }
        }
    }
}

impl<V> Debug for Cell<V>
where
    V: Debug + CellValue,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Collapsed(value) => write!(f, "C{{{:?}}}", value)?,
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
            }
        }
        Ok(())
    }
}

impl<V> Display for Cell<V>
where
    V: Display + CellValue,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Collapsed(value) => {
                write!(f, "{}", value)
            }
            Cell::Uncollapsed(values) => {
                let vals: Vec<_> = values.iter().map(|(e, _count)| e.to_string()).collect();
                write!(f, "{{{}}}", vals.join("|"))
            }
        }
    }
}
