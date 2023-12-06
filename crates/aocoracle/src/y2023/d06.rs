use std::str::Lines;

use anyhow::bail;

fn take_suffix<'a>(lines: &'a mut Lines, expected_prefix: &'static str) -> anyhow::Result<&'a str> {
    let Some(line) = lines.next() else {
        bail!("Expected at least one more line")
    };
    let Some((actual_prefix, suffix)) = line.split_once(':') else {
        bail!("Expected line containing ':' but got {line:?}");
    };
    if actual_prefix != expected_prefix {
        bail!("Expected line starting with {expected_prefix:?} but got {line:?}")
    }
    Ok(suffix)
}
fn races(input: &str) -> anyhow::Result<Vec<(i64, i64)>> {
    let mut lines = input.lines();
    let mut times = Vec::new();
    for time in take_suffix(&mut lines, "Time")?.split_whitespace() {
        times.push(time.parse()?);
    }
    let mut distances = Vec::new();
    for distance in take_suffix(&mut lines, "Distance")?.split_whitespace() {
        distances.push(distance.parse()?)
    }
    Ok(times.into_iter().zip(distances).collect())
}
fn race(input: &str) -> anyhow::Result<(i64, i64)> {
    let mut lines = input.lines();
    let time = take_suffix(&mut lines, "Time")?
        .split_whitespace()
        .collect::<String>()
        .parse()?;
    let distance = take_suffix(&mut lines, "Distance")?
        .split_whitespace()
        .collect::<String>()
        .parse()?;
    Ok((time, distance))
}

fn num_victory(time: i64, distance: i64) -> i64 {
    let b = time as f64;
    let a = -1.0;
    let c = -(distance + 1) as f64;
    let first = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let last = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    last.floor() as i64 - first.ceil() as i64 + 1
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let races = races(input)?;
    Ok(races
        .into_iter()
        .map(|(t, d)| num_victory(t, d))
        .product::<i64>())
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let (time, distance) = race(input)?;
    Ok(num_victory(time, distance))
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
        assert_correct_answer_on_correct_input!(part_1, "f422fdccdd57e74e", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "f422fdccdd57e74e", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
