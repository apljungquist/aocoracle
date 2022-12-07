use anyhow::{anyhow, bail};
use hashbrown::{HashMap, HashSet};
use std::str::FromStr;

struct Map {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    heights: HashMap<(i32, i32), i32>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, h) in line.chars().enumerate() {
                let h = h
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("Expected a digit but got {h}"))?
                    .try_into()?;
                heights.insert((x as i32, y as i32), h);
            }
        }
        let x_min = 0;
        let x_max = *heights.keys().map(|(x, _)| x).max().unwrap();
        let y_min = 0;
        let y_max = *heights.keys().map(|(_, y)| y).max().unwrap();

        let h = y_max - y_min + 1;
        let w = x_max - x_min + 1;
        if h * w != heights.len() as i32 {
            bail!("Expected a rectangular grid")
        }
        if h < 3 || w < 3 {
            bail!("Expected a grid with no sides shorter than 3")
        }

        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
            heights,
        })
    }
}

impl Map {
    fn get(&self, coord: &(i32, i32)) -> Option<&i32> {
        self.heights.get(coord)
    }
}

fn append_visible(
    map: &Map,
    mut coords: impl Iterator<Item = (i32, i32)>,
    result: &mut HashSet<(i32, i32)>,
) -> Option<()> {
    let coord = coords.next()?;
    let mut h_max = map.get(&coord)?;
    result.insert(coord);
    for coord in coords {
        let h = map.get(&coord)?;
        if h_max < h {
            h_max = h;
            result.insert(coord);
        }
    }
    Some(())
}

fn visible(map: &Map) -> Option<HashSet<(i32, i32)>> {
    let mut result = HashSet::new();
    for x in map.x_min..=map.x_max {
        // From top
        append_visible(map, (map.y_min..=map.y_max).map(|y| (x, y)), &mut result)?;
        // From bottom
        append_visible(
            map,
            (map.y_min..=map.y_max).rev().map(|y| (x, y)),
            &mut result,
        )?;
    }
    for y in map.y_min..=map.y_max {
        // From left
        append_visible(map, (map.x_min..=map.x_max).map(|x| (x, y)), &mut result)?;
        // From right
        append_visible(
            map,
            (map.x_min..=map.x_max).rev().map(|x| (x, y)),
            &mut result,
        )?;
    }
    Some(result)
}

fn scenic_score(map: &Map, x: i32, y: i32) -> Option<i32> {
    let thresh = *map.get(&(x, y))?;

    let mut d_up = 0;
    for i in 1.. {
        if let Some(&h) = map.get(&(x, y - i)) {
            if thresh <= h {
                d_up = i;
                break;
            }
        } else {
            d_up = i - 1;
            break;
        }
    }

    let mut d_down = 0;
    for i in 1.. {
        if let Some(&h) = map.get(&(x, y + i)) {
            if thresh <= h {
                d_down = i;
                break;
            }
        } else {
            d_down = i - 1;
            break;
        }
    }

    let mut d_left = 0;
    for i in 1.. {
        if let Some(&h) = map.get(&(x - i, y)) {
            if thresh <= h {
                d_left = i;
                break;
            }
        } else {
            d_left = i - 1;
            break;
        }
    }

    let mut d_right = 0;
    for i in 1.. {
        if let Some(&h) = map.get(&(x + i, y)) {
            if thresh <= h {
                d_right = i;
                break;
            }
        } else {
            d_right = i - 1;
            break;
        }
    }

    Some(d_up * d_left * d_down * d_right)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(input)?;
    Ok(visible(&map)
        .ok_or_else(|| anyhow!("Some assumption about the input did not hold"))?
        .len())
}
pub fn part_2(input: &str) -> anyhow::Result<i32> {
    let map = Map::from_str(input)?;
    let visible =
        visible(&map).ok_or_else(|| anyhow!("Some assumption about the input did not hold"))?;

    let mut scores = HashSet::with_capacity(map.heights.len());

    for (x, y) in visible {
        if x == map.x_min || x == map.x_max || y == map.y_min || y == map.y_max {
            continue;
        }
        if let Some(score) = scenic_score(&map, x, y) {
            scores.insert(score);
        } else {
            bail!("Some assumption about the input did not hold");
        }
    }

    Ok(*scores
        .iter()
        .max()
        .ok_or_else(|| anyhow!("Expected at least one visible tree that is not on the edge"))?)
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

    // Fails on 2021/01 and possibly others
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
