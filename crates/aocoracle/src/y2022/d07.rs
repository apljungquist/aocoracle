use anyhow::{anyhow, bail};
use hashbrown::HashMap;
use itertools::Itertools;

enum Line {
    Cd(String),
    File(String, usize),
}

fn terminal_output(s: &str) -> anyhow::Result<Vec<Line>> {
    let mut result = Vec::new();
    for line in s.lines() {
        if let Some(suffix) = line.strip_prefix("$ cd ") {
            result.push(Line::Cd(suffix.to_string()));
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            // Never used
        } else {
            let (size, name) = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("Could not parse file from line {line}"))?;
            if name.contains(' ') {
                bail!("Expected filename without spaces but got {name}");
            }
            let size = size.parse::<usize>()?;
            result.push(Line::File(name.to_string(), size));
        }
    }
    Ok(result)
}

fn directory_sizes(terminal_output: Vec<Line>) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    let mut path = Vec::new();
    for line in terminal_output.into_iter() {
        match line {
            Line::Cd(name) => match name.as_str() {
                "/" => {
                    path.clear();
                    path.push(name);
                }
                ".." => {
                    path.pop();
                }
                _ => {
                    path.push(name);
                }
            },
            Line::File(_, size) => {
                for i in 1..=path.len() {
                    *result.entry(path[..i].join("/")).or_default() += size;
                }
            }
        }
    }
    result
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let directory_sizes = directory_sizes(terminal_output(input)?);
    Ok(directory_sizes
        .values()
        .filter(|size| **size <= 100000)
        .sum())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let directory_sizes = directory_sizes(terminal_output(input)?);
    let total = directory_sizes
        .get("/")
        .ok_or_else(|| anyhow!("Expected terminal output to include file system root"))?;
    let needed = total - 40000000;
    Ok(*directory_sizes
        .values()
        .filter(|size| **size >= needed)
        .sorted()
        .next()
        .ok_or_else(|| anyhow!("Expected at least one directory to solve the problem"))?)
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
