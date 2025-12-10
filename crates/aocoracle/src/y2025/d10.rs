use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::once;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    diagram: u16,
    buttons: Vec<u16>,
    buttons2: Vec<u128>,
    joltage: u128,
    // buttons2: Vec<[u8;10]>,
    // joltage: [u8;10],
}

impl Machine {
    fn print(&self) {
        print!("[{:010b}]", self.diagram);
        for button in self.buttons.iter() {
            print!(" ({:010b})", button);
        }
        println!();
    }

    fn turn_on(&self) -> Option<usize> {
        for i in 1..=self.buttons.len() {
            for buttons in self.buttons.iter().combinations_with_replacement(i) {
                if buttons.into_iter().fold(0, |acc, x| acc ^ x) == self.diagram {
                    return Some(i);
                }
            }
        }
        None
    }

    fn configure(&self) -> Option<usize> {
        todo!("This just needs a different approach");
        let repeated = self.buttons2.repeat(10).into_iter().collect::<Vec<_>>();
        for i in 1..=self.buttons.len() {
            for buttons in repeated.iter().combinations_with_replacement(i) {
                if buttons.into_iter().fold(0, |acc, x| acc + x) == self.joltage {
                    return Some(i);
                }
            }
        }
        None
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let diagram = parts[0]
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '#' => 2u16.pow(i as u32),
                '.' => 0,
                _ => todo!(),
            })
            .sum();

        let mut buttons = Vec::new();
        for part in parts[1..parts.len() - 1].iter() {
            let button = part
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .map(|s| 2u16.pow(u32::from_str(s).unwrap()))
                // .inspect(|x| println!("{x:?} {x:010b}"))
                .sum();
            buttons.push(button);
        }

        let mut buttons2 = Vec::new();
        for part in parts[1..parts.len() - 1].iter() {
            let button = part
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .map(|s| 256u128.pow(u32::from_str(s).unwrap()))
                // .inspect(|x| println!("{x:?} {x:010b}"))
                .sum();
            buttons2.push(button);
        }
        //
        // let mut buttons2 = Vec::new();
        // for part in parts[1..parts.len()-1].iter() {
        //     let mut button = [0u8;10];
        //     for i in part
        //         .strip_prefix('(')
        //         .unwrap()
        //         .strip_suffix(')')
        //         .unwrap()
        //         .split(',') {
        //         button[i.parse::<usize>().unwrap()] = 1;
        //     }
        //     buttons2.push(button);
        // }

        let joltage = parts[parts.len() - 1]
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .enumerate()
            .map(|(i, s)| 256u128.pow(i as u32) * s.parse::<u128>().unwrap())
            .sum();

        Ok(Self {
            diagram,
            buttons,
            buttons2,
            joltage,
        })
    }
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut machines = Vec::new();
    for line in input.lines() {
        machines.push(Machine::from_str(line).unwrap())
    }
    let mut sum = 0;
    for machine in machines {
        sum += machine.turn_on().unwrap();
    }
    Ok(sum)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut machines = Vec::new();
    for line in input.lines() {
        machines.push(Machine::from_str(line).unwrap())
    }
    let mut sum = 0;
    for machine in machines {
        sum += machine.configure().unwrap();
    }
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
