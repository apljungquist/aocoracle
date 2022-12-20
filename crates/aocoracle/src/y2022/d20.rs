use anyhow::{anyhow, bail};

fn numbers(s: &str) -> anyhow::Result<Vec<i64>> {
    let re = regex::Regex::new(r"^(-?([1-9]\d*)|0)$").expect("Hard coded regex is valid");
    let mut result = Vec::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture number on line {}", line))?;
        result.push(cap[1].parse()?);
    }
    let num_zero = result.iter().filter(|n| **n == 0).count();
    if num_zero != 1 {
        bail!("Expected exactly 1 zero but got {num_zero}")
    }
    Ok(result)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    value: i64,
    prev: usize,
    next: usize,
}

struct LinkedDeque {
    nodes: Vec<Node>,
}

impl LinkedDeque {
    fn new(xs: &[i64]) -> Self {
        let mut nodes = Vec::with_capacity(xs.len());
        for (i, x) in xs.iter().enumerate() {
            nodes.push(Node {
                value: *x,
                prev: (i + xs.len() - 1) % xs.len(),
                next: (i + xs.len() + 1) % xs.len(),
            });
        }
        Self { nodes }
    }

    fn move_node(&mut self, id: usize) {
        // Remove
        let removed = self.nodes[id].clone();
        self.nodes[removed.prev].next = removed.next;
        self.nodes[removed.next].prev = removed.prev;

        // Move
        let rstep = removed.value.rem_euclid(self.nodes.len() as i64 - 1) as usize;
        let lstep = self.nodes.len() - 2 - rstep;
        let right = if rstep < lstep {
            let mut tmp = removed.next;
            for _ in 0..rstep {
                tmp = self.nodes[tmp].next;
            }
            tmp
        } else {
            let mut tmp = removed.prev;
            for _ in 0..lstep {
                tmp = self.nodes[tmp].prev;
            }
            tmp
        };

        // Insert
        let left = self.nodes[right].prev;
        self.nodes[left].next = id;
        self.nodes[right].prev = id;
        self.nodes[id].next = right;
        self.nodes[id].prev = left;
    }

    fn follow(&self, id: usize, offset: i64) -> usize {
        let mut right = id;
        for _ in 0..offset.rem_euclid(self.nodes.len() as i64) {
            right = self.nodes[right].next
        }
        right
    }

    fn mix(&mut self) {
        for id in 0..self.nodes.len() {
            self.move_node(id);
        }
    }

    fn origin(&self) -> usize {
        self.nodes.iter().position(|n| n.value == 0).unwrap()
    }

    #[cfg(debug_assertions)]
    fn print(&self) {
        let origin = self.origin();
        print!("{}", self.nodes[origin].value);
        let mut id = self.nodes[origin].next;
        while id != origin {
            print!(",{}", self.nodes[id].value);
            id = self.nodes[id].next;
        }
        println!();
    }
}

fn part_x(numbers: &[i64], num_round: usize, key: i64) -> anyhow::Result<i64> {
    let mut deque = LinkedDeque::new(&numbers.iter().map(|x| x * key).collect::<Vec<i64>>());
    #[cfg(debug_assertions)]
    deque.print();
    for _ in 0..num_round {
        deque.mix();
        #[cfg(debug_assertions)]
        deque.print();
    }
    let zero_id = deque.origin();
    let onek_id = deque.follow(zero_id, 1000);
    let twok_id = deque.follow(onek_id, 1000);
    let thrk_id = deque.follow(twok_id, 1000);
    let summands = vec![
        deque.nodes[onek_id].value,
        deque.nodes[twok_id].value,
        deque.nodes[thrk_id].value,
    ];
    dbg!(&summands);
    Ok(summands.iter().sum())
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    part_x(&numbers, 1, 1)
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let numbers = numbers(input)?;
    let answer = part_x(&numbers, 10, 811589153)?;
    dbg!(answer);
    // 6bb0c0bd67: answer > 190723450955
    // 3ba7923eae: answer != -6161584849576
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

    #[test]
    fn part_1_works_on_3ba7923eae() {
        assert_correct_answer_on_correct_input!(part_1, "3ba7923eae", Part::One);
    }

    #[test]
    fn part_2_works_on_3ba7923eae() {
        assert_correct_answer_on_correct_input!(part_2, "3ba7923eae", Part::Two);
    }
}
