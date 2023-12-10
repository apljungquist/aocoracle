use std::collections::HashSet;

use anyhow::{anyhow, bail};
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Tile {
    fn from_char(s: &char) -> anyhow::Result<Option<Self>> {
        match s {
            '|' => Ok(Some(Self::NS)),
            '-' => Ok(Some(Self::EW)),
            'L' => Ok(Some(Self::NE)),
            'J' => Ok(Some(Self::NW)),
            '7' => Ok(Some(Self::SW)),
            'F' => Ok(Some(Self::SE)),
            '.' => Ok(Some(Self::Ground)),
            'S' => Ok(None),
            _ => Err(anyhow!("Expected pip but got {s}")),
        }
    }
}

#[derive(Debug)]
struct Input {
    start: (usize, usize),
    tiles: Vec<Vec<Option<Tile>>>,
}

type Pose = (usize, usize, Direction);

impl Input {
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut start = None;
        let mut rows = Vec::new();
        for (row_num, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (col_num, c) in line.chars().enumerate() {
                let tile = Tile::from_char(&c)?;
                if tile.is_none() {
                    start = Some((row_num, col_num))
                }
                row.push(tile);
            }
            rows.push(row)
        }
        Ok(Self {
            tiles: rows,
            start: start.ok_or_else(|| anyhow!("Found no starting position"))?,
        })
    }

    fn traverse_tile(&self, row: usize, col: usize, facing: Direction) -> Option<Pose> {
        use Direction::*;
        use Tile::*;
        let tile = self.tiles[row][col]?;
        match (tile, facing) {
            (NS, N) => Some((row - 1, col, N)),
            (NS, S) => Some((row + 1, col, S)),
            (NS, _) => None,
            (EW, E) => Some((row, col + 1, E)),
            (EW, W) => Some((row, col - 1, W)),
            (EW, _) => None,
            (NE, S) => Some((row, col + 1, E)),
            (NE, W) => Some((row - 1, col, N)),
            (NE, _) => None,
            (NW, S) => Some((row, col - 1, W)),
            (NW, E) => Some((row - 1, col, N)),
            (NW, _) => None,
            (SW, N) => Some((row, col - 1, W)),
            (SW, E) => Some((row + 1, col, S)),
            (SW, _) => None,
            (SE, N) => Some((row, col + 1, E)),
            (SE, W) => Some((row + 1, col, S)),
            (SE, _) => None,
            (Ground, _) => None,
        }
    }

    fn follow(&self, mut pose: (usize, usize, Direction)) -> Vec<(usize, usize, Direction)> {
        // println!("Follow path from {pose:?}");
        let mut path = vec![pose];
        loop {
            let Some(p) = self.traverse_tile(pose.0, pose.1, pose.2) else {
                // println!("Cannot continue path from {:?}", pose);
                return path;
            };
            path.push(p);
            pose = p;
        }
    }

    fn rhs(&self, pose: Pose) -> Option<Vec<(usize, usize)>> {
        use Direction::*;
        use Tile::*;
        let (r, c, d) = pose;
        let tile = self.tiles[r][c]?;
        match (tile, d) {
            (NS, N) => Some(vec![(r, c + 1)]),
            (NS, S) => Some(vec![(r, c.checked_sub(1)?)]),
            (NS, _) => panic!("Not on a loop"),
            (EW, E) => Some(vec![(r + 1, c)]),
            (EW, W) => Some(vec![(r.checked_sub(1)?, c)]),
            (EW, _) => panic!("Not on a loop"),
            (NE, S) => Some(vec![
                (r, c.checked_sub(1)?),
                (r + 1, c.checked_sub(1)?),
                (r + 1, c),
            ]),
            (NE, W) => Some(vec![(r.checked_sub(1)?, c + 1)]),
            (NE, _) => panic!("Not on a loop"),
            (NW, S) => Some(vec![(r.checked_sub(1)?, c.checked_sub(1)?)]),
            (NW, E) => Some(vec![(r, c + 1), (r + 1, c + 1), (r + 1, c)]),
            (NW, _) => panic!("Not on a loop"),
            (SW, N) => Some(vec![
                (r.checked_sub(1)?, c),
                (r.checked_sub(1)?, c + 1),
                (r, c + 1),
            ]),
            (SW, E) => Some(vec![(r + 1, c.checked_sub(1)?)]),
            (SW, _) => panic!("Not on a loop"),
            (SE, N) => Some(vec![(r + 1, c + 1)]),
            (SE, W) => Some(vec![
                (r.checked_sub(1)?, c),
                (r.checked_sub(1)?, c.checked_sub(1)?),
                (r, c.checked_sub(1)?),
            ]),
            (SE, _) => panic!("Not on a loop"),
            (Ground, _) => panic!("Ground is never on a loop"),
        }
    }
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    use Direction::*;
    let input = Input::from_str(input)?;
    for (dr, dc, d) in [(1, 0, S), (0, -1, W), (-1, 0, N), (0, 1, E)] {
        let Some(r) = input.start.0.checked_add_signed(dr) else {
            continue;
        };
        let Some(c) = input.start.1.checked_add_signed(dc) else {
            continue;
        };
        let path = input.follow((r, c, d));
        if path
            .last()
            .map(|p| p.0 == input.start.0 && p.1 == input.start.1)
            .unwrap_or(false)
        {
            return Ok(path.len() / 2);
        }
    }
    bail!("No solution")
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    use Direction::*;
    let input = Input::from_str(input)?;
    'outer: for (dr, dc, d) in [(1, 0, S), (0, -1, W), (-1, 0, N), (0, 1, E)] {
        let Some(r) = input.start.0.checked_add_signed(dr) else {
            continue;
        };
        let Some(c) = input.start.1.checked_add_signed(dc) else {
            continue;
        };
        let path = input.follow((r, c, d));
        if path
            .last()
            .map(|p| p.0 == input.start.0 && p.1 == input.start.1)
            .unwrap_or(false)
        {
            let rmax = input.tiles.len();
            let cmax = input.tiles[0].len();
            let boundary: HashSet<_> = path.iter().map(|(r, c, _)| (*r, *c)).collect();
            let mut seen: HashSet<(usize, usize)> = HashSet::new();
            let mut to_see = path
                .into_iter()
                .flat_map(|p| input.rhs(p))
                .flatten()
                .filter(|p| !boundary.contains(p))
                .collect_vec();
            while let Some(p) = to_see.pop() {
                if seen.contains(&p) {
                    continue;
                }
                seen.insert(p);
                for dr in [-1, 0, 1] {
                    for dc in [-1, 0, 1] {
                        let Some(r) = p.0.checked_add_signed(dr) else {
                            continue 'outer;
                        };
                        let Some(c) = p.1.checked_add_signed(dc) else {
                            continue 'outer;
                        };
                        if cmax <= c || rmax <= r {
                            continue 'outer;
                        }
                        let q = (r, c);
                        if boundary.contains(&q) {
                            continue;
                        }
                        if seen.contains(&q) {
                            continue;
                        }
                        to_see.push(q);
                    }
                }
            }
            return Ok(seen.len());
        }
    }
    bail!("No solution")
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example_a() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE_1a", Part::One);
    }

    #[test]
    fn part_1_works_on_example_b() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE_1b", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "a4caf3ba644984d9", Part::One);
        // > 253652923
    }

    #[test]
    fn part_2_works_on_example_a() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE_2a", Part::Two);
    }

    #[test]
    fn part_2_works_on_example_b() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE_2b", Part::Two);
    }

    #[test]
    fn part_2_works_on_example_c() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE_2c", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "a4caf3ba644984d9", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
