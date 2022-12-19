use anyhow::anyhow;
use hashbrown::{HashMap, HashSet};
use std::ops::Add;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: HashMap<Resource, Point>,
}

fn blueprints(s: &str) -> anyhow::Result<Vec<Blueprint>> {
    let re = regex::Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture a blueprint on line {}", line))?;

        result.push(Blueprint {
            id: cap[1].parse()?,
            costs: [
                (Resource::Ore, Point::new(cap[2].parse()?, 0, 0, 0)),
                (Resource::Clay, Point::new(cap[3].parse()?, 0, 0, 0)),
                (
                    Resource::Obsidian,
                    Point::new(cap[4].parse()?, cap[5].parse()?, 0, 0),
                ),
                (
                    Resource::Geode,
                    Point::new(cap[6].parse()?, 0, cap[7].parse()?, 0),
                ),
            ]
            .into_iter()
            .collect(),
        });
    }
    Ok(result)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn point(&self) -> Point {
        match self {
            Self::Ore => Point::new(1, 0, 0, 0),
            Self::Clay => Point::new(0, 1, 0, 0),
            Self::Obsidian => Point::new(0, 0, 1, 0),
            Self::Geode => Point::new(0, 0, 0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Point {
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

impl Point {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(rhs.ore)?,
            clay: self.clay.checked_sub(rhs.clay)?,
            obsidian: self.obsidian.checked_sub(rhs.obsidian)?,
            geode: self.geode.checked_sub(rhs.geode)?,
        })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    resources: Point,
    robots: Point,
}

impl State {
    fn updated(&self, action: Option<Resource>, blueprint: &Blueprint) -> Option<Self> {
        let removed_resources = match action {
            Some(robot) => blueprint.costs[&robot],
            None => Point::new(0, 0, 0, 0),
        };
        let added_resources = self.robots;
        let resources = self.resources.checked_sub(&removed_resources)? + added_resources;

        let added_robots = match action {
            None => Point::new(0, 0, 0, 0),
            Some(robot) => robot.point(),
        };
        let robots = self.robots + added_robots;

        Some(Self { resources, robots })
    }

    fn successors(&self, blueprint: &Blueprint, max_cost: &Point) -> Vec<Self> {
        if self
            .resources
            .checked_sub(&blueprint.costs[&Resource::Geode])
            .is_some()
        {
            return vec![self.updated(Some(Resource::Geode), blueprint).unwrap()];
        }

        let mut actions = Vec::with_capacity(5);
        if self.robots.ore < max_cost.ore {
            actions.push(Some(Resource::Ore));
        }
        if self.robots.clay < max_cost.clay {
            actions.push(Some(Resource::Clay));
        }
        if self.robots.obsidian < max_cost.obsidian {
            actions.push(Some(Resource::Obsidian));
        }
        actions
            .into_iter()
            .filter_map(|a| self.updated(a, blueprint))
            .collect()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            resources: Point::new(0, 0, 0, 0),
            robots: Point::new(1, 0, 0, 0),
        }
    }
}

fn num_geode(blueprint: &Blueprint, max_depth: usize) -> Option<usize> {
    let mut done = HashMap::new();
    let mut todo = vec![(State::default(), 0)];
    let max_cost = Point::new(
        [
            blueprint.costs[&Resource::Clay].ore,
            blueprint.costs[&Resource::Obsidian].ore,
            blueprint.costs[&Resource::Geode].ore,
        ]
        .into_iter()
        .max()
        .unwrap(),
        blueprint.costs[&Resource::Obsidian].clay,
        blueprint.costs[&Resource::Geode].obsidian,
        0,
    );
    while let Some((state, curr_distance)) = todo.pop() {
        if let Some(best_distance) = done.get(&state) {
            if *best_distance <= curr_distance {
                continue;
            }
        }
        done.insert(state, curr_distance);
        if curr_distance == max_depth {
            continue;
        }

        for successor in state.successors(blueprint, &max_cost) {
            todo.push((successor, curr_distance + 1));
        }
    }
    done.keys().map(|s| s.resources.geode).max()
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let blueprints = blueprints(input)?;
    let factors: Vec<_> = blueprints
        .iter()
        .map(|b| (b.id, num_geode(&b, 24).unwrap()))
        .collect();
    let quality_levels: Vec<_> = factors.iter().map(|(i, n)| i * n).collect();
    let answer = quality_levels.into_iter().sum();
    // assert!(answer == 33 || 824 < answer);
    Ok(answer)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let blueprints = blueprints(input)?;
    let factors: Vec<_> = blueprints[..3.min(blueprints.len())]
        .iter()
        .map(|b| num_geode(&b, 32).unwrap())
        .collect();
    dbg!(&factors);
    let answer = factors.into_iter().product();
    Ok(answer)
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
