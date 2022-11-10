use std::cmp::Ordering;
use std::str::FromStr;

use anyhow::anyhow;
use hashbrown::{HashMap, HashSet};
use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct Input {
    coordinates: Vec<(usize, usize)>,
}

impl Input {
    fn _print_grid(&self, grid: &HashMap<(usize, usize), (usize, Option<u8>)>) {
        let coordinates: HashSet<_> = self.coordinates.iter().collect();
        let left = *grid.keys().map(|(x, _)| x).min().unwrap();
        let right = *grid.keys().map(|(x, _)| x).max().unwrap();
        let top = *grid.keys().map(|(_, y)| y).min().unwrap();
        let bottom = *grid.keys().map(|(_, y)| y).max().unwrap();
        for y in top..=bottom {
            let line: String = (left..=right)
                .map(|x| match grid.get(&(x, y)) {
                    None => '?',
                    Some((_, None)) => '.',
                    Some((_, Some(label))) => {
                        if coordinates.contains(&(x, y)) {
                            (label + 65) as char
                        } else {
                            (label + 97) as char
                        }
                    }
                })
                .collect();
            println!("{}", line)
        }
    }

    fn try_part_one(&self) -> anyhow::Result<usize> {
        let (left, right) = match self.coordinates.iter().map(|(x, _)| x).minmax() {
            MinMaxResult::NoElements => {
                panic!("Expected at least one coordinate")
            }
            MinMaxResult::OneElement(x) => (*x, *x),
            MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
        };
        let (top, bottom) = match self.coordinates.iter().map(|(_, y)| y).minmax() {
            MinMaxResult::NoElements => {
                panic!("Expected at least one coordinate")
            }
            MinMaxResult::OneElement(x) => (*x, *x),
            MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
        };
        let mut grid: HashMap<(usize, usize), (usize, Option<u8>)> = HashMap::new();
        for x in left..=right {
            for y in top..=bottom {
                for (new_label, c) in self.coordinates.iter().enumerate() {
                    let new_distance = x.abs_diff(c.0) + y.abs_diff(c.1);
                    match grid.get(&(x, y)) {
                        None => {
                            grid.insert((x, y), (new_distance, Some(new_label as u8)));
                        }
                        Some((old_distance, _)) => match new_distance.cmp(old_distance) {
                            Ordering::Less => {
                                grid.insert((x, y), (new_distance, Some(new_label as u8)));
                            }
                            Ordering::Equal => {
                                grid.insert((x, y), (new_distance, None));
                            }
                            Ordering::Greater => {}
                        },
                    }
                }
            }
        }

        let mut infinites = HashSet::new();
        for x in left..=right {
            for y in [top, bottom] {
                if let Some((_, Some(label))) = grid.get(&(x, y)) {
                    infinites.insert(label);
                }
            }
        }
        for y in top..=bottom {
            for x in [left, right] {
                if let Some((_, Some(label))) = grid.get(&(x, y)) {
                    infinites.insert(label);
                }
            }
        }

        let mut finite_areas = grid.values().flat_map(|(_, l)| l).counts();
        finite_areas.retain(|k, _| !infinites.contains(k));

        finite_areas
            .into_values()
            .max()
            .ok_or_else(|| anyhow!("All points are equidistant"))
    }

    fn part_two_x(&self, threshold: usize) -> usize {
        let (left, right) = match self.coordinates.iter().map(|(x, _)| x).minmax() {
            MinMaxResult::NoElements => {
                panic!("Expected at least one coordinate")
            }
            MinMaxResult::OneElement(x) => (*x, *x),
            MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
        };
        let (top, bottom) = match self.coordinates.iter().map(|(_, y)| y).minmax() {
            MinMaxResult::NoElements => {
                panic!("Expected at least one coordinate")
            }
            MinMaxResult::OneElement(x) => (*x, *x),
            MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
        };
        let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
        for x in left..=right {
            for y in top..=bottom {
                for c in self.coordinates.iter() {
                    let marginal_distance = x.abs_diff(c.0) + y.abs_diff(c.1);
                    *grid.entry((x, y)).or_default() += marginal_distance;
                }
            }
        }
        grid.drain_filter(|_, d| *d < threshold).count()
    }

    fn _part_two_a(&self) -> usize {
        self.part_two_x(32)
    }
    fn part_two_b(&self) -> usize {
        self.part_two_x(10000)
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coordinates = Vec::new();
        let re = regex::Regex::new(r"^(\d+), (\d+)$").expect("Hard coded regex is valid");
        for line in s.lines() {
            let cap = re
                .captures(line)
                .ok_or_else(|| anyhow!("Regex \"{re:?}\" could not capture line {line:?}"))?;
            coordinates.push((cap[1].parse()?, cap[2].parse()?))
        }
        Ok(Self { coordinates })
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    Ok(Input::from_str(input)?.try_part_one()?.to_string())
}

pub fn _part_2a(input: &str) -> anyhow::Result<String> {
    Ok(Input::from_str(input)?._part_two_a().to_string())
}
pub fn part_2b(input: &str) -> anyhow::Result<String> {
    Ok(Input::from_str(input)?.part_two_b().to_string())
}

#[cfg(test)]
mod tests {
    use crate::testing::{actual_answer2, assert_returns_error_on_wrong_input2, expected_answer};
    use crate::Part;

    use super::*;

    fn assert_correct_answer(part: Part, stem: &str) {
        assert_eq!(
            actual_answer2(
                file!(),
                match part {
                    Part::One => part_1,
                    Part::Two => match stem {
                        "example" => _part_2a,
                        _ => part_2b,
                    },
                },
                stem,
            ),
            expected_answer(file!(), part, stem).unwrap(),
        )
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer(Part::One, "example");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer(Part::One, "3ba7923eae");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer(Part::Two, "example");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(Part::Two, "3ba7923eae");
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_returns_error_on_wrong_input2(file!(), &part_1, &part_2b);
    }
}
