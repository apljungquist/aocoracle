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

fn is_target2(node: &[u8]) -> bool {
    node[2] == b'Z'
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
    let mut directions: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("{c}"),
        })
        .collect();
    lines.next().unwrap();

    let mut map = HashMap::new();
    for line in lines {
        map.insert(
            &line.as_bytes()[..3],
            [&line.as_bytes()[7..10], &line.as_bytes()[12..15]],
        );
    }
    let mut starts: Vec<_> = map
        .keys()
        .filter(|n| n[2] == b'A')
        .cloned()
        .sorted()
        .collect();

    let mut multiples = Vec::with_capacity(starts.len());
    for start in starts {
        // let mut hits = Vec::new();

        let mut curr = start;
        let mut num_step = 0;
        for (i, d) in directions.iter().enumerate().cycle() {
            num_step += 1;
            curr = map.get(curr).unwrap()[*d];
            if is_target2(curr) {
                multiples.push(num_step);
                break;
                // println!(
                //     "{} {} {}: {} {}",
                //     std::str::from_utf8(start).unwrap(),
                //     std::str::from_utf8(curr).unwrap(),
                //     i,
                //     num_step,
                //     num_step - hits.last().unwrap_or(&0)
                // );
                // if hits.len() == 3 {
                //     break;
                // } else {
                //     hits.push(num_step);
                // }
            }
        }
    }
    Ok(multiples
        .into_iter()
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap())
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
