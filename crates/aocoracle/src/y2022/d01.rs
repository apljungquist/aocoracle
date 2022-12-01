use anyhow::bail;
use itertools::Itertools;

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
    let inventory_totals: Vec<_> = inventories(input)?
        .into_iter()
        .map(|inventory| inventory.iter().sum::<u32>())
        .sorted()
        .rev()
        .collect();
    Ok(inventory_totals
        .first()
        .expect("Parsing ensures there are at least three inventories")
        + inventory_totals
            .get(1)
            .expect("Parsing ensures there are at least three inventories")
        + inventory_totals
            .get(2)
            .expect("Parsing ensures there are at least three inventories"))
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "example", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "6bb0c0bd67", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6bb0c0bd67", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
