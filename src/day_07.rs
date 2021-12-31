use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn _census(line: &str) -> HashMap<u32, u64> {
    let mut result = HashMap::new();
    for countdown in line.trim_end().split(',').map(|s| {
        s.parse::<u32>()
            .unwrap_or_else(|_| panic!("Expected an int but got '{}'", s))
    }) {
        *(result.entry(countdown).or_insert(0)) += 1;
    }
    result
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
        .map(|(k, v)| (location as i32 - *k as i32).abs() as u64 * v)
        .sum()
}

fn _quadratic_cost(census: &HashMap<u32, u64>, location: u32) -> u64 {
    census
        .iter()
        .map(|(k, v)| {
            let d = (location as i32 - *k as i32).abs() as u64;
            (d + 1) * d / 2 * v
        })
        .sum()
}

pub fn part_1(input: &str) -> u64 {
    let census = _census(input);
    _linear_cost(&census, _median(&census))
}

pub fn part_2(input: &str) -> u64 {
    let census = _census(input);
    let (min, max) = match census.keys().minmax() {
        itertools::MinMaxResult::NoElements => panic!("No elements"),
        itertools::MinMaxResult::OneElement(only) => (*only, *only),
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
    };
    (min..=max)
        .map(|location| _quadratic_cost(&census, location))
        .min()
        .unwrap() as u64
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("inputs/07/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 37);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 342641);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 168);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 93006301);
    }
}
