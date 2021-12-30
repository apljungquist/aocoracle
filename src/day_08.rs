use std::fs;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type AnyError = Box<dyn std::error::Error>;
type Key = HashMap<String, usize>;

fn _display(line: &str) -> Vec<Vec<String>> {
    line.split('|')
        .map(|vs| vs.split_whitespace().map(|p| p.chars().collect()).collect())
        .collect()
}

fn _displays(text: &str) -> Vec<Vec<Vec<String>>> {
    text.lines().map(_display).collect()
}

fn _key(patterns: &[String]) -> Key {
    let by_length: HashMap<usize, Vec<HashSet<char>>> = patterns
        .iter()
        .map(|v| (v.len(), v.chars().collect()))
        .into_group_map()
        .drain()
        .collect();

    let one = by_length.get(&2).unwrap().iter().exactly_one().unwrap();
    let four = by_length.get(&4).unwrap().iter().exactly_one().unwrap();
    let seven = by_length.get(&3).unwrap().iter().exactly_one().unwrap();
    let eight = by_length.get(&7).unwrap().iter().exactly_one().unwrap();

    let three = by_length
        .get(&5)
        .unwrap()
        .iter()
        .filter(|v| v.is_superset(one))
        .exactly_one()
        .unwrap();
    let six = by_length
        .get(&6)
        .unwrap()
        .iter()
        .filter(|v| !v.is_superset(one))
        .exactly_one()
        .unwrap();
    let b = four.difference(three).exactly_one().unwrap();

    let two = by_length
        .get(&5)
        .unwrap()
        .iter()
        .filter(|v| *v != three && !v.contains(b))
        .exactly_one()
        .unwrap();
    let five = by_length
        .get(&5)
        .unwrap()
        .iter()
        .filter(|v| *v != three && v.contains(b))
        .exactly_one()
        .unwrap();
    let e = six.difference(five).exactly_one().unwrap();

    let zero = by_length
        .get(&6)
        .unwrap()
        .iter()
        .filter(|v| *v != six && v.contains(e))
        .exactly_one()
        .unwrap();
    let nine = by_length
        .get(&6)
        .unwrap()
        .iter()
        .filter(|v| *v != six && !v.contains(e))
        .exactly_one()
        .unwrap();

    vec![zero, one, two, three, four, five, six, seven, eight, nine]
        .iter()
        .enumerate()
        .map(|(i, vs)| (vs.iter().sorted().collect(), i))
        .collect()
}

fn _decoded(digits: &[String], key: Key) -> u32 {
    let mut result = 0;
    digits
        .iter()
        .map(|v| key.get(&v.chars().sorted().collect::<String>()).unwrap())
        .for_each(|d| result = result * 10 + *d as u32);
    result
}

fn _cracked_and_decoded(train: &[String], test: &[String]) -> u32 {
    let key = _key(train);
    _decoded(test, key)
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let displays = _displays(input);
    let num_1478 = displays
        .iter()
        .map(|vs| vs.get(1).unwrap())
        .flatten()
        .filter(|v| matches!(v.len(), 2 | 3 | 4 | 7))
        .count();
    Ok(format!("{}", num_1478))
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let displays = _displays(input);
    let sum = displays
        .iter()
        .map(|d| _cracked_and_decoded(d.get(0).unwrap(), d.get(1).unwrap()))
        .sum::<u32>();
    Ok(format!("{}", sum))
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, AnyError>,
{
    func(&fs::read_to_string(format!("inputs/08/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example_s() {
        assert_eq!(_from_file(part_1, "example"), "26");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), "470");
    }

    #[test]
    fn part_2_works_on_example_l() {
        assert_eq!(_from_file(part_2, "example"), "61229");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), "989396");
    }
}
