use anyhow::{anyhow, bail};
use hashbrown::HashMap;

struct Input<'a> {
    directions: Vec<usize>,
    map: HashMap<&'a [u8], [&'a [u8]; 2]>,
}

impl<'a> Input<'a> {
    fn try_new(input: &'a str) -> anyhow::Result<Self> {
        let mut lines = input.lines();
        let mut directions = Vec::new();
        for d in lines
            .next()
            .ok_or_else(|| anyhow!("Expected a line with directions"))?
            .chars()
        {
            directions.push(match d {
                'L' => 0,
                'R' => 1,
                _ => bail!("Expected L or R but got {d}"),
            });
        }
        lines
            .next()
            .ok_or_else(|| anyhow!("Expected a blank line"))?;

        let mut map = HashMap::new();
        for line in lines {
            map.insert(
                &line.as_bytes()[..3],
                [&line.as_bytes()[7..10], &line.as_bytes()[12..15]],
            );
        }
        Ok(Self { directions, map })
    }

    fn num_step_until<F>(&self, start: &[u8], is_target: F) -> usize
    where
        F: Fn(&[u8]) -> bool,
    {
        let mut curr = start;
        for (i, d) in self.directions.iter().cycle().enumerate() {
            curr = self.map.get(curr).unwrap()[*d];
            if is_target(curr) {
                return i + 1;
            }
        }
        unreachable!("Cycle should never end")
    }
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let input = Input::try_new(input)?;
    Ok(input.num_step_until(b"AAA", |node| node == b"ZZZ"))
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let input = Input::try_new(input)?;
    input
        .map
        .keys()
        .filter(|n| n[2] == b'A')
        .cloned()
        .map(|start| input.num_step_until(start, |node| node[2] == b'Z'))
        .reduce(num::integer::lcm)
        .ok_or_else(|| anyhow!("Overflow"))
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example_a() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE_1a", Part::One);
    }

    #[test]
    fn part_1_works_on_example_b() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE_1b", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "d811eb4b1190750e", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE_2", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "d811eb4b1190750e", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
