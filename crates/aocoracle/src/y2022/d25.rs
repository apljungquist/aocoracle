// 2
// 1
// 0
// -1 (-)
// -2 (=)

use pathfinding::num_traits::Pow;
use pathfinding::prelude::{astar, dijkstra};

fn from_snafu(s: &str) -> i64 {
    let mut result = 0;
    for (i, ch) in s.chars().rev().enumerate() {
        result += 5_i64.pow(i as u32)
            * match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Oops"),
        };
    }
    result
}

fn to_snafu(mut x: i64) -> String {
    let mut exp_max = 0;
    while x / (5_i64.pow(exp_max)) != 0 {
        exp_max += 1;
    }

    let mut result = String::new();
    for i in (0..exp_max).rev() {
        let d = 5_i64.pow(i);
        let max:i64 = (0..i).map(|j| 2 * 5_i64.pow(j)).sum();
        let c = if -max <= x && x <= max {
            '0'
        } else if 0<x {
            assert!(max < x);
            match (x - max)/d{
                0=>{x -= d;'1'},
                1=>{x -= 2*d;'2'},
                _=>panic!("Oops")
            }
        } else {
            assert!(x < -max);
            match (x + max)/d{
                0=>{x += d;'-'},
                -1=>{x += 2*d;'='},
                _=>panic!("Oops")
            }
        };
        result.push(c);
    }
    result
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;
    for line in input.lines() {
        let x = from_snafu(line);
        sum += x;
    }
    Ok(to_snafu(sum))
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
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
