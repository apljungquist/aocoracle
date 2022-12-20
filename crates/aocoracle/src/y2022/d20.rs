use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::LinkedList;
use std::ptr::NonNull;

fn numbers(s: &str) -> anyhow::Result<Vec<i32>> {
    Ok(s.lines().map(|l| l.parse().unwrap()).collect())
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Node {
    prev: i32,
    next: i32,
    curr: i32,
}

struct LinkedDeque {
    first: i32,
    nodes: HashMap<i32, Node>,
}

impl LinkedDeque {
    fn new(data: Vec<i32>) -> Self {
        let first = data[0];

        let mut nodes = HashMap::new();
        nodes.insert(
            data[0],
            Node {
                prev: data[data.len() - 1],
                curr: data[0],
                next: data[1],
            },
        );
        nodes.insert(
            data[data.len() - 1],
            Node {
                prev: data[data.len() - 2],
                curr: data[data.len() - 1],
                next: data[0],
            },
        );
        for (&prev, &curr, &next) in data.iter().tuple_windows() {
            nodes.insert(curr, Node { prev, curr, next });
        }
        Self { first, nodes }
    }

    fn move_r(&mut self, x: &i32, i: i32) {
        // Remove x
        // Cannot borrow as mutable more than once at a time...
        // let mut curr = self.nodes.get_mut(x).unwrap();
        // let mut prev = self.nodes.get_mut(&curr.prev).unwrap();
        // let mut next = self.nodes.get_mut(&curr.next).unwrap();
        // prev.next = next.curr;
        // next.prev = prev.curr;

        //Insert x
    }
}

fn print_numbers(numbers: &[i32]) {
    for x in numbers {
        print!("{}, ", x);
    }
    println!()
}

fn index(i: i32, len: usize) -> usize {
    let len: i32 = len as i32;
    (((i % len) + len) % len) as usize
}

fn move_number(numbers: &mut Vec<i32>, number: i32) {
    let old = numbers.iter().position(|n| *n == number).unwrap();
    numbers.remove(old);
    let mut new = index(old as i32 + number, numbers.len());
    if new == 0 && number < 0 {
        new = numbers.len();
    }
    numbers.insert(new, number);
    // println!(
    //     "{} moves between {} and {}",
    //     number,
    //     numbers[index(new as i32 - 1, numbers.len())],
    //     numbers[index(new as i32 + 1, numbers.len())]
    // );
}

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    let numbers = numbers(input)?;
    let mut moved = numbers.clone();
    // print_numbers(&moved);
    for x in numbers {
        move_number(&mut moved, x);
        // print_numbers(&moved);
    }
    let i = moved.iter().position(|n| *n == 0).unwrap();
    let summands = vec![
        moved[index(i as i32 + 1000, moved.len())],
        moved[index(i as i32 + 2000, moved.len())],
        moved[index(i as i32 + 3000, moved.len())],
    ];
    dbg!(&summands);
    let answer = summands.iter().sum();
    dbg!(answer);
    assert!(answer == 3 || answer < 10797);
    assert!(answer == 3 || answer < 3533);
    Ok(answer)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
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
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "example", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "6bb0c0bd67", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }

    #[test]
    fn index_works() {
        let xs: Vec<_> = (0..10).collect();
        assert_eq!(xs[index(0, xs.len())], 0);
        assert_eq!(xs[index(9, xs.len())], 9);
        assert_eq!(xs[index(-1, xs.len())], 9);
        assert_eq!(xs[index(-11, xs.len())], 9);
        assert_eq!(xs[index(10, xs.len())], 0);
    }

    #[test]
    fn move_number_works_simple() {
        let mut xs: Vec<_> = (5..12).collect();
        move_number(&mut xs, 5);
        assert_eq!(xs, vec![6, 7, 8, 9, 10, 5, 11])
    }

    #[test]
    fn move_number_works_more_than_len() {
        let mut xs: Vec<_> = (5..12).collect();
        move_number(&mut xs, 10);
        assert_eq!(xs, vec![5, 6, 7, 10, 8, 9, 11])
    }

    #[test]
    fn move_number_works_more_less_than_minus_len() {
        let mut xs: Vec<_> = vec![1, -10, 3, 4, 5, 6, 7];
        move_number(&mut xs, -10);
        assert_eq!(xs, vec![1, 3, 4, -10, 5, 6, 7])
    }

    #[test]
    fn move_number_works_exactly_end() {
        let mut xs = vec![6, 5, 7, 8, 9, 10, 11];
        move_number(&mut xs, 6);
        assert_eq!(xs, vec![6, 5, 7, 8, 9, 10, 11])
    }

    #[test]
    fn move_number_works_exactly_start() {
        let mut xs = vec![6, -1, 7, 8, 9, 10, 11];
        move_number(&mut xs, -1);
        assert_eq!(xs, vec![6, 7, 8, 9, 10, 11, -1])
    }
}
