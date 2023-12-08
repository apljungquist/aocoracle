use anyhow::bail;
use hashbrown::HashMap;
use itertools::Itertools;

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let mut lines = input.lines();
    let mut directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("{c}"),
        })
        .cycle();
    lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines {
        map.insert(
            &line.as_bytes()[..3],
            (&line.as_bytes()[7..10], &line.as_bytes()[12..15]),
        );
    }
    let target = "ZZZ".as_bytes();
    let mut curr = "AAA".as_bytes();

    let mut num_step = 0;
    while curr != target {
        num_step += 1;
        let (l, r) = map.get(curr).unwrap();
        curr = match directions.next().unwrap() {
            0 => l,
            1 => r,
            _ => panic!(""),
        };
    }
    Ok(num_step)
}

fn is_target2(nodes: &[&[u8]]) -> bool {
    for node in nodes {
        if node[2] != b'Z' {
            return false;
        }
    }
    return true;
}

fn dbg_stage(nodes: &[&[u8]]) {
    print!("{}", std::str::from_utf8(nodes[0]).unwrap());
    for n in &nodes[1..] {
        print!(", {}", std::str::from_utf8(n).unwrap());
    }
    println!("");
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let mut lines = input.lines();
    let mut directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("{c}"),
        })
        .cycle();
    lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines {
        map.insert(
            &line.as_bytes()[..3],
            [&line.as_bytes()[7..10], &line.as_bytes()[12..15]],
        );
    }
    let mut curr: Vec<_> = map
        .keys()
        .filter(|n| n[2] == b'A')
        .cloned()
        .sorted()
        .collect();
    dbg_stage(&curr);

    let mut num_step = 0;
    while !is_target2(&curr) {
        num_step += 1;
        let direction = directions.next().unwrap();
        curr = curr
            .into_iter()
            .map(|node| map.get(node).unwrap()[direction])
            .collect();
        // if num_step == 100 {
        //     break
        // }
        // dbg_stage(&curr);
    }
    Ok(num_step)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE_2", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "INPUT", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
