use anyhow::Context;
use hashbrown::HashMap;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    orderings: HashMap<i64, HashSet<i64>>,
    groups: Vec<Vec<i64>>,
}
fn parse(input: &str) -> anyhow::Result<Input> {
    let mut orderings = HashMap::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (l, r) = line.split_once("|").context("Cannot split")?;
        let l = l.parse::<i64>()?;
        let r = r.parse::<i64>()?;
        orderings.entry(l).or_insert(HashSet::new()).insert(r);
    }

    let mut pages = Vec::new();
    while let Some(line) = lines.next() {
        let mut row = Vec::new();
        for page in line.split(",") {
            row.push(page.parse::<i64>()?);
        }
        pages.push(row);
    }

    Ok(Input {
        orderings,
        groups: pages,
    })
}

fn is_ordered(orderings: &HashMap<i64, HashSet<i64>>, pages: &[i64]) -> bool {
    for i in 0..pages.len() {
        let curr = pages[i];
        for later in pages[i..pages.len()].iter() {
            if let Some(constraints) = orderings.get(&later) {
                if constraints.contains(&curr) {
                    return false;
                }
            }
        }
    }
    true
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let Input { orderings, groups } = dbg!(parse(input)?);
    Ok(groups
        .into_iter()
        .filter(|g| dbg!(is_ordered(&orderings, dbg!(g))))
        .map(|g| dbg!(g[dbg!(g.len() / 2)]))
        .sum())
}

fn topological_order(mut orderings: HashMap<i64, HashSet<i64>>, group: &[i64]) -> Vec<i64> {
    let mut sorted = Vec::new();
    let mut nodes: HashSet<_> = orderings.keys().cloned().collect();
    nodes.extend(orderings.values().flatten());

    let incoming: Vec<_> = orderings.keys().cloned().sorted().collect();
    let outgoing: Vec<_> = orderings
        .values()
        .flatten()
        .unique()
        .sorted()
        .cloned()
        .collect();
    // Seems there are cycles in the input
    dbg!(&incoming, &outgoing, incoming == outgoing);

    let group: HashSet<_> = group.into_iter().cloned().collect();
    let mut unconstrained: Vec<_> = nodes
        .into_iter()
        .filter(|n| {
            orderings
                .get(n)
                .map(|vs| vs.intersection(&group).count() == 0)
                .unwrap_or(true)
        })
        .sorted()
        .collect();
    assert!(!unconstrained.is_empty());
    while let Some(node) = unconstrained.pop() {
        sorted.push(node);
        for (k, vs) in orderings.iter_mut() {
            if vs.remove(&node) && vs.is_empty() {
                unconstrained.push(*k);
            }
        }
    }
    sorted
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let Input {
        orderings,
        mut groups,
    } = parse(input)?;
    groups.retain(|g| !is_ordered(&orderings, g));

    let mut sum = 0;
    for group in groups {
        let sorted: Vec<_> = group
            .iter()
            .cloned()
            .sorted_by(|l, r| {
                if let Some(after) = orderings.get(&l) {
                    if after.contains(r) {
                        return Ordering::Less;
                    }
                }
                if let Some(after) = orderings.get(&r) {
                    if after.contains(&l) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            })
            .collect();
        if sorted != group {
            sum += sorted[group.len() / 2]
        }
    }
    Ok(sum)
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
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "INPUT", Part::Two);
        // 5128 is too high
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
