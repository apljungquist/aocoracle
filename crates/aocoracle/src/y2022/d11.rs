use anyhow::bail;
use hashbrown::HashMap;
use std::collections::VecDeque;
use std::iter;
use std::str::FromStr;

#[derive(Debug)]
enum Operand {
    Const(i32),
    Old,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<i32>,
    lhs: Operand,
    op: Operation,
    rhs: Operand,
    test: i32,
    destination_true: usize,
    destination_false: usize,
}

impl Monkey {
    fn inspect_and_throw(&mut self) -> Option<(usize, i32)> {
        let old = self.items.pop_front()?;
        let new = match (&self.op, &self.lhs, &self.rhs) {
            (Operation::Add, Operand::Const(lhs), Operand::Const(rhs)) => lhs + rhs,
            (Operation::Add, Operand::Const(lhs), Operand::Old) => lhs + old,
            (Operation::Add, Operand::Old, Operand::Const(rhs)) => old + rhs,
            (Operation::Add, Operand::Old, Operand::Old) => old + old,
            (Operation::Mul, Operand::Const(lhs), Operand::Const(rhs)) => lhs * rhs,
            (Operation::Mul, Operand::Const(lhs), Operand::Old) => lhs * old,
            (Operation::Mul, Operand::Old, Operand::Const(rhs)) => old * rhs,
            (Operation::Mul, Operand::Old, Operand::Old) => old * old,
        } / 3;
        if new % self.test == 0 {
            Some((self.destination_true, new))
        } else {
            Some((self.destination_false, new))
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .0
            .split_once(' ')
            .unwrap()
            .1
            .parse()?;
        // let re_id = regex::Regex::new(r"^Monkey (\d+):$").expect("Hard coded regex is valid");
        // let cap_id = re_id.captures(lines.next().unwrap()).unwrap();
        // let id = cap_id[1].parse()?;

        let mut items = VecDeque::new();
        for item in lines.next().unwrap().split_once(':').unwrap().1.split(", ") {
            items.push_back(item.trim().parse()?);
        }

        let operation_parts: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect();
        let op = match operation_parts[3] {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            operation => {
                bail!("but got {operation}")
            }
        };
        let lhs = match operation_parts[2] {
            "old" => Operand::Old,
            x => Operand::Const(x.parse()?),
        };
        let rhs = match operation_parts[4] {
            "old" => Operand::Old,
            x => Operand::Const(x.parse()?),
        };

        let test_parts: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect();
        let test = test_parts[2].parse()?;

        let destination_true_parts: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect();
        let destination_false_parts: Vec<_> = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect();
        let destination_true = destination_true_parts[3].parse()?;
        let destination_false = destination_false_parts[3].parse()?;

        Ok(Monkey {
            id,
            items,
            lhs,
            op,
            rhs,
            test,
            destination_true,
            destination_false,
        })
    }
}

fn monkeys(s: &str) -> anyhow::Result<Vec<Monkey>> {
    let mut result = Vec::new();
    for monkey in s.split("\n\n") {
        let monkey = Monkey::from_str(monkey)?;
        result.push(monkey);
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut monkeys = dbg!(monkeys(input)?);
    let mut counts: Vec<usize> = iter::repeat(0).take(monkeys.len()).collect();
    for _ in 0..20 {
        for src in 0..monkeys.len() {
            while let Some((dst, lvl)) = monkeys[src].inspect_and_throw() {
                counts[src] += 1;
                monkeys[dst].items.push_back(lvl);
            }
        }
    }
    dbg!(&counts);
    counts.sort();
    let first = counts.pop().unwrap();
    let second = counts.pop().unwrap();
    Ok(first * second)
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    Ok("".to_string())
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
