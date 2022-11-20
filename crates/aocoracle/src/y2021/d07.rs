use crate::AnyError;
use itertools::Itertools;
use std::collections::HashMap;

type Census = HashMap<u32, u64>;

fn _census(line: &str) -> Result<Census, AnyError> {
    let mut result = HashMap::new();
    for countdown in line.trim_end().split(',') {
        let countdown = countdown.parse::<u32>()?;
        *(result.entry(countdown).or_insert(0)) += 1;
    }
    match *result
        .keys()
        .max()
        .ok_or("Expected at least 1 fish but got 0")?
    {
        v if v < 9 => {
            Err(format!("Righmost position is {}, this is probably day 6 input", v).into())
        }
        _ => Ok(result),
    }
}

fn _median(census: &HashMap<u32, u64>) -> u32 {
    let mut countdown = (census.values().sum::<u64>() / 2) as i32;
    for k in census.keys().sorted() {
        countdown -= *census.get(k).unwrap() as i32;
        if countdown <= 0 {
            return *k;
        }
    }
    panic!("Huh?")
}

fn _linear_cost(census: &HashMap<u32, u64>, location: u32) -> u64 {
    census
        .iter()
        .map(|(k, v)| (location as i32 - *k as i32).unsigned_abs() as u64 * v)
        .sum()
}

fn _quadratic_cost(census: &HashMap<u32, u64>, location: u32) -> u64 {
    census
        .iter()
        .map(|(k, v)| {
            let d = (location as i32 - *k as i32).unsigned_abs() as u64;
            (d + 1) * d / 2 * v
        })
        .sum()
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let census = _census(input)?;
    let cost = _linear_cost(&census, _median(&census));
    Ok(format!("{}", cost))
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let census = _census(input)?;
    let (min, max) = match census.keys().minmax() {
        itertools::MinMaxResult::NoElements => panic!("No elements"),
        itertools::MinMaxResult::OneElement(only) => (*only, *only),
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
    };
    let cost = (min..=max)
        .map(|location| _quadratic_cost(&census, location))
        .min()
        .unwrap() as u64;
    Ok(format!("{}", cost))
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

    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(&part_1, &part_2);
    }
}
