/// Functions for summarizing iterators into a constant number of values
use hashbrown::HashMap;
use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Input is empty")]
    TooFew,
    #[error("Output is ambiguous")]
    TooMany,
}

pub trait Itersum: Iterator {
    fn mode(self) -> Result<Self::Item, AggregationError>
    where
        Self: Sized,
        Self::Item: Copy + Eq + Hash,
    {
        let mut counts: HashMap<Self::Item, usize> = HashMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += 1);

        let best = counts.iter().max_by_key(|&(_, count)| count);
        let best = match best {
            Some(v) => v,
            None => return Err(AggregationError::TooFew),
        };

        for (k, v) in counts.iter() {
            if best.0 != k && best.1 == v {
                return Err(AggregationError::TooMany);
            }
        }

        Ok(*best.0)
    }
}
impl<T: ?Sized> Itersum for T where T: Iterator {}
