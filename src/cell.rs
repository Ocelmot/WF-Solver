use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

/// Describes which parts of the overlaping sets of possibilities should be in
/// the cell after the merge operation.
///
/// This table describes which operations will leave what parts in the cell.
/// Possibilities in the middle row will be further processed to combine thier
/// weights.
/// ```text
///                 | U | M | X | S | R | I | E | N |
/// Cell only       | x | x | x | x |   |   |   |   |
/// Cell and Param  | x | x |   |   | x | x |   |   |
/// Param only      | x |   | x |   | x |   | x |   |
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    /// Include the elements from the cell, the merged weights, and the overlap.
    Union,
    /// Include the elements from the cell including the overlap, but not the weights.
    Modification,
    /// Include the elements from the cell, the merged weights, but not the overlap.
    Xor,
    /// Include the elements from the cell, but not the merged weights or the overlap.
    Subtraction,
    /// Only include the elements from the merged weights and the overlap, but not the cell.
    Replacement,
    /// Only include the elements from the overlap, but not the cell or the merged weights.
    Intersection,
    /// Only include the elements from the merged weights, not from the overlap, or the cell.
    ExclusiveReplacement,
    /// Include no elements.
    Null,
}

/// Determines how two weights are combined. The first weight is what exists in
/// the cell already, the second is the modifier to be added.
#[derive(Clone, Copy, Debug)]
pub enum Function {
    /// The minimum of the two weights.
    Min,
    /// The maximum of the two weights.
    Max,
    /// Use the weight from the cell.
    A,
    /// The the weight from the merged weights.
    B,
    /// Multiply the weights together (Unimplimented).
    Multiply,
    /// The sum of the two weights.
    Add,
    /// The difference of the two weights.
    Subtract,
}

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

    /// Gets the value if the cell is collapsed, None otherwise.
    pub fn get_value(&mut self) -> Option<V> {
        match self {
            Cell::Collapsed(value) => Some(*value),
            Cell::Uncollapsed(_) => None,
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

    /// Merges another map of weights into this cell's map of weights.
    /// 
    /// The merge takes two enums to describe how it will perform the operation
    /// on the data.
    /// 
    /// The first is [Operation], which describes which parts of the overlap
    /// between the original weights and the new weights will remain in the
    /// cell. For example, if the operation is [Operation::Union], the resulting
    /// weights will include the original weights as well as all of the new
    /// weights. However, if the operation is [Operation::Subtraction], any
    /// weights that appeared in the new weights will not be included in the
    /// result at all.
    /// 
    /// The second is [Function], which describes how the overlaping weights
    /// will be combined.  For example, if the function is [Function::Max], the
    /// larger of the two weights will be used.
    pub fn merge_cell_possibilities(
        &mut self,
        op: Operation,
        func: Function,
        weights: &HashMap<V, usize>,
    ) {
        let Self::Uncollapsed(cell_weights) = self else {
            return;
        };
        cell_weights.retain(|value, weight| match op {
            Operation::Union => {
                if let Some(other_weight) = weights.get(value) {
                    merge_possibility(func, weight, other_weight);
                }
                true
            }
            Operation::Modification => {
                if let Some(other_weight) = weights.get(value) {
                    merge_possibility(func, weight, other_weight);
                }
                true
            }
            Operation::Xor => !weights.contains_key(value),
            Operation::Subtraction => !weights.contains_key(value),
            Operation::Replacement => {
                if let Some(other_weight) = weights.get(value) {
                    merge_possibility(func, weight, other_weight);
                    true
                } else {
                    false
                }
            }
            Operation::Intersection => {
                if let Some(other_weight) = weights.get(value) {
                    merge_possibility(func, weight, other_weight);
                    true
                } else {
                    false
                }
            }
            Operation::ExclusiveReplacement => false,
            Operation::Null => false,
        });

        // For operations that include new weights from the params, add them here
        match op {
            Operation::Union
            | Operation::Xor
            | Operation::Replacement
            | Operation::ExclusiveReplacement => {
                self.add_possibilities(&weights);
            }
            _ => return, // Other options do not care
        }
    }
}

fn merge_possibility(func: Function, first: &mut usize, second: &usize) {
    // let Some(second) = second else {return};
    match func {
        Function::Min => *first = min(*first, *second),
        Function::Max => *first = max(*first, *second),
        Function::A => {} // the item is already set to the first item
        Function::B => *first = *second,
        Function::Multiply => unimplemented!("This is more complicated"), // because it should turn it into a float [0, 1] before multiplication
        Function::Add => *first = *first + second,
        Function::Subtract => *first = first.saturating_sub(*second),
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
