use anyhow::{anyhow, bail};
use hashbrown::HashSet;
use std::iter;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Default, Eq, Hash, PartialEq)]
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn motions(s: &str) -> anyhow::Result<Vec<(Point, i32)>> {
    let mut result = Vec::new();
    for line in s.lines() {
        let (direction, distance) = line.split_once(' ').ok_or_else(|| {
            anyhow!("Expected direction and distance separated by space but got {line}")
        })?;
        let direction = match direction {
            "U" => Point { x: 0, y: 1 },
            "D" => Point { x: 0, y: -1 },
            "L" => Point { x: -1, y: 0 },
            "R" => Point { x: 1, y: 0 },
            _ => bail!("Expected direction to be one of U, D, L, R but got {direction}"),
        };
        let distance = distance.parse::<i32>()?;
        result.push((direction, distance));
    }
    Ok(result)
}

fn tail_motion(head: &Point, tail: &Point) -> Point {
    let displacement = head - tail;
    assert!((-2..=2).contains(&displacement.x));
    assert!((-2..=2).contains(&displacement.y));
    if displacement.x.abs() == 2 || displacement.y.abs() == 2 {
        Point {
            x: displacement.x.signum(),
            y: displacement.y.signum(),
        }
    } else {
        Point::default()
    }
}

fn part_x(input: &str, num_knot: usize) -> anyhow::Result<usize> {
    if num_knot < 2 {
        bail!("Expected at least 2 knots but got {num_knot}");
    }
    let motions = motions(input)?;
    let mut rope: Vec<_> = iter::repeat_with(Point::default).take(num_knot).collect();
    let mut visited: HashSet<_> = iter::once(Point::default()).collect();
    for (direction, distance) in motions {
        for _ in 0..distance {
            rope[0] += direction.clone();
            for i in 1..rope.len() {
                let tail_motion = tail_motion(&rope[i - 1], &rope[i]);
                rope[i] += tail_motion;
            }
            visited.insert(rope[rope.len() - 1].clone());
        }
    }
    Ok(visited.len())
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    part_x(input, 2)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    part_x(input, 10)
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
    fn part_2_works_on_example_l() {
        assert_correct_answer_on_correct_input!(part_2, "example_l", Part::Two);
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
