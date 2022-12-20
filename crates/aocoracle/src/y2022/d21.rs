use anyhow::{anyhow, bail};
use hashbrown::HashMap;

use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "/" => Ok(Self::Div),
            "*" => Ok(Self::Mul),
            "-" => Ok(Self::Sub),
            other => Err(anyhow!(
                "Expected operator '+', '/', '*', '-' but got {other}"
            )),
        }
    }
}

impl Operator {
    fn call(&self, lhs: i128, rhs: i128) -> i128 {
        match self {
            Self::Add => lhs + rhs,
            Self::Div => lhs / rhs,
            Self::Mul => lhs * rhs,
            Self::Sub => lhs - rhs,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Job {
    Operand(i128),
    Operation(Operator, u32, u32),
}

impl FromStr for Job {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim().parse() {
            Ok(x) => Job::Operand(x),
            Err(_) => {
                let tokens: Vec<_> = s.split_whitespace().collect();
                if tokens.len() != 3 {
                    bail!("Expected expression with 3 token but got {}", tokens.len());
                }
                Job::Operation(
                    Operator::from_str(tokens[1])?,
                    u32::from_str_radix(tokens[0], 36)?,
                    u32::from_str_radix(tokens[2], 36)?,
                )
            }
        })
    }
}

fn root() -> u32 {
    u32::from_str_radix("root", 36).expect("Hard coded value is valid")
}

fn jobs(s: &str) -> anyhow::Result<HashMap<u32, Job>> {
    let mut result = HashMap::new();
    for line in s.lines() {
        let (id, job) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("Expected exactly two parts separated by ':' but got {s}"))?;
        let id = u32::from_str_radix(id, 36)?;
        let job = Job::from_str(job)?;
        result.insert(id, job);
    }
    if !result.contains_key(&root()) {
        bail!("Expected expression to have a root")
    }
    Ok(result)
}

fn evaluate(jobs: &HashMap<u32, Job>, job: &u32) -> i128 {
    match jobs.get(job).unwrap() {
        Job::Operation(op, lhs, rhs) => op.call(evaluate(jobs, lhs), evaluate(jobs, rhs)),
        Job::Operand(x) => *x,
    }
}

pub fn part_1(input: &str) -> anyhow::Result<i128> {
    let jobs = jobs(input)?;
    Ok(evaluate(&jobs, &root()))
}

fn binary_search<F>(mut lo: i128, mut hi: i128, mut cmp: F) -> Option<i128>
where
    F: FnMut(i128) -> Ordering,
{
    while lo != hi {
        let mid = (lo + hi) / 2;
        match cmp(mid) {
            // Always return the leftmost result
            Ordering::Equal => hi = mid,
            Ordering::Less => hi = mid - 1,
            Ordering::Greater => lo = mid + 1,
        }
    }
    match cmp(lo) {
        Ordering::Equal => Some(lo),
        Ordering::Less => None,
        Ordering::Greater => None,
    }
}

pub fn part_2(input: &str) -> anyhow::Result<i128> {
    let mut jobs = jobs(input)?;
    let humn = u32::from_str_radix("humn", 36)?;
    let (lhs, rhs) = match jobs.get(&root()).unwrap() {
        Job::Operand(_) => panic!("Oops"),
        Job::Operation(_, lhs, rhs) => (*lhs, *rhs),
    };

    let old = evaluate(&jobs, &lhs).cmp(&evaluate(&jobs, &rhs));
    let answer = binary_search(0, i64::MAX as i128, |x| {
        jobs.insert(humn, Job::Operand(x));
        let new = evaluate(&jobs, &lhs).cmp(&evaluate(&jobs, &rhs));
        match new {
            Ordering::Equal => Ordering::Equal,
            new => {
                if old == new {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    })
    .unwrap();
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
