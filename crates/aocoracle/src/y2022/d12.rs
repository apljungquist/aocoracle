use anyhow::{anyhow, bail};
use hashbrown::HashMap;
use pathfinding::prelude::bfs;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn successors(&self, heights: &HashMap<Point, u8>, reverse: bool) -> Vec<Point> {
        let &Point { x, y } = self;
        let prev = heights[self];
        let mut result = vec![
            Point { x, y: y - 1 },
            Point { x, y: y + 1 },
            Point { x: x - 1, y },
            Point { x: x + 1, y },
        ];
        result.retain(|p| {
            if let Some(next) = heights.get(p) {
                match reverse {
                    false => *next <= prev + 1,
                    true => *next + 1 >= prev,
                }
            } else {
                false
            }
        });
        result
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

fn heightmap(s: &str) -> anyhow::Result<(HashMap<Point, u8>, Point, Point)> {
    let mut heights = HashMap::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in s.lines().enumerate() {
        let y = y as i32;
        for (x, b) in line.bytes().enumerate() {
            let x = x as i32;
            let h = match b {
                b'S' => {
                    start = Some(Point { x, y });
                    0
                }
                b'E' => {
                    end = Some(Point { x, y });
                    b'z' - b'a'
                }
                b if b >= b'a' => b - b'a',
                _ => {
                    bail!("Expected height to be in [SEa-z] but got {}", b as char);
                }
            };
            heights.insert(Point { x, y }, h);
        }
    }
    Ok((
        heights,
        start.ok_or_else(|| anyhow!("Expected input to contain starting point"))?,
        end.ok_or_else(|| anyhow!("Expected input to contain starting point"))?,
    ))
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let (heights, end, start) = heightmap(input)?;
    bfs(&start, |p| p.successors(&heights, true), |p| *p == end)
        .map(|path| path.len() - 1)
        .ok_or_else(|| anyhow!("Could not find any path)"))
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let (heights, _, end) = heightmap(input)?;
    bfs(&end, |p| p.successors(&heights, true), |p| heights[p] == 0)
        .map(|path| path.len() - 1)
        .ok_or_else(|| anyhow!("Could not find any path)"))
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

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
