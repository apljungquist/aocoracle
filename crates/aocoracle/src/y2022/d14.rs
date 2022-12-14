use hashbrown::HashSet;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
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

fn rock_paths(s: &str) -> Vec<Vec<Point>> {
    let mut result = Vec::new();
    for line in s.lines() {
        result.push(
            line.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Point { x, y }
                })
                .collect(),
        );
    }
    result
}

fn rock_coordinates(paths: &Vec<Vec<Point>>) -> HashSet<Point> {
    let mut result = HashSet::new();
    for path in paths {
        for j in 1..path.len() {
            let i = j - 1;
            let start = path[i];
            let end = path[j];
            if start.x == end.x {
                let x = start.x;
                for y in start.y.min(end.y)..=start.y.max(end.y) {
                    result.insert(Point { x, y });
                }
            } else if start.y == end.y {
                let y = start.y;
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    result.insert(Point { x, y });
                }
            } else {
                panic!("Expected vertical or horizontal line")
            }
        }
    }
    result
}

fn moved_sand(blocked: &HashSet<Point>, curr: &Point) -> Option<Point> {
    let movements = [
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 1, y: 1 },
    ];
    for movement in movements {
        let next = *curr + movement;
        if !blocked.contains(&next) {
            return Some(next);
        }
    }
    None
}

fn debug_print_coordinates(coordinates: &HashSet<Point>) {
    let x_min = coordinates.iter().map(|p| p.x).min().unwrap();
    let x_max = coordinates.iter().map(|p| p.x).max().unwrap();
    let y_min = coordinates.iter().map(|p| p.y).min().unwrap();
    let y_max = coordinates.iter().map(|p| p.y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if coordinates.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let paths = rock_paths(input);
    let mut coordinates = rock_coordinates(&paths);
    let y_max = coordinates.iter().map(|p| p.y).max().unwrap();
    debug_print_coordinates(&coordinates);
    for i in 1.. {
        let mut curr = Point { x: 500, y: 0 };
        while let Some(next) = moved_sand(&coordinates, &curr) {
            if next.y == y_max {
                return Ok(i - 1);
            }
            curr = next;
        }
        coordinates.insert(curr);
    }
    unreachable!()
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
