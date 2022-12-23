use anyhow::bail;
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn map(s: &str) -> anyhow::Result<HashSet<Point>> {
    let mut result = HashSet::new();
    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    result.insert(Point::new(x as i32, y as i32));
                }
                '.' => {}
                _ => bail!("Expected #."),
            }
        }
    }
    Ok(result)
}

enum Direction {
    N,
    S,
    W,
    E,
}

fn updated_map(old: &HashSet<Point>, directions: &VecDeque<Direction>) -> HashSet<Point> {
    let mut intermediate: HashMap<Point, Vec<Point>> = HashMap::with_capacity(old.len());
    'outer: for elf in old {
        // let mut headings = Vec::with_capacity(4);
        let nw = !old.contains(&Point::new(elf.x - 1, elf.y - 1));
        let nn = !old.contains(&Point::new(elf.x, elf.y - 1));
        let ne = !old.contains(&Point::new(elf.x + 1, elf.y - 1));

        let ww = !old.contains(&Point::new(elf.x - 1, elf.y));
        let ee = !old.contains(&Point::new(elf.x + 1, elf.y));

        let sw = !old.contains(&Point::new(elf.x - 1, elf.y + 1));
        let ss = !old.contains(&Point::new(elf.x, elf.y + 1));
        let se = !old.contains(&Point::new(elf.x + 1, elf.y + 1));
        if nw && nn && ne && ww && ee && sw && ss && se {
            intermediate
                .entry(elf.clone())
                .or_default()
                .push(elf.clone());
        } else {
            for d in directions {
                match d {
                    Direction::N => {
                        if nn && ne && nw {
                            intermediate
                                .entry(Point::new(elf.x, elf.y - 1))
                                .or_default()
                                .push(elf.clone());
                            continue 'outer;
                        }
                    }
                    Direction::S => {
                        if ss && se && sw {
                            intermediate
                                .entry(Point::new(elf.x, elf.y + 1))
                                .or_default()
                                .push(elf.clone());
                            continue 'outer;
                        }
                    }
                    Direction::W => {
                        if ww && nw && sw {
                            intermediate
                                .entry(Point::new(elf.x - 1, elf.y))
                                .or_default()
                                .push(elf.clone());
                            continue 'outer;
                        }
                    }
                    Direction::E => {
                        if ee && ne && se {
                            intermediate
                                .entry(Point::new(elf.x + 1, elf.y))
                                .or_default()
                                .push(elf.clone());
                            continue 'outer;
                        }
                    }
                }
            }
            intermediate
                .entry(elf.clone())
                .or_default()
                .push(elf.clone());
        }
    }
    let mut new = HashSet::with_capacity(old.len());
    for (proposed, elfs) in intermediate.drain() {
        if elfs.len() > 1 {
            for elf in elfs {
                new.insert(elf);
            }
        } else {
            assert_eq!(elfs.len(), 1);
            new.insert(proposed);
        }
    }
    new
}

fn print_map(map: &HashSet<Point>) {
    let min_x = map.iter().map(|p| p.x).min().unwrap();
    let max_x = map.iter().map(|p| p.x).max().unwrap();
    let min_y = map.iter().map(|p| p.y).min().unwrap();
    let max_y = map.iter().map(|p| p.y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    let mut old = map(input)?;
    let mut directions = VecDeque::with_capacity(4);
    directions.push_back(Direction::N);
    directions.push_back(Direction::S);
    directions.push_back(Direction::W);
    directions.push_back(Direction::E);
    let mut new = updated_map(&old, &directions);
    directions.rotate_left(1);
    println!("== Initial State ==");
    print_map(&old);
    println!("== End of Round 1 ==");
    print_map(&new);
    for i in 2..=10 {
        old = new;
        new = updated_map(&old, &directions);
        directions.rotate_left(1);
        assert_eq!(old.len(), new.len());
        println!("== End of Round {i} ==");
        print_map(&new);
        if old == new {
            break;
        }
    }
    let min_x = new.iter().map(|p| p.x).min().unwrap();
    let max_x = new.iter().map(|p| p.x).max().unwrap();
    let min_y = new.iter().map(|p| p.y).min().unwrap();
    let max_y = new.iter().map(|p| p.y).max().unwrap();
    let w = max_x - min_x + 1;
    let h = max_y - min_y + 1;
    let a = w * h;
    Ok(a - new.len() as i32)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
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
