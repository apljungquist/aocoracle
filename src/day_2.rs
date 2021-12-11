use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
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
    fn parse(line: &str) -> Command {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap();

        Command {
            direction: match direction {
                "forward" => Some(Direction::Forward),
                "down" => Some(Direction::Down),
                "up" => Some(Direction::Up),
                _ => None,
            }
            .unwrap(),
            magnitude: distance.parse::<u32>().unwrap(),
        }
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

pub fn part_1(input: &str) -> u32 {
    let mut counts: HashMap<Direction, u32> = _map_reduce(
        input.lines().map(Command::parse),
        |v| v.direction,
        |v| v.magnitude,
        |vs| vs.iter().sum(),
    );
    counts.remove(&Direction::Forward).unwrap_or(0)
        * (counts.remove(&Direction::Down).unwrap_or(0)
            - counts.remove(&Direction::Up).unwrap_or(0))
}

pub fn part_2(input: &str) -> u32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for command in input.lines().map(Command::parse) {
        match command.direction {
            Direction::Forward => {
                horizontal += command.magnitude;
                vertical += aim * command.magnitude
            }
            Direction::Down => aim += command.magnitude,
            Direction::Up => aim -= command.magnitude,
        }
    }
    horizontal * vertical
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/2/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 150);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 2187380);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 900);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 2086357770);
    }
}
