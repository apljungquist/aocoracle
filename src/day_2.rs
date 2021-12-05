use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn _read_input(name: &str) -> String {
    fs::read_to_string(format!("day/2/{}", name)).unwrap()
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    FORWARD,
    DOWN,
    UP,
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
                "forward" => Some(Direction::FORWARD),
                "down" => Some(Direction::DOWN),
                "up" => Some(Direction::UP),
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

fn part_1(filename: &str) -> u32 {
    let mut counts: HashMap<Direction, u32> = _map_reduce(
        _read_input(filename).lines().map(Command::parse),
        |v| v.direction,
        |v| v.magnitude,
        |vs| vs.iter().sum(),
    );
    counts.remove(&Direction::FORWARD).unwrap_or(0)
        * (counts.remove(&Direction::DOWN).unwrap_or(0)
            - counts.remove(&Direction::UP).unwrap_or(0))
}

fn part_2(filename: &str) -> u32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertital = 0;
    for command in _read_input(filename).lines().map(Command::parse) {
        match command.direction {
            Direction::FORWARD => {
                horizontal += command.magnitude;
                vertital += aim * command.magnitude
            }
            Direction::DOWN => aim += command.magnitude,
            Direction::UP => aim -= command.magnitude,
        }
    }
    horizontal * vertital
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1("example.txt"), 150);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(part_1("input.txt"), 2187380);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2("example.txt"), 900);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(part_2("input.txt"), 2086357770);
    }
}
