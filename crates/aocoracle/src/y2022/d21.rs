use anyhow::bail;
use hashbrown::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn call(&self, lhs: u64, rhs: u64) -> u64 {
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
    Operand(u64),
    Expression(String, Operation, String),
}

fn jobs(s: &str) -> anyhow::Result<HashMap<String, Job>> {
    let mut result = HashMap::new();
    for line in s.lines() {
        let (id, job) = line.split_once(':').unwrap();
        let id = id.to_string();
        let job = match job.trim().parse() {
            Ok(x) => Job::Operand(x),
            Err(_) => {
                let mut parts = job.trim().split_whitespace();
                Job::Expression(
                    parts.next().unwrap().to_string(),
                    match parts.next().unwrap() {
                        "+" => Operation::Add,
                        "-" => Operation::Sub,
                        "*" => Operation::Mul,
                        "/" => Operation::Div,
                        _ => panic!("Expected +-*/ but got"),
                    },
                    parts.next().unwrap().to_string(),
                )
            }
        };
        result.insert(id, job);
    }
    Ok(result)
}

fn evaluate(jobs: &HashMap<String, Job>, job: &String) -> u64 {
    match jobs.get(job).unwrap() {
        Job::Expression(lhs, op, rhs) => op.call(evaluate(jobs, &lhs), evaluate(jobs, &rhs)),
        Job::Operand(x) => *x,
    }
}

pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let jobs = jobs(input)?;
    Ok(evaluate(&jobs, &"root".to_string()))
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut jobs = jobs(input)?;
    let root = "root".to_string();
    let humn = "humn".to_string();
    let (lhs, rhs) = match jobs.get(&root).unwrap() {
        Job::Operand(_) => panic!("Oops"),
        Job::Expression(lhs, _, rhs) => (lhs.clone(), rhs.clone()),
    };
    let lhs = evaluate(&jobs, &lhs);
    for x in 0.. {
        jobs.insert(humn.clone(), Job::Operand(x));
        let rhs = evaluate(&jobs, &rhs);
        if lhs == rhs {
            return Ok(x);
        }
    }
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
