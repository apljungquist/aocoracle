use std::fs;

use hashbrown::HashMap;

fn _starting_positions(input: &str) -> Vec<u64> {
    let re = regex::Regex::new(r"^Player (1|2) starting position: (\d+)$").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap()[2].parse::<u64>().unwrap() - 1)
        .collect()
}

fn _quantum_die() -> HashMap<u64, u64> {
    let mut result = HashMap::with_capacity(27);
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                *(result.entry(i + j + k).or_insert(0)) += 1;
            }
        }
    }
    result
}

fn _quantum(
    die: &HashMap<u64, u64>,
    positions: Vec<u64>,
    scores: Vec<u64>,
    hero: usize,
    num_universe: u64,
    result: &mut Vec<u64>,
) {
    for (num_step, multiplier) in die.iter() {
        let new_num_universe = multiplier * num_universe;
        let position = (positions[hero] + num_step) % 10;
        let score = scores[hero] + position + 1;
        if 21 <= score {
            result[hero] += new_num_universe;
            continue;
        }

        let mut new_positions = positions.clone();
        new_positions[hero] = position;
        let mut new_scores = scores.clone();
        new_scores[hero] = score;
        _quantum(
            die,
            new_positions,
            new_scores,
            (hero + 1) % 2,
            new_num_universe,
            result,
        );
    }
}

pub fn part_1(input: &str) -> u64 {
    let mut positions = _starting_positions(input);
    let mut scores = vec![0; 2];
    let mut i = 0;
    loop {
        for player in 0..=1 {
            let mut num_step = 0;
            for _ in 0..3 {
                num_step += (i % 100) + 1;
                i += 1;
            }
            positions[player] = (positions[player] + num_step) % 10;
            scores[player] += positions[player] + 1;
            if 1000 <= scores[player] {
                return i * scores[(player + 1) % 2];
            }
        }
    }
}

pub fn part_2(input: &str) -> u64 {
    let positions = _starting_positions(input);
    let scores = vec![0; 2];
    let mut result = vec![0; 2];
    _quantum(&_quantum_die(), positions, scores, 0, 1, &mut result);
    *result.iter().max().unwrap()
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/21/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 739785);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 916083);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 444356092776315);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 49982165861983);
    }
}
