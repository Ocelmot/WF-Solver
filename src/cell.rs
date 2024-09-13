use std::{
    collections::HashMap, fmt::{Debug, Display}, hash::Hash
};

/// Trait that indicates a type can be used as the value in a [Cell].
/// 
/// CellValues must be `PartialEq + Eq + Hash` because they are used as keys in
/// a HashMap, and they must be `Copy` since CellValues must be copied many
/// times and should not be complex.
pub trait CellValue: Copy + PartialEq + Eq + Hash {}
impl<T: Copy + PartialEq + Eq + Hash> CellValue for T {}

/// A Cell that contains values for the wavefunction collapse algorithm.
/// 
/// A Cell may be Collapsed or Uncollapsed. If the Cell is Collapsed, it will
/// contain a single [CellValue]. If it is Uncollapsed it will contain an
/// arbitrairy number of [CellValue]s along with thier associated chance of
/// occuring.
/// 
/// The chance is represented as a usize. The actual chance this represents is
/// `chance / sum(chances)`. This way as chances are added, existing chances do
/// not have to be recalculated.
#[derive(Clone)]
pub enum Cell<V: CellValue> {
    /// The cell is in a fully certain state with exactly one [CellValue].
    Collapsed(V),

    /// The cell is uncollapsed and could take any of the values in the hashmap.
    Uncollapsed(HashMap<V, usize>),
}

impl<V: CellValue> Cell<V> {
    /// Returns true if the Cell is collapsed.
    pub fn is_collapsed(&self) -> bool {
        if let Cell::Collapsed(_) = self {
            true
        } else {
            false
        }
    }

    /// Calculates the Shannon entropy of the cell. If the cell is collapsed,
    /// this is 0.0. If the cell is uncollapsed, it is calculated from the
    /// weights associated with each possibility.
    pub fn entropy(&self) -> f64 {
        match self {
            Cell::Collapsed(_) => 0.0,
            Cell::Uncollapsed(possibilities) => {
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

    /// Convert the cell into a [Cell::Collapsed] that contains the given value.
    /// 
    /// Returns true if either the cell was already collapsed to this value, or
    /// if the value was in the set of uncollapsed possibilities.
    /// 
    /// Returns false if the given value was not in the set of uncollapsed
    /// possibilities or the cell was already collapsed to a different value.
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

    /// Returns a HashMap of possibilities this cell could collapse to. If the
    /// cell is already collapsed, it returns an empty HashMap.
    pub fn get_possibilities(&mut self) -> HashMap<V, usize> {
        match self {
            Cell::Collapsed(_) => HashMap::new(),
            Cell::Uncollapsed(possibilities) => possibilities.clone(),
        }
    }

    /// Makes the cell into [Cell::Uncollapsed] with the given set of possibilities.
    pub fn set_possibilities(&mut self, possibilities: HashMap<V, usize>) {
        *self = Cell::Uncollapsed(possibilities);
    }

    /// If uncollapsed, adds a possibility to the set of possibilities with a
    /// weight of 1
    /// 
    /// If the possibility is already in the set, 1 is added to the existing
    /// weight
    pub fn add_possibility(&mut self, possibility: &V) {
        if let Self::Uncollapsed(values) = self {
            let value = values.entry(*possibility).or_insert(0);
            *value = *value + 1;
        }
    }

    /// If uncollapsed, adds a possibility to the set of possibilities with the
    /// given weight
    /// 
    /// If the possibility is already in the set, the weight is added to the
    /// existing weight
    pub fn add_possibility_count(&mut self, possibility: &V, weight: usize) {
        if let Self::Uncollapsed(values) = self {
            let value = values.entry(*possibility).or_insert(0);
            *value = *value + weight;
        }
    }

    /// If uncollapsed, adds the possibilities to the set of possibilities with
    /// the associated weights
    /// 
    /// If the possibility is already in the set, the weight is added to the
    /// existing weight
    pub fn add_possibilities(&mut self, possibilities: &HashMap<V, usize>) {
        if let Self::Uncollapsed(values) = self {
            for (possibility, count) in possibilities {
                let value = values.entry(*possibility).or_insert(0);
                *value = *value + count;
            }
        }
    }

    /// If uncollapsed, removes a possibility from the set of possibilities
    /// 
    /// The weight is always removed outright
    pub fn remove_possibility(&mut self, possibility: &V) {
        if let Self::Uncollapsed(values) = self {
            values.remove(possibility);
        }
    }

    /// If uncollapsed, reduces the weight of a possibility by the given amount.
    /// 
    /// If the weight would be zero or below, the possibility is removed.
    pub fn remove_possibility_count(&mut self, possibility: &V, weight: usize) {
        if let Self::Uncollapsed(values) = self {
            if let Some(value) = values.get_mut(possibility) {
                *value = value.saturating_sub(weight);
                if *value == 0 {
                    values.remove(possibility);
                }
            }
        }
    }

    /// If uncollapsed, reduces the weight of the possibilities by the
    /// associated amount in the HashMap.
    /// 
    /// If the weight would be zero or below, the possibility is removed.
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
