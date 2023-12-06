use anyhow::{anyhow, bail};
use hashbrown::HashSet;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn parsed(text: &str) -> anyhow::Result<Vec<(Point, Point)>> {
    let re = regex::Regex::new(
        r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
    )
    .expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in text.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture line {}", line))?;
        result.push((
            Point {
                x: cap[1].parse()?,
                y: cap[2].parse()?,
            },
            Point {
                x: cap[3].parse()?,
                y: cap[4].parse()?,
            },
        ));
    }
    Ok(result)
}

pub fn part_1x(input: &str, tgt: i64) -> anyhow::Result<usize> {
    let input = parsed(input)?;
    let mut coverage = HashSet::new();
    let mut beacons = HashSet::new();
    for (s, b) in input {
        if b.y == tgt {
            beacons.insert(b.x);
        }
        let r = s.manhattan(&b) as i64;
        let rx = r - s.y.abs_diff(tgt) as i64;
        if rx < 0 {
            continue;
        }
        let x_first = s.x - rx;
        let x_last = s.x + rx;
        for x in x_first..=x_last {
            coverage.insert(x);
        }
    }
    let coverage = coverage.difference(&beacons).collect_vec();
    Ok(coverage.len())
}

pub fn _part_1a(input: &str) -> anyhow::Result<usize> {
    part_1x(input, 10)
}

pub fn part_1b(input: &str) -> anyhow::Result<usize> {
    part_1x(input, 2000000)
}

pub fn part_2x(input: &str, lo: i64, hi: i64) -> anyhow::Result<i64> {
    let input = parsed(input)?;
    let mut x = lo;
    let mut y = lo;
    'outer: while y <= hi {
        if hi < x {
            x = lo;
            y += 1;
        }
        for (s, b) in input.iter() {
            let r = s.manhattan(b) as i64;
            let rx = r - s.y.abs_diff(y) as i64;
            if rx < 0 {
                continue;
            }
            let x_first = s.x - rx;
            let x_last = s.x + rx;
            if x_first <= x && x <= x_last {
                x = x_last + 1;
                continue 'outer;
            }
        }
        return Ok(x * 4000000 + y);
    }
    bail!("Found no possible beacon locations");
}

pub fn _part_2a(input: &str) -> anyhow::Result<i64> {
    part_2x(input, 0, 20)
}

pub fn part_2b(input: &str) -> anyhow::Result<i64> {
    part_2x(input, 0, 4000000)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One, &_part_1a);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("f95773a9a6b7f551", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two, &_part_2a);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("f95773a9a6b7f551", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
