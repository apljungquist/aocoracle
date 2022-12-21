use anyhow::bail;
use hashbrown::HashMap;
use pathfinding::prelude::bfs;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn call(&self, lhs: i128, rhs: i128) -> i128 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Job {
    Operand(i128),
    Expression(u32, Operation, u32),
}

fn jobs(s: &str) -> anyhow::Result<HashMap<u32, Job>> {
    let mut result = HashMap::new();
    for line in s.lines() {
        let (id, job) = line.split_once(':').unwrap();
        let id = u32::from_str_radix(id, 36)?;
        let job = match job.trim().parse() {
            Ok(x) => Job::Operand(x),
            Err(_) => {
                let mut parts = job.trim().split_whitespace();
                Job::Expression(
                    u32::from_str_radix(parts.next().unwrap(), 36)?,
                    match parts.next().unwrap() {
                        "+" => Operation::Add,
                        "-" => Operation::Sub,
                        "*" => Operation::Mul,
                        "/" => Operation::Div,
                        _ => panic!("Expected +-*/ but got"),
                    },
                    u32::from_str_radix(parts.next().unwrap(), 36)?,
                )
            }
        };
        result.insert(id, job);
    }
    Ok(result)
}

fn evaluate(jobs: &HashMap<u32, Job>, job: &u32) -> i128 {
    match jobs.get(job).unwrap() {
        Job::Expression(lhs, op, rhs) => op.call(evaluate(jobs, &lhs), evaluate(jobs, &rhs)),
        Job::Operand(x) => *x,
    }
}

pub fn part_1(input: &str) -> anyhow::Result<i128> {
    let jobs = jobs(input)?;
    let root = u32::from_str_radix("root", 36)?;
    Ok(evaluate(&jobs, &root))
}

pub fn part_2(input: &str) -> anyhow::Result<i128> {
    let mut jobs = jobs(input)?;
    let root = u32::from_str_radix("root", 36)?;
    let humn = u32::from_str_radix("humn", 36)?;

    let path = bfs(
        &root,
        |id| match jobs.get(id).unwrap() {
            Job::Operand(_) => vec![],
            Job::Expression(lhs, _, rhs) => vec![*lhs, *rhs],
        },
        |id| *id == humn,
    )
    .unwrap();

    assert_eq!(path[0], root);
    let (lhs, rhs) = match jobs.get(&root).unwrap() {
        Job::Operand(_) => panic!("Oops"),
        Job::Expression(lhs, _, rhs) => (lhs.clone(), rhs.clone()),
    };
    let (target, candidate) = if path[1] == lhs {
        (rhs, lhs)
    } else if path[1] == rhs {
        (lhs, rhs)
    } else {
        panic!("Oops");
    };
    let target = evaluate(&jobs, &target);
    // let mut l = i64::MIN as i128;
    // let mut r = 3469704905529;
    // while l!=r {
    //     let m = (l+r)/2;
    //     jobs.insert(humn.clone(), Job::Operand(m));
    //     let candidate = evaluate(&jobs, &candidate);
    //     match candidate.cmp(&target) {
    //         Ordering::Equal=>{
    //             dbg!(m, target, candidate);
    //             return Ok(m)
    //         },
    //         Ordering::Less=>{l=m;},
    //         Ordering::Greater=>{r=m;},
    //     }
    // }
    for x in (0..=3469704905531).rev() {
        jobs.insert(humn.clone(), Job::Operand(x));
        let candidate = evaluate(&jobs, &candidate);
        if candidate != target {
            break;
        }
        println!("{x}");
        println!("{:>20}", target);
        println!("{:>20}", candidate);
    }
    // answer < 3469704905531
    //          3469704905529
    bail!("Search ended")
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
