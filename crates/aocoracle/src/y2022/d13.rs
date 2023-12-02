use anyhow::anyhow;

use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn unit_list(x: i32) -> Self {
        Self::List(vec![Self::Integer(x)])
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(x) => write!(f, "{x}"),
            Packet::List(xs) => write!(f, "{xs:?}"),
        }
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = serde_json::from_str(s)?;
        match result {
            Packet::Integer(_) => Err(anyhow!(
                "Expected outermost packet to be of the list variety"
            )),
            Packet::List(_) => Ok(result),
        }
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
            (Packet::Integer(l), Packet::List(_)) => Packet::unit_list(*l).cmp(other),
            (Packet::List(_), Packet::Integer(r)) => self.cmp(&Packet::unit_list(*r)),
            (Packet::List(ls), Packet::List(rs)) => {
                for (l, r) in ls.iter().zip(rs) {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }
                ls.len().cmp(&rs.len())
            }
        }
    }
}

fn packet_pairs(s: &str) -> anyhow::Result<Vec<(Packet, Packet)>> {
    let mut result = Vec::new();
    for pair in s.split("\n\n") {
        let (left, right) = pair
            .split_once('\n')
            .ok_or_else(|| anyhow!("Expected exactly two lines"))?;
        let left = left.parse()?;
        let right = right.parse()?;
        result.push((left, right));
    }
    Ok(result)
}

fn packets(s: &str) -> anyhow::Result<Vec<Packet>> {
    let mut result = Vec::new();
    for line in s.lines().filter(|l| !l.is_empty()) {
        result.push(line.parse()?);
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let pairs = packet_pairs(input)?;
    Ok(pairs
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| if l < r { i + 1 } else { 0 })
        .sum())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut packets = packets(input)?;
    let div1: Packet = "[[2]]".parse().expect("Hard coded packet is valid");
    let div2: Packet = "[[6]]".parse().expect("Hard coded packet is valid");
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let pos1 = packets
        .iter()
        .position(|s| *s == div1)
        .expect("Needle inserted in haystack above")
        + 1;
    let pos2 = packets
        .iter()
        .position(|s| *s == div2)
        .expect("Needle inserted in haystack above")
        + 1;
    Ok(pos1 * pos2)
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
        assert_correct_answer_on_correct_input!(part_1, "cb26c4d83d6ff809", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "cb26c4d83d6ff809", Part::Two);
    }

    // Fails on 2021/18/3ba7923eae and possibly others
    // Both are nested lists of integers
    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
