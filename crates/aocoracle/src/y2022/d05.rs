use std::str::Lines;

use anyhow::{anyhow, bail};
use hashbrown::HashMap;

fn take_stacks(lines: &mut Lines) -> anyhow::Result<HashMap<usize, Vec<char>>> {
    let mut result = HashMap::<usize, Vec<char>>::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        for (i, char) in line.chars().enumerate() {
            match (i % 4, char) {
                (0, ' ') => {}
                (0, '[') => {}
                (0, _) => bail!("Expected one of '[ ' but got {char}"),
                (1, ' ') => {}
                (1, _) => result.entry(i / 4 + 1).or_default().push(char),
                (2, ' ') => {}
                (2, ']') => {}
                (2, _) => bail!("Expected one of '] ' but got {char}"),
                (3, ' ') => {}
                (3, _) => bail!("Expected ' ' nut got {char}"),
                _ => unreachable!(),
            };
        }
    }
    for stack in result.values_mut() {
        stack.pop();
        stack.reverse();
    }
    Ok(result)
}

fn take_procedure(lines: &mut Lines) -> anyhow::Result<Vec<(usize, usize, usize)>> {
    let re =
        regex::Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Hard coded regex is valid");
    let mut result = Vec::<(usize, usize, usize)>::new();
    for line in lines {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture line {}", line))?;
        result.push((cap[1].parse()?, cap[2].parse()?, cap[3].parse()?))
    }
    Ok(result)
}

fn part_x(input: &str, preserve_order: bool) -> anyhow::Result<String> {
    let mut lines = input.lines();
    let mut stacks = take_stacks(&mut lines)?;
    let steps = take_procedure(&mut lines)?;

    for (num, src, dst) in steps {
        let src = stacks
            .get_mut(&src)
            .ok_or_else(|| anyhow!("Expected step to reference existing stack but got {src}"))?;
        let mut moved = Vec::with_capacity(num);
        for _ in 0..num {
            moved.push(
                src.pop()
                    .ok_or_else(|| anyhow!("Expected step to reference non-empty stack"))?,
            );
        }
        if preserve_order {
            moved.reverse();
        }
        let dst = stacks
            .get_mut(&dst)
            .ok_or_else(|| anyhow!("Expected step to reference existing stack but got {dst}"))?;
        dst.append(&mut moved);
    }

    let mut result = String::new();
    for i in 1.. {
        if let Some(letter) = stacks.get(&i).and_then(|stack| stack.last()) {
            result.push(*letter);
        } else {
            break;
        }
    }
    Ok(result)
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    part_x(input, false)
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    part_x(input, true)
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
