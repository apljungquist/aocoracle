use anyhow::anyhow;
use hashbrown::HashSet;
use itertools::Itertools;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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
            .ok_or(anyhow!("Could not capture line {}", line))?;
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

pub fn part_1x(input: &str, tgt: i32) -> anyhow::Result<usize> {
    let input = parsed(input)?;
    let mut coverage = HashSet::new();
    let mut beacons = HashSet::new();
    for (s, b) in input {
        if b.y == tgt {
            beacons.insert(b.x);
        }
        let r = s.manhattan(&b) as i32;
        let rx = r - s.y.abs_diff(tgt) as i32;
        if rx < 0 {
            continue;
        }
        let x_first = s.x - rx;
        let x_last = s.x + rx;
        dbg!((s, b, r, rx, x_first, x_last));
        for x in x_first..=x_last {
            coverage.insert(x);
        }
    }
    let coverage: Vec<_> = coverage.difference(&beacons).collect();
    Ok(coverage.len())
}

pub fn part_1a(input: &str) -> anyhow::Result<usize> {
    part_1x(input, 10)
}

pub fn part_1b(input: &str) -> anyhow::Result<usize> {
    part_1x(input, 2000000)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1a, "example", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1b, "6bb0c0bd67", Part::One);
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
        assert_error_on_wrong_input!(part_1a, part_2);
    }
}
