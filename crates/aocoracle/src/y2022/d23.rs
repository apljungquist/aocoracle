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
    fn north_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn north_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn south_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn south_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
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

fn updated_elf(map: &HashSet<Point>, directions: &VecDeque<Direction>, elf: &Point) -> Point {
    let nw = !map.contains(&elf.north_west());
    let nn = !map.contains(&elf.north());
    let ne = !map.contains(&elf.north_east());

    let ww = !map.contains(&elf.west());
    let ee = !map.contains(&elf.east());

    let sw = !map.contains(&elf.south_west());
    let ss = !map.contains(&elf.south());
    let se = !map.contains(&elf.south_east());
    if nw && nn && ne && ww && ee && sw && ss && se {
        elf.clone()
    } else {
        for direction in directions {
            match direction {
                Direction::N => {
                    if nn && ne && nw {
                        return elf.north();
                    }
                }
                Direction::S => {
                    if ss && se && sw {
                        return elf.south();
                    }
                }
                Direction::W => {
                    if ww && nw && sw {
                        return elf.west();
                    }
                }
                Direction::E => {
                    if ee && ne && se {
                        return elf.east();
                    }
                }
            }
        }
        elf.clone()
    }
}

fn updated_map(before: &HashSet<Point>, directions: &VecDeque<Direction>) -> HashSet<Point> {
    let mut during: HashMap<Point, Vec<Point>> = HashMap::with_capacity(before.len());
    for elf in before {
        during
            .entry(updated_elf(before, directions, elf))
            .or_default()
            .push(elf.clone());
    }
    let mut after = HashSet::with_capacity(before.len());
    for (proposed, elfs) in during.drain() {
        if elfs.len() > 1 {
            for elf in elfs {
                after.insert(elf);
            }
        } else {
            after.insert(proposed);
        }
    }
    after
}

fn simulate(map: HashSet<Point>, num_step: Option<usize>) -> (usize, HashSet<Point>) {
    let mut directions: VecDeque<_> = [Direction::N, Direction::S, Direction::W, Direction::E]
        .into_iter()
        .collect();

    let mut old = map;
    let mut new = updated_map(&old, &directions);
    directions.rotate_left(1);

    let mut i = 1;
    while new != old && Some(i) != num_step {
        i += 1;
        old = new;
        new = updated_map(&old, &directions);
        directions.rotate_left(1);
    }
    (i, new)
}

fn num_empty_tile(map: &HashSet<Point>) -> Option<usize> {
    let min_x = map.iter().map(|p| p.x).min()?;
    let max_x = map.iter().map(|p| p.x).max()?;
    let min_y = map.iter().map(|p| p.y).min()?;
    let max_y = map.iter().map(|p| p.y).max()?;
    let w = (max_x - min_x + 1) as usize;
    let h = (max_y - min_y + 1) as usize;
    let a = w * h;
    Some(a - map.len())
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let before = map(input)?;
    let (_, after) = simulate(before, Some(10));
    Ok(num_empty_tile(&after).expect("Validation ensures there is at least one elf on the map"))
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let before = map(input)?;
    let (num_step, _) = simulate(before, None);
    Ok(num_step)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "e6adf19648853d59", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "e6adf19648853d59", Part::Two);
    }

    // Fails on 2015/18/3ba7923eae and possibly others
    // That is rectangular grid of '#' and '.', just as the input to this problem.
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
