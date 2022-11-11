use std::cmp::Ordering;

use std::str::FromStr;

use anyhow::anyhow;
use hashbrown::HashSet;
use itertools::{Itertools, MinMaxResult};

struct Grid<T> {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn from_coordinates(coordinates: &[(usize, usize)], default: T) -> Self {
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
            MinMaxResult::OneElement(x) => (*x, *x),
            MinMaxResult::MinMax(lo, hi) => (*lo, *hi),
        };
        let width = right - left + 1;
        let height = bottom - top + 1;
        let area = width * height;
        Self {
            left,
            top,
            width,
            height,
            data: vec![default; area],
        }
    }

    // I wanted to try implementing this as an iterator with mutable references but I think this may
    // be impossible using safe rust.
    fn apply<F>(&mut self, f: F)
    where
        F: Fn(usize, usize, &mut T),
    {
        for (i, value) in self.data.iter_mut().enumerate() {
            let x = self.left + i % self.width;
            let y = self.top + i / self.width;
            f(x, y, value);
        }
    }

    // I wanted to try implementing this as an iterator but found no good way to track the state.
    // When collecting the indices to a vector it takes twice the time of the for each
    // implementation.
    // It seems it should be possible to avoid collecting the indices by using chain but so far I
    // have been unsuccessful.
    // Besides, iterators are a lot of boilerplate.
    fn foreach_border_value<F>(&mut self, mut f: F)
    where
        F: FnMut(&T),
    {
        for col in 0..self.width {
            for row in [0, self.height - 1] {
                f(&self.data[col + row * self.width]);
            }
        }
        for row in 0..self.height {
            for col in [0, self.width - 1] {
                f(&self.data[col + row * self.width]);
            }
        }
    }
}

impl Grid<(usize, Option<u8>)> {
    fn _print(&self, coordinates: &[(usize, usize)]) {
        let coordinates: HashSet<_> = coordinates.iter().collect();
        let mut line = String::with_capacity(self.width);
        for (i, (_, label)) in self.data.iter().enumerate() {
            let x = self.left + i % self.width;
            let y = self.top + i / self.width;
            if x == self.left {
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
    coordinates: Vec<(usize, usize)>,
}

impl Input {
    fn try_part_one(&self) -> anyhow::Result<usize> {
        let mut grid = Grid::from_coordinates(&self.coordinates, (usize::MAX, None));
        grid.apply(|x, y, old| {
            for (new_label, c) in self.coordinates.iter().enumerate() {
                let new_distance = x.abs_diff(c.0) + y.abs_diff(c.1);
                match new_distance.cmp(&old.0) {
                    Ordering::Less => {
                        *old = (new_distance, Some(new_label as u8));
                    }
                    Ordering::Equal => {
                        *old = (new_distance, None);
                    }
                    Ordering::Greater => {}
                };
            }
        });

        // An iterator may be nicer but is annoying to implement.
        // Returning a list with border values feels wasteful.
        let mut infinites: HashSet<u8> = HashSet::new();
        grid.foreach_border_value(|(_, l)| {
            if let Some(l) = l {
                infinites.insert(*l);
            }
        });

        let mut finite_areas = grid.data.iter().flat_map(|(_, l)| l).counts();
        finite_areas.retain(|k, _| !infinites.contains(k));

        finite_areas
            .into_values()
            .max()
            .ok_or_else(|| anyhow!("All points are equidistant"))
    }

    fn part_two_x(&self, threshold: usize) -> usize {
        let mut grid = Grid::from_coordinates(&self.coordinates, 0);
        grid.apply(|x, y, old| {
            for c in self.coordinates.iter() {
                let marginal_distance = x.abs_diff(c.0) + y.abs_diff(c.1);
                *old += marginal_distance;
            }
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
