use crate::AnyError;
use hashbrown::HashMap;

fn _starting_positions(input: &str) -> Result<[u64; 2], AnyError> {
    let re = regex::Regex::new(r"^Player (1|2) starting position: (\d+)$").unwrap();
    let mut result = [0; 2];
    let mut positions: Vec<u64> = Vec::new();
    for line in input.lines() {
        let cap = re.captures(line).ok_or("Regex does not match line")?;
        positions.push(cap[2].parse::<u64>().unwrap() - 1);
    }
    result[0] = positions[0];
    result[1] = positions[1];
    Ok(result)
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
    cache: &mut HashMap<(u64, u64, u64, u64), [u64; 2]>,
    die: &HashMap<u64, u64>,
    lpos: u64,
    rpos: u64,
    lscore: u64,
    rscore: u64,
) -> [u64; 2] {
    if 21 <= rscore {
        assert!(lscore < 21);
        return [0, 1];
    }
    let mut result = [0; 2];
    for (num_step, multiplier) in die.iter() {
        let pos = (lpos + num_step) % 10;
        let score = lscore + pos + 1;
        let key = (rpos, pos, rscore, score);
        let sub = {
            if !cache.contains_key(&key) {
                let value = _quantum(cache, die, rpos, pos, rscore, score);
                cache.insert(key, value);
                value
            } else {
                *cache.get(&key).unwrap()
            }
        };
        result[0] += sub[1] * multiplier;
        result[1] += sub[0] * multiplier;
    }
    result
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let mut positions = _starting_positions(input)?;
    let mut scores = [0; 2];
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
                return Ok((i * scores[(player + 1) % 2]).to_string());
            }
        }
    }
}

pub fn part_2(input: &str) -> Result<String, AnyError> {
    let positions = _starting_positions(input)?;
    let mut cache = HashMap::new();
    Ok(_quantum(
        &mut cache,
        &_quantum_die(),
        positions[0],
        positions[1],
        0,
        0,
    )
    .iter()
    .max()
    .unwrap()
    .to_string())
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
        assert_correct_answer_on_correct_input!(part_1, "da2f29b03f7891fc", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "da2f29b03f7891fc", Part::Two);
    }

    #[ignore]
    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(&part_1, &part_2);
    }
}
