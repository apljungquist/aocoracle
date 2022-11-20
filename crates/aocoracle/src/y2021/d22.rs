use std::cmp;
use std::fmt::Debug;

use hashbrown::HashMap;

use crate::AnyError;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Cuboid {
    xl: i64,
    xr: i64,
    yl: i64,
    yr: i64,
    zl: i64,
    zr: i64,
}

impl Cuboid {
    fn new(xl: i64, xr: i64, yl: i64, yr: i64, zl: i64, zr: i64) -> Option<Cuboid> {
        if xr <= xl || yr <= yl || zr <= zl {
            return None;
        }
        Some(Cuboid {
            xl,
            xr,
            yl,
            yr,
            zl,
            zr,
        })
    }
}

impl Cuboid {
    fn contains_cuboid(&self, other: &Cuboid) -> bool {
        self.xl <= other.xl
            && self.yl <= other.yl
            && self.zl <= other.zl
            && other.xr <= self.xr
            && other.yr <= self.yr
            && other.zr <= self.zr
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        Cuboid::new(
            cmp::max(self.xl, other.xl),
            cmp::min(self.xr, other.xr),
            cmp::max(self.yl, other.yl),
            cmp::min(self.yr, other.yr),
            cmp::max(self.zl, other.zl),
            cmp::min(self.zr, other.zr),
        )
    }

    fn volume(&self) -> i64 {
        (self.xr - self.xl) * (self.yr - self.yl) * (self.zr - self.zl)
    }
}

fn _steps(text: &str) -> Result<Vec<(bool, Cuboid)>, AnyError> {
    let re = regex::Regex::new(
        r"(on|off) x=(-?\d+)[.][.](-?\d+),y=(-?\d+)[.][.](-?\d+),z=(-?\d+)[.][.](-?\d+)",
    )
    .unwrap();
    let mut result = Vec::new();
    for cap in re.captures_iter(text) {
        let state = match &cap[1] {
            "on" => true,
            "off" => false,
            state => panic!("Unexpected state '{}'", state),
        };
        let cuboid = Cuboid::new(
            cap[2].parse::<i64>()?,
            cap[3].parse::<i64>()? + 1,
            cap[4].parse::<i64>()?,
            cap[5].parse::<i64>()? + 1,
            cap[6].parse::<i64>()?,
            cap[7].parse::<i64>()? + 1,
        )
        .ok_or_else(|| format!("Not a valid cuboid: {:?}", cap))?;
        result.push((state, cuboid));
    }
    if result.len() != text.lines().count() {
        return Err("Expected one cuboid per line".into());
    }
    Ok(result)
}

#[allow(clippy::needless_collect)]
fn _num_on(steps: Vec<(bool, Cuboid)>) -> i64 {
    let mut counts: HashMap<Cuboid, i64> = HashMap::new();
    for (state, new_cuboid) in steps {
        let changes = counts
            .iter()
            .filter_map(|(old_cuboid, old_count)| {
                old_cuboid
                    .intersection(&new_cuboid)
                    .map(|intersection| (intersection, *old_count))
            })
            .collect::<Vec<(Cuboid, i64)>>();

        for (intersection, old_count) in changes.into_iter() {
            *counts.entry(intersection).or_insert(0) -= old_count;
        }

        if state {
            *counts.entry(new_cuboid).or_insert(0) += 1;
        }

        // Make the quadratic runtime somewhat less excruciating
        counts.retain(|_, v| *v != 0);
    }
    counts
        .into_iter()
        .map(|(cuboid, count)| cuboid.volume() * count)
        .sum()
}

pub fn part_1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bound = Cuboid::new(-50, 51, -50, 51, -50, 51).unwrap();
    let steps = _steps(input)?
        .into_iter()
        .filter(|(_, cuboid)| bound.contains_cuboid(cuboid))
        .collect();
    Ok(format!("{}", _num_on(steps)))
}

pub fn part_2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let steps = _steps(input)?;
    Ok(format!("{}", _num_on(steps)))
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example_s() {
        assert_correct_answer_on_correct_input!(part_1, "example_s", Part::One);
    }
    #[test]
    fn part_1_works_on_example_m() {
        assert_correct_answer_on_correct_input!(part_1, "example_m", Part::One);
    }
    #[test]
    fn part_1_works_on_example_l() {
        assert_correct_answer_on_correct_input!(part_1, "example_l", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "6bb0c0bd67", Part::One);
    }

    #[test]
    fn part_2_works_on_example_l() {
        assert_correct_answer_on_correct_input!(part_2, "example_l", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6bb0c0bd67", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(&part_1, &part_2);
    }
}
