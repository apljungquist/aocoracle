fn sheet(input: &str) -> Vec<(i64, i64)> {
    let mut lines = input.lines();
    let (time_title, times) = lines.next().unwrap().split_once(':').unwrap();
    let (distance_title, distances) = lines.next().unwrap().split_once(':').unwrap();
    times
        .split_whitespace()
        .zip(distances.split_whitespace())
        .map(|(t, d)| (t.parse().unwrap(), d.parse().unwrap()))
        .collect()
}
fn race(input: &str) -> anyhow::Result<(i64, i64)> {
    let mut lines = input.lines();
    let (time_title, times) = lines.next().unwrap().split_once(':').unwrap();
    let (distance_title, distances) = lines.next().unwrap().split_once(':').unwrap();
    // concatenate the strings in times
    let time = times.split_whitespace().collect::<String>().parse()?;
    let distance = distances.split_whitespace().collect::<String>().parse()?;
    Ok((time, distance))
}

fn num_victory(time: i64, distance: i64) -> i64 {
    let b = time as f64;
    let a = -1.0;
    let c = -(distance + 1) as f64;
    let x1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let x2 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    dbg!(x1.max(x2)).floor() as i64 - dbg!(x1.min(x2)).ceil() as i64 + 1
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let races = sheet(input);
    Ok(races
        .into_iter()
        .map(|(t, d)| dbg!(num_victory(t, d)))
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
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
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
