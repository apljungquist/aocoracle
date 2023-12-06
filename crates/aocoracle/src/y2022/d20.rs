use anyhow::{anyhow, bail};

fn numbers(s: &str) -> anyhow::Result<Vec<i64>> {
    let re = regex::Regex::new(r"^(-?([1-9]\d*)|0)$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture number on line {}", line))?;
        result.push(cap[1].parse()?);
    }
    let num_zero = result.iter().filter(|n| **n == 0).count();
    if num_zero != 1 {
        bail!("Expected exactly 1 zero but got {num_zero}")
    }
    Ok(result)
}

fn move_number(mixed: &mut Vec<(usize, i64)>, id: usize) {
    let old_index = mixed
        .iter()
        .position(|x| x.0 == id)
        .expect("This function is only called with valid ids");
    let value = mixed.remove(old_index).1;
    let mut new_index = (old_index as i64 + value).rem_euclid(mixed.len() as i64) as usize;
    // Keep the first element the same as in example
    if new_index == 0 && value < 0 {
        new_index = mixed.len();
    }
    mixed.insert(new_index, (id, value));
}

fn part_x(numbers: &[i64], num_round: usize, key: i64) -> i64 {
    let mut mixed: Vec<(usize, i64)> = numbers
        .iter()
        .map(|value| value * key)
        .enumerate()
        .collect();
    for _ in 0..num_round {
        for id in 0..mixed.len() {
            move_number(&mut mixed, id);
        }
    }
    let origin = mixed
        .iter()
        .position(|x| x.1 == 0)
        .expect("Input validation ensures there is exactly 1 zero");
    [
        mixed[(origin + 1000) % mixed.len()].1,
        mixed[(origin + 2000) % mixed.len()].1,
        mixed[(origin + 3000) % mixed.len()].1,
    ]
    .iter()
    .sum()
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    Ok(part_x(&numbers, 1, 1))
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    Ok(part_x(&numbers, 10, 811589153))
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("6f67c62119a690c4", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("6f67c62119a690c4", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
