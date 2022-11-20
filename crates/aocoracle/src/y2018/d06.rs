use std::str::FromStr;

use crate::itersum::unambiguous_argmin;
use crate::rect::Rectangle;
use anyhow::anyhow;
use hashbrown::HashSet;
use itertools::{Itertools, MinMaxResult};
fn bounding_box(coordinates: &[(usize, usize)]) -> Rectangle<usize> {
    let (left, right) = match coordinates.iter().map(|(x, _)| x).minmax() {
        MinMaxResult::NoElements => {
            panic!("Expected at least one coordinate")
        }
        MinMaxResult::OneElement(x) => (*x, *x),
        MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
    };
    let (top, bottom) = match coordinates.iter().map(|(_, y)| y).minmax() {
        MinMaxResult::NoElements => {
            panic!("Expected at least one coordinate")
        }
        MinMaxResult::OneElement(y) => (*y, *y),
        MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
    };
    let width = right - left + 1;
    let height = bottom - top + 1;
    Rectangle::<usize>::new(left, top, width, height)
}

struct Grid<T> {
    shape: Rectangle<usize>,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn from_coordinates<F>(coordinates: &[(usize, usize)], f: F) -> Self
    where
        F: Fn((usize, usize)) -> T,
    {
        let shape = bounding_box(coordinates);
        let data = shape.tiles().map(f).collect();
        Self { shape, data }
    }

    fn border_values(&mut self) -> impl Iterator<Item = &T> {
        let mut result = Vec::with_capacity(2 * self.shape.width + 2 * self.shape.height - 2);
        for col in 0..self.shape.width {
            for row in [0, self.shape.height - 1] {
                result.push(&self.data[col + row * self.shape.width]);
            }
        }
        for row in 0..self.shape.height {
            for col in [0, self.shape.width - 1] {
                result.push(&self.data[col + row * self.shape.width]);
            }
        }
        result.into_iter()
    }
}

impl Grid<Option<u8>> {
    fn _print(&self, coordinates: &[(usize, usize)]) {
        let coordinates: HashSet<_> = coordinates.iter().collect();
        let mut line = String::with_capacity(self.shape.width);
        for (i, label) in self.data.iter().enumerate() {
            let x = self.shape.left + i % self.shape.width;
            let y = self.shape.top + i / self.shape.width;
            if x == self.shape.left {
                println!("{}", line);
                line.clear();
            }
            line.push(match label {
                None => '.',
                Some(label) => {
                    if coordinates.contains(&(x, y)) {
                        (label + 65) as char
                    } else {
                        (label + 97) as char
                    }
                }
            });
        }
        println!("{}", line);
    }
}

#[derive(Debug)]
struct Input {
    pub coordinates: Vec<(usize, usize)>,
}

impl Input {
    fn try_part_one(&self) -> anyhow::Result<usize> {
        let mut grid = Grid::from_coordinates(&self.coordinates, |(x, y)| {
            unambiguous_argmin(
                self.coordinates
                    .iter()
                    .enumerate()
                    .map(|(l, c)| (Some(l as u8), x.abs_diff(c.0) + y.abs_diff(c.1))),
            )
            .unwrap_or(None)
        });

        let infinite_areas: HashSet<u8> = grid.border_values().flatten().cloned().collect();

        let mut finite_areas = grid.data.into_iter().flatten().counts();
        finite_areas.retain(|k, _| !infinite_areas.contains(k));

        finite_areas
            .into_values()
            .max()
            .ok_or_else(|| anyhow!("All points are equidistant"))
    }

    fn part_two_x(&self, threshold: usize) -> usize {
        let grid = Grid::from_coordinates(&self.coordinates, |(x, y)| {
            self.coordinates
                .iter()
                .map(|c| x.abs_diff(c.0) + y.abs_diff(c.1))
                .sum::<usize>()
        });
        grid.data.into_iter().filter(|d| *d < threshold).count()
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
