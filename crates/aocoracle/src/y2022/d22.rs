use anyhow::bail;
use hashbrown::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    row: i32,
    col: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point(row: {}, col: {})", self.row, self.col)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Open,
    Blocked,
}

fn map(s: &str) -> anyhow::Result<HashMap<Point, Tile>> {
    let (s, _) = s.split_once("\n\n").unwrap();
    let mut result = HashMap::new();
    for (row, line) in s.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    result.insert(
                        Point {
                            row: row as i32,
                            col: col as i32,
                        },
                        Tile::Open,
                    );
                }
                '#' => {
                    result.insert(
                        Point {
                            row: row as i32,
                            col: col as i32,
                        },
                        Tile::Blocked,
                    );
                }
                ' ' => {}
                _ => {
                    bail!("Expected '.', '#', or  ' ' but got {char}");
                }
            }
        }
    }
    Ok(result)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Step {
    Move(i32),
    Rotate(Direction),
}

fn steps(s: &str) -> anyhow::Result<Vec<Step>> {
    let (_, s) = s.split_once("\n\n").unwrap();
    let s = s.replace('R', " R ");
    let s = s.replace('L', " L ");
    let mut result = Vec::new();
    for token in s.split_whitespace() {
        result.push(match token.parse::<i32>() {
            Ok(distance) => Step::Move(distance),
            Err(_) => match token {
                "L" => Step::Rotate(Direction::Left),
                "R" => Step::Rotate(Direction::Right),
                _ => {
                    bail!("Expected number, 'L', or 'R' but got {token}");
                }
            },
        })
    }
    Ok(result)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Heading {
    N,
    E,
    S,
    W,
}

use Heading::*;

impl Heading {
    fn score(&self) -> i32 {
        match self {
            Self::E => 0,
            Self::S => 1,
            Self::W => 2,
            Self::N => 3,
        }
    }
    fn from_score(score: i32) -> Self {
        match score {
            0 => Self::E,
            1 => Self::S,
            2 => Self::W,
            3 => Self::N,
            _ => {
                panic!("Oops");
            }
        }
    }

    fn rotated(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Left => Self::from_score((self.score() + 3) % 4),
            Direction::Right => Self::from_score((self.score() + 1) % 4),
        }
    }

    fn point(&self) -> Point {
        match self {
            Self::N => Point { row: -1, col: 0 },
            Self::E => Point { row: 0, col: 1 },
            Self::S => Point { row: 1, col: 0 },
            Self::W => Point { row: 0, col: -1 },
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pose {
    position: Point,
    orientation: Heading,
}

impl Pose {
    fn updated<F>(&self, step: &Step, map: &HashMap<Point, Tile>, wrap: &F) -> Self
    where
        F: Fn(&Self) -> Self,
    {
        match step {
            Step::Move(d) => {
                let mut confirmed = self.clone();
                for _ in 0..*d {
                    if let Some(candidate) = confirmed.moved(map, wrap) {
                        confirmed = candidate;
                    } else {
                        return confirmed;
                    }
                }
                confirmed
            }
            Step::Rotate(d) => self.rotated(d),
        }
    }

    fn moved<F>(&self, map: &HashMap<Point, Tile>, wrap: F) -> Option<Self>
    where
        F: Fn(&Self) -> Self,
    {
        let mut result = self.clone();
        result.position = &result.position + &result.orientation.point();
        if !map.contains_key(&result.position) {
            println!("Leaving {self:?}");
            result = wrap(self);
            println!("Entering {result:?}");
        }
        match map.get(&result.position).unwrap() {
            Tile::Open => {
                // println!("{:?} is open, updating position", &result.position);
                Some(result)
            }
            Tile::Blocked => {
                // println!(
                //     "{:?} is blocked, returning {:?}",
                //     &result.position, &self.position
                // );
                None
            }
        }
    }

    fn rotated(&self, direction: &Direction) -> Self {
        Self {
            position: self.position.clone(),
            orientation: self.orientation.rotated(direction),
        }
    }

    fn wrapped_1_example(&self, side: i32) -> Self {
        let leaving = (
            self.position.row / side,
            self.position.col / side,
            self.orientation.clone(),
        );
        let entering = match leaving {
            // 1
            (0, 2, N) => (2, 2, N),
            (0, 2, E) => (0, 2, E),
            (0, 2, W) => (0, 2, W),
            // 2
            (1, 0, N) => (1, 0, N),
            (1, 0, S) => (1, 0, S),
            (1, 0, E) => (1, 2, E),
            // 3
            (1, 1, N) => (1, 1, N),
            (1, 1, S) => (1, 1, S),
            // 4
            (1, 2, E) => (1, 0, E),
            // 5
            (2, 2, S) => (0, 2, S),
            (2, 2, W) => (2, 3, W),
            //
            (2, 3, N) => (2, 3, N),
            (2, 3, E) => (2, 2, E),
            (2, 3, S) => (2, 3, S),
            _ => {
                panic!("Oops {leaving:?} {self:?}");
            }
        };
        let (rel_row, rel_col) = match self.orientation {
            N => (side, self.position.col % side),
            S => (0, self.position.col % side),
            E => (self.position.row % side, 0),
            W => (self.position.row % side, side),
        };
        Self {
            position: Point {
                row: entering.0 * side + rel_row,
                col: entering.1 * side + rel_col,
            },
            orientation: entering.2,
        }
    }

    fn wrapped_2_example(&self, side: i32) -> Self {
        let leaving = (
            self.position.row / side,
            self.position.col / side,
            self.orientation.clone(),
        );
        let rel_row = self.position.row % side;
        let rel_col = self.position.col % side;
        let rel_min = 0;
        let rel_max = side - 1;
        let entering = match leaving {
            // 1
            // 2
            // 3
            (1, 1, N) => (0, 2, E, rel_col, rel_min),
            // 4
            (1, 2, E) => (2, 3, S, rel_min, rel_max - rel_row),
            // 5
            (2, 2, S) => (1, 0, N, rel_max, rel_max - rel_col),
            // 6
            _ => {
                panic!("Oops {leaving:?} {self:?}");
            }
        };
        Self {
            position: Point {
                row: entering.0 * side + entering.3,
                col: entering.1 * side + entering.4,
            },
            orientation: entering.2,
        }
    }
}

fn score(row: i32, col: i32, heading: Heading) -> i32 {
    1000 * (row + 1) + 4 * (col + 1) + heading.score()
}

fn initial_pose(map: &HashMap<Point, Tile>) -> Pose {
    for col in 0.. {
        let p = Point { row: 0, col: col };
        if let Some(tile) = map.get(&p) {
            match tile {
                Tile::Open => {
                    return Pose {
                        position: p,
                        orientation: Heading::E,
                    };
                }
                Tile::Blocked => {}
            }
        }
    }
    unreachable!()
}

//              111111
//    0123456789012345
// 00         >>v#
// 01         .#v.
// 02         #.v.
// 03         ..v.
// 04 ...#...v..v#
// 05 >>>v...>#.>>
// 06 ..#v...#....
// 07 ...>>>>v..#.
// 08         ...#....
// 19         .....#..
// 10         .#......
// 11         ......#.
//  0,  8
//  0, 10
//  5, 10
//  5,  3
//  7,  3
//  7,  7
//  5,  7

fn part_x(s: &str, is_part_2: bool) -> anyhow::Result<i32> {
    let map = map(s)?;
    let side = match map.len() {
        96 => 4,
        15000 => 50,
        _ => {
            bail!("Expected faces with side 4 or 50");
        }
    };
    let steps = steps(s)?;
    let mut pose = initial_pose(&map);
    // dbg!(("initial", &position, &rotation));
    println!("initial {pose:?}");
    for (i, step) in steps.into_iter().enumerate() {
        pose = match (is_part_2, side) {
            (false, 4) => pose.updated(&step, &map, &|p| p.wrapped_1_example(side)),
            (true, 4) => pose.updated(&step, &map, &|p| p.wrapped_2_example(side)),
            _ => {
                bail!("Not implemented")
            }
        };
        println!("{i} {step:?} {pose:?}");
    }

    Ok(score(
        pose.position.row,
        pose.position.col,
        pose.orientation,
    ))
}

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    part_x(input, false)
}

pub fn part_2(input: &str) -> anyhow::Result<i32> {
    part_x(input, true)
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
