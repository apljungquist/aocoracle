use std::fs;

fn _read_input(name: &str) -> String {
    fs::read_to_string(format!("day/2/{}", name)).unwrap()
}

enum Direction {
    FORWARD,
    DOWN,
    UP,
}

struct Command {
    direction: Direction,
    magnitude: u32,
}

fn _commands(text: &str) -> Vec<Command> {
    let mut result = Vec::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap();

        result.push(Command {
            direction: match direction {
                "forward" => Some(Direction::FORWARD),
                "down" => Some(Direction::DOWN),
                "up" => Some(Direction::UP),
                _ => None,
            }
            .unwrap(),
            magnitude: distance.parse::<u32>().unwrap(),
        });
    }
    result
}

fn part_1(filename: &str) -> u32 {
    let mut horizontal = 0;
    let mut vertital = 0;
    for command in _commands(&_read_input(filename)) {
        match command.direction {
            Direction::FORWARD => horizontal += command.magnitude,
            Direction::DOWN => vertital += command.magnitude,
            Direction::UP => vertital -= command.magnitude,
        }
    }
    horizontal * vertital
}

fn part_2(filename: &str) -> u32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertital = 0;
    for command in _commands(&_read_input(filename)) {
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
