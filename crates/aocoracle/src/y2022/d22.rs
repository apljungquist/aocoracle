use crate::y2022::d22::Step::Move;
use anyhow::bail;
use hashbrown::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;

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
                            row: row as i32 + 1,
                            col: col as i32 + 1,
                        },
                        Tile::Open,
                    );
                }
                '#' => {
                    result.insert(
                        Point {
                            row: row as i32 + 1,
                            col: col as i32 + 1,
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
    Up,
    Right,
    Down,
    Left,
}

impl Heading {
    fn score(&self) -> i32 {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }
    fn from_score(score: i32) -> Self {
        match score {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => {
                panic!("Oops");
            }
        }
    }
    fn rotated_left(&self) -> Self {
        Self::from_score((self.score() + 3) % 4)
    }
    fn rotated_right(&self) -> Self {
        Self::from_score((self.score() + 1) % 4)
    }
    fn point(&self) -> Point {
        match self {
            Self::Up => Point { row: -1, col: 0 },
            Self::Right => Point { row: 0, col: 1 },
            Self::Down => Point { row: 1, col: 0 },
            Self::Left => Point { row: 0, col: -1 },
        }
    }
}

fn score(row: i32, col: i32, heading: Heading) -> i32 {
    1000 * row + 4 * col + heading.score()
}

fn initial_position(map: &HashMap<Point, Tile>) -> Point {
    for col in 1.. {
        let p = Point { row: 1, col: col };
        if let Some(tile) = map.get(&p) {
            match tile {
                Tile::Open => {
                    return p;
                }
                Tile::Blocked => {}
            }
        }
    }
    unreachable!()
}

fn updated_rotation(old: Heading, step: &Direction) -> Heading {
    match step {
        Direction::Left => old.rotated_left(),
        Direction::Right => old.rotated_right(),
    }
}

fn updated_position(
    map: &HashMap<Point, Tile>,
    mut old: Point,
    rotation: &Heading,
    distance: i32,
) -> Point {
    let mut new = old.clone();
    let delta = rotation.point();
    for _ in 0..distance {
        new = &old + &delta;
        if let Some(tile) = map.get(&new) {
            match tile {
                Tile::Open => {
                    // println!("{:?} is open, updating position", &new);
                    old = new;
                }
                Tile::Blocked => {
                    // println!("{:?} is blocked, returning {:?}", &new, &old);
                    return old;
                }
            }
        } else {
            // println!("{:?} is missing, rewinding", &new);
            new = old.clone();
            while let Some(_) = map.get(&new) {
                new = &new - &delta;
            }
            new = &new + &delta;
            if let Some(tile) = map.get(&new) {
                match tile {
                    Tile::Open => {
                        // println!("{:?} is open, updating position after rewind", &new);
                        old = new;
                    }
                    Tile::Blocked => {
                        // println!("{:?} is blocked, returning after rewind {:?}", &new, &old);
                        return old;
                    }
                }
            } else {
                panic!("Should not happen")
            }
        }
    }
    // println!("Came to a stop, returning {:?}", &old);
    old
}

//             111
//    123456789012
// 01         >>v#
// 02         .#v.
// 03         #.v.
// 04         ..v.
// 05 ...#...v..v#
// 06 >>>v...>#.>>
// 07 ..#v...#....
// 08 ...>>>>v..#.
// 09         ...#....
// 10         .....#..
// 11         .#......
// 12         ......#.

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    let map = map(input)?;
    let steps = steps(input)?;

    let mut position = initial_position(&map);
    let mut rotation = Heading::Right;

    dbg!(&position, &rotation);
    for (i, step) in steps.into_iter().enumerate() {
        match &step {
            Step::Move(distance) => {
                position = updated_position(&map, position, &rotation, *distance);
            }
            Step::Rotate(direction) => {
                rotation = updated_rotation(rotation, &direction);
            }
        }
        // dbg!(i, &step, &position, &rotation);
    }

    Ok(score(position.row, position.col, rotation))
}

pub fn part_2(input: &str) -> anyhow::Result<i128> {
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
