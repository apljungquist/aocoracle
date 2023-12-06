use std::collections::BTreeMap;

use hashbrown::HashSet;
use itertools::Itertools;

struct Input {
    symbols: BTreeMap<(isize, isize), bool>,
    numbers: BTreeMap<(isize, isize), (usize, u32)>,
}

impl Input {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let re = regex::Regex::new(r"([0-9]+|[^0-9])").expect("Hard coded regex is valid");

        let mut next_number_id = 0;
        let mut numbers = BTreeMap::new();
        let mut symbols = BTreeMap::new();

        for (row_num, line) in input.lines().enumerate() {
            let row_num: isize = row_num.try_into()?;
            let mut col_num: isize = 0;
            for token in re.captures_iter(line) {
                let token = token
                    .get(1)
                    .expect("Hard coded regex always matches exactly one group")
                    .as_str();

                match token.parse::<u32>() {
                    Ok(number) => {
                        for _ in 0..token.len() {
                            numbers.insert((row_num, col_num), (next_number_id, number));
                            col_num += 1;
                        }
                        next_number_id += 1;
                    }
                    Err(e) => match e.kind() {
                        std::num::IntErrorKind::InvalidDigit => {
                            let token = token.chars().exactly_one().expect(
                                "Hard coded regex matches a valid number or a single symbol",
                            );
                            match token {
                                '.' => {}
                                '*' => {
                                    symbols.insert((row_num, col_num), true);
                                }
                                '#' | '$' | '%' | '&' | '+' | '-' | '/' | '=' | '@' => {
                                    symbols.insert((row_num, col_num), false);
                                }
                                _ => anyhow::bail!("Invalid symbol"),
                            }
                            col_num += 1;
                        }
                        _ => anyhow::bail!("Expected small numbers and symbols but got {}", token),
                    },
                }
            }
        }

        if numbers.is_empty() {
            anyhow::bail!("No numbers found");
        }
        if !symbols.values().any(|&could_be_gear| could_be_gear) {
            anyhow::bail!("No potential gear found");
        }

        Ok(Self { symbols, numbers })
    }

    fn adjacent_numbers(&self, row: isize, col: isize) -> Vec<(usize, u32)> {
        let mut adjacent_numbers = Vec::new();
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let row = row + dr;
                let col = col + dc;
                if let Some(number) = self.numbers.get(&(row, col)) {
                    adjacent_numbers.push(*number);
                }
            }
        }
        adjacent_numbers
    }
}

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let input = Input::parse(input)?;
    let mut used_ids = HashSet::new();
    let mut sum = 0;
    for ((r, c), _) in input.symbols.iter() {
        for (id, value) in input.adjacent_numbers(*r, *c) {
            if used_ids.insert(id) {
                sum += value;
            }
        }
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u32> {
    let input = Input::parse(input)?;

    let mut sum = 0;
    for ((r, c), could_be_gear) in input.symbols.iter() {
        if !could_be_gear {
            continue;
        }
        let adjacent_numbers: BTreeMap<usize, u32> =
            input.adjacent_numbers(*r, *c).into_iter().collect();
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers.values().product::<u32>();
        }
    }
    Ok(sum)
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
        assert_correct_answer_on_correct_input!("6107a576a7163737", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("6107a576a7163737", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
