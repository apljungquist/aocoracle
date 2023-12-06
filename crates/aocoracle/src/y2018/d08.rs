use anyhow::{anyhow, bail};

struct Node {
    child_nodes: Vec<Node>,
    metadata_entries: Vec<usize>,
}

impl Node {
    fn entries_sum(&self) -> usize {
        self.child_nodes
            .iter()
            .map(|child| child.entries_sum())
            .sum::<usize>()
            + self.metadata_entries.iter().sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.child_nodes.is_empty() {
            return self.entries_sum();
        }

        let mut result = 0;
        for i in self.metadata_entries.iter() {
            if *i == 0 {
                continue;
            }
            if let Some(node) = self.child_nodes.get(*i - 1) {
                result += node.value()
            }
        }
        result
    }
}

fn try_take_node(depth: usize, numbers: &mut Vec<&str>) -> anyhow::Result<Node> {
    if depth > 99 {
        bail!("Tree deeper than expected, bailing to prevent stack overflow")
    }
    let num_child: usize = numbers
        .pop()
        .ok_or_else(|| anyhow!("Expected quantity of child nodes"))?
        .parse()?;
    let num_entry: usize = numbers
        .pop()
        .ok_or_else(|| anyhow!("Expected quantity of metadata entries"))?
        .parse()?;

    let mut child_nodes = Vec::with_capacity(num_child);
    for _ in 0..num_child {
        child_nodes.push(try_take_node(depth + 1, numbers)?)
    }

    let mut metadata_entries = Vec::with_capacity(num_entry);
    for _ in 0..num_entry {
        metadata_entries.push(
            numbers
                .pop()
                .ok_or_else(|| anyhow!("Expected metadata entry"))?
                .parse()?,
        )
    }

    Ok(Node {
        child_nodes,
        metadata_entries,
    })
}

fn tree_from_str(s: &str) -> anyhow::Result<Node> {
    let mut lines = s.lines();
    let line = lines
        .next()
        .ok_or_else(|| anyhow!("Expected exactly one line but got 0"))?;
    if lines.next().is_some() {
        bail!("Expected exactly one line but got at least 2");
    }
    let mut numbers: Vec<_> = line.split(' ').rev().collect();
    try_take_node(0, &mut numbers)
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let tree = tree_from_str(input)?;
    Ok(tree.entries_sum())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let tree = tree_from_str(input)?;
    Ok(tree.value())
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("e2feb0983af51c38", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("e2feb0983af51c38", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
