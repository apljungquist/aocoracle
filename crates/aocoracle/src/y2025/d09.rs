use hashbrown::HashSet;
use std::collections::HashMap;
use std::iter::once;
use std::ops::AddAssign;

pub fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut tiles = HashSet::new();
    for line in input.lines() {
        let (i, j) = line.split_once(",").unwrap();
        let i = i.parse::<i64>()?;
        let j = j.parse::<i64>()?;
        tiles.insert((i, j));
    }
    let mut max = 0;
    for &(i1, j1) in &tiles {
        for &(i2, j2) in &tiles {
            let candidate = (i1.abs_diff(i2) + 1) * (j1.abs_diff(j2) + 1);
            max = max.max(candidate);
        }
    }
    Ok(max)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut tiles = HashSet::new();
    for line in input.lines() {
        let (i, j) = line.split_once(",").unwrap();
        let i = i.parse::<i64>()?;
        let j = j.parse::<i64>()?;
        tiles.insert((i, j));
    }
    let mut max = 0;
    for &(i1, j1) in &tiles {
        'outer: for &(i2, j2) in &tiles {
            // 2,5 -> 11,1: 50
            let imin = i1.min(i2); // 2
            let imax = i1.max(i2); // 11
            let jmin = j1.min(j2); // 1
            let jmax = j1.max(j2); // 5
            let candidate = (i1.abs_diff(i2) + 1) * (j1.abs_diff(j2) + 1);
            for &(i3, j3) in &tiles {
                // // Corner check
                // // 2,1 --- 11,1
                // //  |        |
                // // 2,5 --- 11,5
                // if (i3 == i1 && j3 == j2) || (i3 == i2 && j3 == j1) || (i3 == i1 && j3 == j2) || (i3 == i2 && j3 == j2) {
                //     continue;
                // }

                // 7,3
                // 2 < 7 < 11: true
                // 1 < 3 < 5: true
                if imin < i3 && i3 < imax && jmin <= j3 && j3 <= jmax {
                    // if candidate == 24 {
                    //     println!("breaking {i1},{j1}->{i2},{j2} at {i3},{j3}");
                    // }
                    continue 'outer;
                }
                if imin <= i3 && i3 <= imax && jmin < j3 && j3 < jmax {
                    // if candidate == 24 {
                    //     println!("breaking {i1},{j1}->{i2},{j2} at {i3},{j3}");
                    // }
                    continue 'outer;
                }
            }
            // println!("{i1},{j1}->{i2},{j2}: {candidate}");
            max = max.max(candidate);
        }
    }

    assert!(max < 4511989482);
    Ok(max)
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
