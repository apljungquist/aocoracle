use crate::y2022::d19::Resource::Clay;
use anyhow::anyhow;
use hashbrown::{HashMap, HashSet};

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: HashMap<Resource, HashMap<Resource, usize>>,
    // ore_per_ore: usize,
    // ore_per_clay: usize,
    // ore_per_obsidian: usize,
    // clay_per_obsidian: usize,
    // ore_per_geode: usize,
    // obsidian_per_geode: usize,
}

fn blueprints(s: &str) -> anyhow::Result<Vec<Blueprint>> {
    let re = regex::Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture a blueprint on line {}", line))?;
        let mut costs = HashMap::new();
        costs.insert(
            Resource::Ore,
            [(Resource::Ore, cap[2].parse()?)].into_iter().collect(),
        );
        costs.insert(
            Resource::Clay,
            [(Resource::Ore, cap[3].parse()?)].into_iter().collect(),
        );
        costs.insert(
            Resource::Obsidian,
            [
                (Resource::Ore, cap[4].parse()?),
                (Resource::Clay, cap[5].parse()?),
            ]
            .into_iter()
            .collect(),
        );
        costs.insert(
            Resource::Geode,
            [
                (Resource::Ore, cap[6].parse()?),
                (Resource::Obsidian, cap[7].parse()?),
            ]
            .into_iter()
            .collect(),
        );
        result.push(Blueprint {
            id: cap[1].parse()?,
            costs: costs,
            //     ore_per_obsidian: cap[4].parse()?,
            //     clay_per_obsidian: cap[5].parse()?,
            //     ore_per_geode: cap[6].parse()?,
            //     obsidian_per_geode: cap[7].parse()?,
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

fn is_affordable(has: &HashMap<Resource, usize>, needs: &HashMap<Resource, usize>) -> bool {
    for (resource, num) in needs {
        if has.get(resource).unwrap_or(&0) < num {
            return false;
        }
    }
    return true;
}

fn with_production(
    resources: &HashMap<Resource, usize>,
    robots: &HashMap<Resource, usize>,
) -> HashMap<Resource, usize> {
    let mut resources = resources.clone();
    for (robot, num) in robots.iter() {
        *resources.entry(*robot).or_insert(0) += num;
    }
    resources
}

fn without_expenditure(
    resources: &HashMap<Resource, usize>,
    cost: &HashMap<Resource, usize>,
) -> HashMap<Resource, usize> {
    let mut resources = resources.clone();
    for (resource, num) in cost.iter() {
        *resources.get_mut(resource).unwrap() -= num;
    }
    resources
}

fn unaffordable_robots(resources: &HashMap<Resource, usize>, blueprint: &Blueprint) -> Vec<Resource> {
    [
        Resource::Geode,
        Resource::Obsidian,
        Resource::Clay,
        Resource::Ore,
    ]
    .into_iter()
    .filter(|r| !is_affordable(&resources, &blueprint.costs[r]))
    .collect()
}

fn new_time_remaining(time_remaining:usize)->usize{
    time_remaining-1
}

fn num_geode(
    blueprint: &Blueprint,
    resources: HashMap<Resource, usize>,
    robots: HashMap<Resource, usize>,
    unaffordable: Vec<Resource>,
    time_remaining: usize,
    mut best: usize,
) -> Option<usize> {
    if time_remaining == 0 {
        return Some(*resources.get(&Resource::Geode).unwrap_or(&0));
    }

    // Suboptimal
    if unaffordable.is_empty() {
        return None;
    }

    // let new_resources = with_production(&resources, &robots);
    let new_time_remaining = time_remaining - 1;

    // let num_geode_now = *resources.get(&Resource::Geode).unwrap_or(&0);
    // let num_geode_robot_now = *resources.get(&Resource::Geode).unwrap_or(&0);
    // let num_geode_produced = time_remaining * (2 * num_geode_robot_now + time_remaining - 1) / 2;
    // let num_geode_end = num_geode_now + num_geode_produced;
    // if num_geode_end <= best {
    //     return None;
    // }

    for robot in unaffordable {
        if is_affordable(&resources, &blueprint.costs[&robot]) {
            let new_resources = without_expenditure(&resources, &blueprint.costs[&robot]);
            let new_resources = with_production(&new_resources, &robots);

            let mut new_robots = robots.clone();
            *new_robots.entry(robot).or_insert(0) += 1;

            let new_unaffordable = unaffordable_robots(&resources, blueprint);

            if let Some(candidate) = num_geode(
                blueprint,
                new_resources,
                new_robots,
                new_unaffordable,
                new_time_remaining,
                best,
            ) {
                best = best.max(candidate);
            }
            break;
        }
    }
    let new_resources = with_production(&resources, &robots);
    let new_robots = robots;
    let new_unaffordable = unaffordable_robots(&resources, blueprint);
    if let Some(candidate) = num_geode(blueprint, new_resources, new_robots, new_unaffordable, new_time_remaining, best) {
        best = best.max(candidate);
    }
    Some(best)
}

// One ore-collecting robot
pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let blueprints = blueprints(input)?;
    let factors: Vec<_> = blueprints
        .iter()
        .map(|b| {
            (
                b.id,
                num_geode(
                    &b,
                    HashMap::new(),
                    [(Resource::Ore, 1)].into_iter().collect(),
                    vec![Resource::Geode,Resource::Obsidian,Resource::Clay,Resource::Ore],
                    24,
                    0,
                )
                .unwrap(),
            )
        })
        .collect();
    dbg!(&factors);
    let quality_levels: Vec<_> = factors.iter().map(|(i, n)| i * n).collect();
    dbg!(&quality_levels);
    Ok(quality_levels.into_iter().sum())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
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
