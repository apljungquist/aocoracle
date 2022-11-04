/// Functions for summarizing iterators into a constant number of values
use hashbrown::HashMap;
use std::cmp::Ordering;
use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Input is empty")]
    TooFew,
    #[error("Output is ambiguous")]
    TooMany,
}

pub fn mode<T: Copy + Eq + Hash>(values: impl Iterator<Item = T>) -> Result<T, AggregationError> {
    let mut counts = HashMap::new();
    for v in values {
        let count = counts.entry(v).or_insert(0);
        // This bothers me; surely I should not be able to increment an immutable value...
        *count += 1;
    }
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

pub fn unambiguous_argmax<KT, VT, Iter>(mut items: Iter) -> Result<KT, AggregationError>
where
    VT: Copy + Ord,
    Iter: Iterator<Item = (KT, VT)>,
{
    // Can be simplified if avoiding ambiguity is not important:
    // items.max_by_key(|(_, v)| *v).map(|(k, _)| k)
    let mut ambiguous = false;
    let mut best = items.next().ok_or(AggregationError::TooFew)?;
    for item in items {
        match best.1.cmp(&item.1) {
            Ordering::Less => {
                best = item;
                ambiguous = false;
            }
            Ordering::Equal => {
                ambiguous = true;
            }
            Ordering::Greater => {}
        }
    }
    if ambiguous {
        return Err(AggregationError::TooMany);
    }
    Ok(best.0)
}
