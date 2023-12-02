use anyhow::bail;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Return the `n` greatest items
pub trait TopN: Iterator {
    fn top(mut self, n: usize) -> Option<Vec<Self::Item>>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut result: BinaryHeap<Reverse<Self::Item>> = BinaryHeap::with_capacity(n);
        for _ in 0..n {
            result.push(Reverse(self.next()?));
        }
        for item in self {
            if result.peek()?.0 < item {
                result.pop();
                result.push(Reverse(item));
            }
        }
        Some(result.into_sorted_vec().into_iter().map(|r| r.0).collect())
    }
}

impl<T: ?Sized> TopN for T where T: Iterator {}
fn inventories(s: &str) -> anyhow::Result<Vec<Vec<u32>>> {
    let mut result = Vec::new();
    let mut inventory = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            result.push(inventory);
            inventory = Vec::new();
        } else {
            inventory.push(line.parse()?);
        }
    }
    result.push(inventory);
    if result.len() < 3 {
        bail!(
            "Expected at least three inventories but got {}",
            result.len()
        );
    }
    for inventory in result.iter() {
        if inventory.is_empty() {
            bail!("Expected every inventory to contain at least one item")
        }
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    Ok(inventories(input)?
        .into_iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .expect("Parsing ensures there is at least one inventory"))
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    Ok(inventories(input)?
        .into_iter()
        .map(|inventory| inventory.iter().sum::<u32>())
        .top(3)
        .expect("Parsing ensures there are at least three inventories")
        .into_iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "fb18049b20d06f1c", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "fb18049b20d06f1c", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
