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

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> Result<T, Box<dyn std::error::Error>>,
{
    func(&fs::read_to_string(format!("inputs/02/{}.txt", stem)).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), "150");
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), "2187380");
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), "900");
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), "2086357770");
    }
}
