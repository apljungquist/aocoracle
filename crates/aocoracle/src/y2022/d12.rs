use hashbrown::HashMap;
use pathfinding::prelude::bfs;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn successors(&self, heights: &HashMap<Point, u8>) -> Vec<Point> {
        let &Point { x, y } = self;
        let height = heights[&self];
        let mut result = vec![
            Point { x: x, y: y - 1 },
            Point { x: x, y: y + 1 },
            Point { x: x - 1, y: y },
            Point { x: x + 1, y: y },
        ];
        result.retain(|p| {
            if let Some(h) = heights.get(p) {
                *h < height + 2
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

fn height_map(s: &str) -> (HashMap<(Point), u8>, Point, Point) {
    let mut result = HashMap::new();
    let mut start = Point::default();
    let mut end = Point::default();
    for (y, line) in s.lines().enumerate() {
        for (x, height) in line.bytes().enumerate() {
            match height {
                b'S' => {
                    let x = x as i32;
                    let y = y as i32;
                    let height = 0;
                    start = Point { x, y };
                    result.insert(Point { x, y }, height);
                }
                b'E' => {
                    let x = x as i32;
                    let y = y as i32;
                    let height = b'z' - b'a';
                    end = Point { x, y };
                    result.insert(Point { x, y }, height);
                }
                _ => {
                    let x = x as i32;
                    let y = y as i32;
                    let height = height - b'a';
                    result.insert(Point { x, y }, height);
                }
            }
        }
    }
    (result, start, end)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let (heights, start, end) = height_map(input);
    let result = bfs(&start, |p| p.successors(&heights), |p| *p == end).unwrap();
    // let mut distances = HashMap::with_capacity(heights.len());
    // let mut curr_h = heights[&start];
    // let mut curr_d = 0;
    // distances.insert(curr_p, curr_d);

    // while start != end {
    //     for delta in deltas {
    //         let next_p = curr_p + delta;
    //         if let Some(next_h) = heights.get(&next_p){
    //             if next_h == curr_h+1{
    //                 di
    //             }
    //         }
    //     }
    // }
    Ok(result.len() - 1)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let (heights, _, end) = height_map(input);
    let result = heights
        .iter()
        .filter(|(p, h)| **h == 0)
        .filter_map(|(p, _)| {
            bfs(p, |p| p.successors(&heights), |p| *p == end).and_then(|r| Some(r.len()))
        })
        .min()
        .unwrap();

    Ok(result - 1)
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
