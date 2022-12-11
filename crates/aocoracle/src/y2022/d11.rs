use anyhow::{anyhow, bail};

use std::collections::VecDeque;

use std::str::{FromStr, Lines};

#[derive(Debug)]
enum Operand {
    Const(i64),
    Old,
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            x => Ok(Self::Const(x.parse()?)),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}
impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            other => Err(anyhow!("Expected operator '+' or '*' but got {other}")),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<i64>,
    lhs: Operand,
    op: Operation,
    rhs: Operand,
    test: i64,
    destination_true: usize,
    destination_false: usize,
}

impl Monkey {
    fn inspect_and_throw(&mut self, denominator: i64, modulus: i64) -> Option<(usize, i64)> {
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
        } / denominator
            % modulus;
        if new % self.test == 0 {
            Some((self.destination_true, new))
        } else {
            Some((self.destination_false, new))
        }
    }
}

fn take_line<'a>(lines: &'a mut Lines, prefix: &str) -> anyhow::Result<&'a str> {
    let line = lines
        .next()
        .ok_or_else(|| anyhow!("Expected line starting with '{prefix}' but got no line"))?;
    line.strip_prefix(prefix)
        .ok_or_else(|| anyhow!("Expected line starting with '{prefix}' but got '{line}'"))
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id = take_line(&mut lines, "Monkey ")?
            .strip_suffix(':')
            .ok_or_else(|| anyhow!("Expected line ending with ':'"))?
            .parse()?;

        let mut items = VecDeque::new();
        for item in take_line(&mut lines, "  Starting items: ")?.split(',') {
            items.push_back(item.trim().parse()?);
        }

        let expression: Vec<_> = take_line(&mut lines, "  Operation: new = ")?
            .split_whitespace()
            .collect();
        if expression.len() != 3 {
            bail!(
                "Expected expression with 3 token but got {}",
                expression.len()
            );
        }
        let op = Operation::from_str(expression[1])?;
        let lhs = Operand::from_str(expression[0])?;
        let rhs = Operand::from_str(expression[2])?;

        let test = take_line(&mut lines, "  Test: divisible by ")?.parse()?;

        let destination_true = take_line(&mut lines, "    If true: throw to monkey ")?.parse()?;
        let destination_false = take_line(&mut lines, "    If false: throw to monkey ")?.parse()?;

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
    for (i, monkey) in s.split("\n\n").enumerate() {
        let monkey = Monkey::from_str(monkey)?;
        if monkey.id != i {
            bail!("Expected monkey id {i} but got {0}", monkey.id);
        }
        result.push(monkey);
    }
    Ok(result)
}

fn monkey_business(
    mut monkeys: Vec<Monkey>,
    denominator: i64,
    num_round: usize,
) -> anyhow::Result<usize> {
    if monkeys.len() < 2 {
        bail!("Expected at least 2 monkeys but got {}", monkeys.len());
    }
    let mut counts = vec![0; monkeys.len()];
    let modulus = monkeys.iter().map(|m| m.test).product();
    for _ in 0..num_round {
        for src in 0..monkeys.len() {
            while let Some((dst, lvl)) = monkeys[src].inspect_and_throw(denominator, modulus) {
                counts[src] += 1;
                monkeys[dst].items.push_back(lvl);
            }
        }
    }
    counts.sort();
    let first = counts.pop().expect("Length checked above");
    let second = counts.pop().expect("Length checked above");
    Ok(first * second)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let monkeys = monkeys(input)?;
    monkey_business(monkeys, 3, 20)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let monkeys = monkeys(input)?;
    monkey_business(monkeys, 1, 10000)
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
