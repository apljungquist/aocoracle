use hashbrown::HashSet;
use std::collections::HashMap;
use std::iter::once;
use std::ops::AddAssign;

pub fn part_1(input: &str) -> anyhow::Result<u32> {
    let mut start = None;
    let mut splitters = Vec::new();
    for (i, line) in input.lines().enumerate() {
        splitters.resize(i + 1, HashSet::new());
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    assert_eq!(i, 0);
                    start = Some(j)
                }
                '^' => {
                    splitters[i].insert(j);
                }
                '.' => {}
                _ => panic!("Invalid character: {}", c),
            }
        }
    }

    assert_eq!(splitters[0].len(), 0);

    let start: HashSet<_> = start.into_iter().collect();
    let mut beams = vec![start];
    let mut sum = 0;
    for i in 1..splitters.len() {
        beams.resize(i + 1, HashSet::new());
        for beam in beams[i - 1].iter().cloned().collect::<Vec<_>>() {
            if splitters[i].contains(&beam) {
                sum += 1;
                beams[i].insert(beam + 1);
                beams[i].insert(beam - 1);
            } else {
                beams[i].insert(beam);
            }
        }
    }

    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<u128> {
    let mut start = None;
    let mut splitters = Vec::new();
    for (i, line) in input.lines().enumerate() {
        splitters.resize(i + 1, HashSet::new());
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    assert_eq!(i, 0);
                    start = Some(j as u128)
                }
                '^' => {
                    splitters[i].insert(j as u128);
                }
                '.' => {}
                _ => panic!("Invalid character: {}", c),
            }
        }
    }

    assert_eq!(splitters[0].len(), 0);

    let start: HashMap<u128, u128> = [(start.unwrap(), 1)].into_iter().collect();
    let mut beams = vec![start];
    for i in 1..splitters.len() {
        beams.resize(i + 1, HashMap::new());
        for (beam, n) in beams[i - 1].clone() {
            if splitters[i].contains(&beam) {
                beams[i].entry(beam + 1).or_default().add_assign(n);
                beams[i].entry(beam - 1).or_default().add_assign(n);
            } else {
                beams[i].entry(beam).or_default().add_assign(n);
            }
        }
    }

    // for (j, beams) in beams.iter().enumerate() {
    //     if splitters[j].len() != 0 {
    //         for i in 0..=*beams.keys().max().unwrap() {
    //             match splitters[j].contains(&i) {
    //                 true => print!("^"),
    //                 false => print!(" "),
    //             }
    //         }
    //         println!("");
    //     }
    //     for i in 0..=*beams.keys().max().unwrap() {
    //         match beams.get(&i) {
    //             Some(n)=>print!("{n:x}"),
    //             None => print!(" "),
    //         }
    //     }
    //     println!("");
    //
    // }

    let sum = beams.into_iter().last().unwrap().values().sum();

    Ok(sum)
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
