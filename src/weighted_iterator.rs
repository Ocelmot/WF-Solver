use std::{collections::HashMap, hash::Hash};

use rand::Rng;

pub(crate) struct WeightedIterator<T: Hash + Eq> {
    items: Vec<(T, usize)>,
    total_sum: usize,
}

impl<T: Hash + Eq> WeightedIterator<T> {
    pub fn new(map: HashMap<T, usize>) -> Self {
        let total_sum = map.iter().map(|(_item, chance)| chance).sum();
        Self {
            items: map.into_iter().collect(),
            total_sum,
        }
    }
}

impl<T: Hash + Eq> Iterator for WeightedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chosen_index = None;
        let mut cumulation = 0;
        let selection = rand::thread_rng().gen_range(0..self.total_sum);
        for (index, chance) in self.items.iter().map(|(_item, chance)| chance).enumerate() {
            cumulation += chance;
            if cumulation >= selection {
                chosen_index = Some(index);
                break;
            }
        }

        let (item, _chance) = self.items.swap_remove(chosen_index?);
        Some(item)
    }
}
