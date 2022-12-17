use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::ops::{Add, Index};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum Jet {
    L,
    R,
}

impl Jet {
    fn point(&self) -> Point {
        match self {
            Self::L => Point::new(-1, 0),
            Self::R => Point::new(1, 0),
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::L => "L",
            Self::R => "R",
        }
    }
}

fn parsed(s: &str) -> Vec<Jet> {
    let mut result = Vec::new();
    for direction in s.trim().chars() {
        result.push(match direction {
            '<' => Jet::L,
            '>' => Jet::R,
            other => panic!("Expected <> but got {other}"),
        })
    }
    result
}

fn move_h(state: &HashSet<Point>, jet: &Jet, prev: &Vec<Point>) -> Option<Vec<Point>> {
    let next: Vec<_> = prev.iter().map(|p| p + &jet.point()).collect();
    let max_x = next.iter().map(|p| p.x).max().unwrap();
    let min_x = next.iter().map(|p| p.x).min().unwrap();
    if min_x < 0 || 7 <= max_x {
        None
    } else {
        for p in next.iter() {
            if state.contains(p) {
                return None;
            }
        }
        Some(next)
    }
}

fn move_v(state: &HashSet<Point>, prev: &Vec<Point>) -> Option<Vec<Point>> {
    let d = Point::new(0, -1);
    let next: Vec<_> = prev.iter().map(|p| p + &d).collect();
    let min_y = next.iter().map(|p| p.y).min().unwrap();
    if min_y < 0 {
        None
    } else {
        for p in next.iter() {
            if state.contains(p) {
                return None;
            }
        }
        Some(next)
    }
}

fn print_state(
    state: &HashSet<Point>,
    rock: &Vec<Point>,
    label: &str,
    default: char,
    limit: Option<i32>,
) {
    let mut state: HashMap<Point, char> = state.iter().cloned().map(|p| (p, '#')).collect();
    for p in rock {
        state.insert(p.clone(), '@');
    }
    let x_min = state.keys().map(|p| p.x).min().unwrap_or(0).min(-1);
    let x_max = state.keys().map(|p| p.x).max().unwrap_or(0).max(7);
    let y_max = state.keys().map(|p| p.y).max().unwrap_or(0);
    let y_min = state.keys().map(|p| p.y).min().unwrap_or(0).min(-1);

    for x in x_min..=x_max {
        state.insert(Point::new(x, y_min), '-');
    }
    for y in y_min..=y_max {
        state.insert(Point::new(x_min, y), '|');
        state.insert(Point::new(x_max, y), '|');
    }
    state.insert(Point::new(x_min, y_min), '+');
    state.insert(Point::new(x_max, y_min), '+');

    let y_min_effective = match limit {
        Some(limit) => y_min.max(y_max - limit),
        None => y_min,
    };
    println!("{}", label);
    for y in (y_min_effective..=y_max).rev() {
        for x in x_min..=x_max {
            print!(
                "{}",
                char::from(*state.get(&Point::new(x, y)).unwrap_or(&default))
            );
        }
        println!();
    }
}

pub fn part_1(input: &str) -> anyhow::Result<i32> {
    let rock1 = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ];
    let rock2 = vec![
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ];
    let rock3 = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
    ];
    let rock4 = vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ];
    let rock5 = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
    ];

    let finite_jets = parsed(input);
    let mut jets = finite_jets.iter().cycle();
    let mut state: HashSet<Point> = HashSet::new();
    for rock in [rock1, rock2, rock3, rock4, rock5]
        .iter()
        .cycle()
        .take(2022)
    {
        let max_y = state.iter().map(|p| p.y).max().unwrap_or(-1);
        let mut prev = rock
            .iter()
            .map(|p| Point::new(p.x + 2, p.y + 4 + max_y))
            .collect();
        // print_state(&state, &prev, "Begins falling", '.');
        loop {
            let jet = jets.next().unwrap();
            if let Some(h) = move_h(&state, jet, &prev) {
                // println!("{jet}");
                // print_state(&state, &h,"Pushed horizontally", '.');
                if let Some(v) = move_v(&state, &h) {
                    // println!("hv");
                    prev = v;
                } else {
                    // println!("h.");
                    prev = h;
                    break;
                }
            } else {
                if let Some(v) = move_v(&state, &prev) {
                    // println!(".v");
                    prev = v;
                } else {
                    // println!("..");
                    break;
                }
            }
            // print_state(&state, &prev, jet.name(), '.');
        }
        for p in prev {
            state.insert(p);
        }
    }
    print_state(&state, &vec![], "Done", '.', None);
    Ok(state.iter().map(|p| p.y).max().unwrap() + 1)
}

fn fingerprint(state: &HashSet<Point>) -> Option<Vec<u8>> {
    let y_max = state.iter().map(|p| p.y).max().unwrap_or(0);
    let mut result = Vec::new();
    for y in (0..=y_max).rev() {
        let mut row = 0;
        for x in 0..=6 {
            if state.contains(&Point::new(x, y)) {
                row += 2 << x;
            }
        }
        if row == 0b1111111 {
            // for y2 in 0..y{
            //     for x in 0..=6{
            //         state.remove(&Point::new(x,y2));
            //     }
            // }
            return Some(result);
        }
        result.push(row);
    }
    None
}

struct CyclesDetector {
    fingerprints: Vec<Option<Vec<u8>>>,
}

impl CyclesDetector {
    fn update(&mut self, state: &HashSet<Point>) -> bool {
        let f_new = fingerprint(state);
        match f_new.as_ref().and_then(|f| Some(f.len())) {
            Some(h) => {
                println!("New fingerprint has height {h}");
            }
            None => {
                println!("New fingerprint has not height");
            }
        }
        let epoch_num = self.fingerprints.len() + 1;
        let result = if let Some(i) = self.fingerprints.iter().position(|f_old| *f_old == f_new) {
            println!("Cycle detected after {epoch_num} epochs matching epoch {i}");
            for (j, f_old) in self.fingerprints.iter().enumerate() {
                if let Some(f_old) = f_old {
                    println!("Epoch {j:>3} has height {0:>6}", f_old.len())
                } else {
                    println!("Epoch {j:>3} has no fingerprint")
                }
            }
            true
        } else {
            println!("No cycles detected after {epoch_num} epochs");
            false
        };
        self.fingerprints.push(f_new);
        result
    }
}

pub fn part_2(input: &str) -> anyhow::Result<i32> {
    let rock1 = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ];
    let rock2 = vec![
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ];
    let rock3 = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
    ];
    let rock4 = vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ];
    let rock5 = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
    ];

    let mut epoch_num = 0;
    let mut cycles_detector = CyclesDetector {
        fingerprints: vec![Some(vec![127])],
    };
    let finite_jets = parsed(input);
    let mut jets = finite_jets.iter().enumerate().cycle().peekable();
    let mut state: HashSet<Point> = HashSet::new();
    for (i, rock) in [rock1, rock2, rock3, rock4, rock5]
        .iter()
        .enumerate()
        .cycle()
        .take(1300 + 1620)
    {
        let j = jets.peek().unwrap().0;
        // println!("{i:>4}, {j:>4}");
        if i == 0 && j == 0 {
            println!("Epoch {epoch_num}");
            if cycles_detector.update(&state) {
                return Ok(0);
            }
            print_state(&state, &vec![], "Epoch end", '.', Some(100));
            if epoch_num > 1 {
                break;
            }
            epoch_num += 1;
        }
        let max_y = state.iter().map(|p| p.y).max().unwrap_or(-1);
        let mut prev = rock
            .iter()
            .map(|p| Point::new(p.x + 2, p.y + 4 + max_y))
            .collect();
        // print_state(&state, &prev, "Begins falling", '.');
        loop {
            let (_, jet) = jets.next().unwrap();
            if let Some(h) = move_h(&state, jet, &prev) {
                // println!("{jet}");
                // print_state(&state, &h,"Pushed horizontally", '.');
                if let Some(v) = move_v(&state, &h) {
                    // println!("hv");
                    prev = v;
                    // std::mem::replace(&mut prev, v);
                } else {
                    // println!("h.");
                    prev = h;
                    break;
                }
            } else {
                if let Some(v) = move_v(&state, &prev) {
                    // println!(".v");
                    prev = v;
                } else {
                    // println!("..");
                    break;
                }
            }
            // print_state(&state, &prev, jet.name(), '.');
        }
        for p in prev {
            state.insert(p);
        }
    }
    // print_state(&state, &vec![], "Done", '.', None);
    Ok(state.iter().map(|p| p.y).max().unwrap() + 1)
}

// 10091
// 5
// 50455

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
}

// tiles in rock cycle: 4+5+5+4+4=22

// example
//  offset
//   height 43
//   tiles 110
//   rocks 110/22*5 = 25
//  cycle
//   height 53
//   tiles 154
//   rocks 154/22*5 = 35
//
// (1000000000000-25)//35 = 28571428570
// (1000000000000-25)%35 = 25
// => > 28571428570*53+43
// => > 1514285714253
// tgt  1514285714288
// ok
// simulate 25+25 => height 78
// 28571428570*53+78 = 1514285714288
//                tgt  1514285714288
// ok

// input
//  offset
//   height 1998
//   tiles 5720
//   rocks 5720/22*5 = 1300
//  cycle
//   height 2681
//   tiles 7656
//   rocks 7656/22*5 = 1740
//
// (1000000000000-1300)//1740 = 574712642
// (1000000000000-1300)%1740 = 1620
// simulate 1300+1620 => height 4480
// 574712642*2681+4480 = 1540804597682
