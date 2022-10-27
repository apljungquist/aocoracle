use itertools::Itertools;
use std::collections::HashMap;

use std::hash::Hash;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    magnitude: u32,
}

impl Command {
    fn parse(line: &str) -> Option<Command> {
        let mut parts = line.split_whitespace();
        let direction = parts.next()?;
        let distance = parts.next()?;

        Some(Command {
            direction: match direction {
                "forward" => Some(Direction::Forward),
                "down" => Some(Direction::Down),
                "up" => Some(Direction::Up),
                _ => None,
            }?,
            magnitude: distance.parse::<u32>().unwrap(),
        })
    }
}

fn _map_reduce<T, U: Eq + Hash, V, W, F1, F2, F3>(
    values: impl Iterator<Item = T>,
    key_func: F1,
    value_func: F2,
    reduce_func: F3,
) -> HashMap<U, W>
where
    F1: Fn(&T) -> U,
    F2: Fn(&T) -> V,
    F3: Fn(Vec<V>) -> W,
{
    values
        .map(|v| (key_func(&v), value_func(&v)))
        .into_group_map()
        .drain()
        .map(|(k, vs)| (k, reduce_func(vs)))
        .collect()
}

pub fn part_1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let command = Command::parse(line).ok_or("Could not parse command")?;
        commands.push(command);
    }
    let mut counts: HashMap<Direction, u32> = _map_reduce(
        commands.iter(),
        |v| v.direction,
        |v| v.magnitude,
        |vs| vs.iter().sum(),
    );
    let result = counts.remove(&Direction::Forward).unwrap_or(0)
        * (counts.remove(&Direction::Down).unwrap_or(0)
            - counts.remove(&Direction::Up).unwrap_or(0));
    Ok(format!("{}", result))
}

pub fn part_2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for command in input.lines().map(Command::parse) {
        let command = command.ok_or("Could not parse command")?;
        match command.direction {
            Direction::Forward => {
                horizontal += command.magnitude;
                vertical += aim * command.magnitude
            }
            Direction::Down => aim += command.magnitude,
            Direction::Up => aim -= command.magnitude,
        }
    }
    Ok(format!("{}", horizontal * vertical))
}

#[cfg(test)]
mod tests {
    use crate::testing::{actual_answer, expected_answer};

    use super::*;

    fn assert_correct_answer(part: u8, stem: &str) {
        assert_eq!(
            actual_answer(
                file!(),
                match part {
                    1 => part_1,
                    2 => part_2,
                    _ => panic!(),
                },
                stem
            ),
            expected_answer(file!(), part, stem).unwrap(),
        )
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer(1, "example");
    }
    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer(1, "6bb0c0bd67");
    }
    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer(2, "example");
    }
    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer(2, "6bb0c0bd67");
    }
}
