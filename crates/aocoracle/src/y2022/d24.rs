use anyhow::{anyhow, bail};
use hashbrown::HashSet;
use pathfinding::prelude::astar;
use std::str::FromStr;

#[derive(Debug)]
struct Map {
    start: (usize, usize),
    goal: (usize, usize),
    up: HashSet<(usize, usize)>,
    down: HashSet<(usize, usize)>,
    left: HashSet<(usize, usize)>,
    right: HashSet<(usize, usize)>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let x_min = 1;
        let x_max = lines[0]
            .len()
            .checked_sub(2)
            .ok_or_else(|| anyhow!("Expected rectangular input with sides no shorter than 3"))?;
        let y_min = 1;
        let y_max = lines
            .len()
            .checked_sub(2)
            .ok_or_else(|| anyhow!("Expected rectangular input with sides no shorter than 3"))?;
        let x_start = lines[0]
            .chars()
            .position(|char| char == '.')
            .ok_or_else(|| anyhow!("Expected start tile on first line"))?;
        let x_goal = lines[lines.len() - 1]
            .chars()
            .position(|char| char == '.')
            .ok_or_else(|| anyhow!("Expected goal tile on last line"))?;

        if x_max < 1 || y_max < 1 {
            bail!("Expected rectangular input with sides no shorter than 3");
        }

        let mut up = HashSet::new();
        let mut down = HashSet::new();
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if y == y_min - 1 || y == y_max + 1 {
                    continue;
                }
                if x == x_min - 1 || x == x_max + 1 {
                    if char != '#' {
                        bail!("Expected wall on first and last column");
                    }
                    continue;
                }
                match char {
                    '^' => {
                        up.insert((x, y));
                    }
                    'v' => {
                        down.insert((x, y));
                    }
                    '<' => {
                        left.insert((x, y));
                    }
                    '>' => {
                        right.insert((x, y));
                    }
                    '.' => {}
                    _ => {
                        bail!("Expected one of '^', 'v', '<', '>', or '.' but got {char}");
                    }
                }
            }
        }

        Ok(Self {
            start: (x_start, y_min - 1),
            goal: (x_goal, y_max + 1),
            up,
            down,
            left,
            right,
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

impl Map {
    fn tile(&self, x: usize, y: usize, t: usize) -> Option<&'static str> {
        let up = wrapping_add(y, t, self.y_min, self.y_max);
        let down = wrapping_sub(y, t, self.y_min, self.y_max);
        let right = wrapping_sub(x, t, self.x_min, self.x_max);
        let left = wrapping_add(x, t, self.x_min, self.x_max);

        let mut blizzards = Vec::with_capacity(4);
        if self.up.contains(&(x, up)) {
            blizzards.push("^");
        }
        if self.down.contains(&(x, down)) {
            blizzards.push("v");
        }
        if self.left.contains(&(left, y)) {
            blizzards.push("<");
        }
        if self.right.contains(&(right, y)) {
            blizzards.push(">");
        }

        match blizzards.len() {
            0 => None,
            1 => Some(blizzards[0]),
            2 => Some("2"),
            3 => Some("3"),
            4 => Some("4"),
            _ => {
                panic!("Oops");
            }
        }
    }

    fn is_available(&self, x: usize, y: usize, t: usize) -> bool {
        if self.start == (x, y) || self.goal == (x, y) {
            return true;
        }

        if !(self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max) {
            return false;
        }
        self.tile(x, y, t).is_none()
    }
}

fn wrapping_add(lhs: usize, rhs: usize, min: usize, max: usize) -> usize {
    let i = lhs - min;
    let modulus = max - min + 1;
    let j = (i + rhs) % modulus;
    j + min
}

fn wrapping_sub(lhs: usize, rhs: usize, min: usize, max: usize) -> usize {
    let i = lhs - min;
    let modulus = max - min + 1;
    let j = ((i + modulus) - (rhs % modulus)) % modulus;
    j + min
}

fn manhattan(start: (usize, usize), goal: (usize, usize)) -> usize {
    start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    reached_goal: bool,
    reached_start: bool,
    elf: (usize, usize),
    t: usize,
}

impl State {
    fn updated_blizzards(&self) -> State {
        State {
            reached_goal: self.reached_goal,
            reached_start: self.reached_start,
            elf: self.elf,
            t: self.t + 1,
        }
    }
    fn moved_up(&self, map: &Map) -> Option<Self> {
        if self.elf.1 > 0 && map.is_available(self.elf.0, self.elf.1 - 1, self.t) {
            let mut result = self.clone();
            result.elf.1 -= 1;
            result.reached_goal = self.reached_goal || result.elf == map.goal;
            result.reached_start =
                self.reached_goal && (self.reached_start || result.elf == map.start);
            Some(result)
        } else {
            None
        }
    }
    fn moved_down(&self, map: &Map) -> Option<Self> {
        if map.is_available(self.elf.0, self.elf.1 + 1, self.t) {
            let mut result = self.clone();
            result.elf.1 += 1;
            result.reached_goal = self.reached_goal || result.elf == map.goal;
            result.reached_start =
                self.reached_goal && (self.reached_start || result.elf == map.start);
            Some(result)
        } else {
            None
        }
    }
    fn moved_left(&self, map: &Map) -> Option<Self> {
        if map.is_available(self.elf.0 - 1, self.elf.1, self.t) {
            let mut result = self.clone();
            result.elf.0 -= 1;
            Some(result)
        } else {
            None
        }
    }
    fn moved_right(&self, map: &Map) -> Option<Self> {
        if map.is_available(self.elf.0 + 1, self.elf.1, self.t) {
            let mut result = self.clone();
            result.elf.0 += 1;
            Some(result)
        } else {
            None
        }
    }
    fn wait(self, map: &Map) -> Option<Self> {
        if map.is_available(self.elf.0, self.elf.1, self.t) {
            Some(self)
        } else {
            None
        }
    }

    fn neighbors(&self, map: &Map) -> Vec<(State, usize)> {
        let cost = 1;
        let partial = self.updated_blizzards();
        let mut result = Vec::new();
        if let Some(state) = partial.moved_up(map) {
            result.push((state, cost));
        }
        if let Some(state) = partial.moved_down(map) {
            result.push((state, cost));
        }
        if let Some(state) = partial.moved_left(map) {
            result.push((state, cost));
        }
        if let Some(state) = partial.moved_right(map) {
            result.push((state, cost));
        }
        if let Some(state) = partial.wait(map) {
            result.push((state, cost));
        }
        result
    }

    fn start_state(map: &Map) -> Self {
        Self {
            reached_goal: false,
            reached_start: false,
            elf: map.start,
            t: 0,
        }
    }

    fn heuristic(&self, map: &Map) -> usize {
        manhattan(self.elf, map.goal)
    }
    fn success(&self, map: &Map) -> bool {
        self.elf == map.goal
    }

    fn heuristic2(&self, map: &Map) -> usize {
        match (self.reached_goal, self.reached_start) {
            (false, false) => manhattan(self.elf, map.goal) + manhattan(map.start, map.goal) * 2,
            (true, false) => manhattan(self.elf, map.start) + manhattan(map.start, map.goal),
            (true, true) => manhattan(self.elf, map.goal),
            _ => {
                panic!("Oops");
            }
        }
    }
    fn success2(&self, map: &Map) -> bool {
        self.reached_goal && self.reached_start && self.elf == map.goal
    }

    fn _print(&self, map: &Map, label: &str) {
        println!("{}:", label);
        for x in (map.x_min - 1)..=(map.x_max + 1) {
            if self.elf == (x, map.y_min - 1) {
                print!("E");
            } else if map.start.0 == x {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
        for y in map.y_min..=map.y_max {
            print!("#");
            for x in map.x_min..=map.x_max {
                if self.elf == (x, y) {
                    print!("E");
                    continue;
                }
                match map.tile(x, y, self.t) {
                    Some(s) => print!("{}", s),
                    None => print!("."),
                }
            }
            print!("#");
            println!();
        }
        for x in (map.x_min - 1)..=(map.x_max + 1) {
            if self.elf == (x, map.y_max + 1) {
                print!("E");
            } else if map.goal.0 == x {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
        println!();
    }
}

fn _print_example(map: &Map) -> Option<()> {
    let mut state = State::start_state(map);
    state._print(map, "Initial state");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 1, move down");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 2, move down");
    state = state.updated_blizzards().wait(map)?;
    state._print(map, "Minute 3, wait");
    state = state.updated_blizzards().moved_up(map)?;
    state._print(map, "Minute 4, move up");
    state = state.updated_blizzards().moved_right(map)?;
    state._print(map, "Minute 5, move right");
    state = state.updated_blizzards().moved_right(map)?;
    state._print(map, "Minute 6, move right");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 7, move down");
    state = state.updated_blizzards().moved_left(map)?;
    state._print(map, "Minute 8, move left");
    state = state.updated_blizzards().moved_up(map)?;
    state._print(map, "Minute 9, move up");
    state = state.updated_blizzards().moved_right(map)?;
    state._print(map, "Minute 10, move right");
    state = state.updated_blizzards().wait(map)?;
    state._print(map, "Minute 11, wait");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 12, move down");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 13, move down");
    state = state.updated_blizzards().moved_right(map)?;
    state._print(map, "Minute 14, move right");
    state = state.updated_blizzards().moved_right(map)?;
    state._print(map, "Minute 15, move right");
    state = state.updated_blizzards().moved_right(map)?;
    state._print(map, "Minute 16, move right");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 17, move down");
    state = state.updated_blizzards().moved_down(map)?;
    state._print(map, "Minute 18, move down");
    Some(())
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(input)?;
    let start = State::start_state(&map);

    let (_, cost) = astar(
        &start,
        |s| s.neighbors(&map),
        |s| s.heuristic(&map),
        |s| s.success(&map),
    )
    .unwrap();
    Ok(cost)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(input)?;
    let start = State::start_state(&map);
    let (_, cost) = astar(
        &start,
        |s| s.neighbors(&map),
        |s| s.heuristic2(&map),
        |s| s.success2(&map),
    )
    .unwrap();
    Ok(cost)
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!("965994098166be30", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("965994098166be30", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }

    #[test]
    fn wrapping_add_works() {
        assert_eq!(wrapping_add(6, 1, 1, 6), 1);
        assert_eq!(wrapping_add(1, 1, 1, 6), 2);
        assert_eq!(wrapping_add(6, 6, 1, 6), 6);
        assert_eq!(wrapping_add(1, 6, 1, 6), 1);
    }

    #[test]
    fn wrapping_sub_works() {
        assert_eq!(wrapping_sub(6, 1, 1, 6), 5);
        assert_eq!(wrapping_sub(1, 1, 1, 6), 6);
        assert_eq!(wrapping_sub(6, 6, 1, 6), 6);
        assert_eq!(wrapping_sub(1, 6, 1, 6), 1);
    }
}
