use anyhow::bail;
use hashbrown::HashSet;
use pathfinding::prelude::{astar, dijkstra};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Grid {
    data: Vec<bool>,
    x_min: usize,
    y_min: usize,
    width: usize,
}

impl Grid {
    fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;
        Self {
            data: vec![false; width * height],
            x_min,
            y_min,
            width,
        }
    }
    fn insert(&mut self, location: (usize, usize)) {
        let row = (location.1 - self.y_min);
        let col = (location.0 - self.x_min);
        self.data[row * self.width + col] = true;
    }
    fn contains(&self, location: &(usize, usize)) -> bool {
        let row = (location.1 - self.y_min);
        let col = (location.0 - self.x_min);
        self.data[row * self.width + col]
    }
    fn empty_like(&self) -> Self {
        Self {
            data: vec![false; self.data.len()],
            x_min: self.x_min,
            y_min: self.y_min,
            width: self.width,
        }
    }
    fn iter(&self) -> GridIterator {
        GridIterator { grid: self, i: 0 }
    }
}

struct GridIterator<'a> {
    grid: &'a Grid,
    i: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.grid.data.get(self.i).unwrap_or(&true) {
            self.i += 1;
        }
        if self.i == self.grid.data.len() {
            return None;
        }

        let row = self.i / self.grid.width;
        let col = self.i % self.grid.width;
        let x = col + self.grid.x_min;
        let y = row + self.grid.y_min;
        self.i += 1;
        Some((x, y))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    start: (usize, usize),
    elf: (usize, usize),
    goal: (usize, usize),
    up: Grid,
    down: Grid,
    left: Grid,
    right: Grid,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let x_min = 1;
        let x_max = lines[0].len() - 2;
        let y_min = 1;
        let y_max = lines.len() - 2;
        let x_start = lines[0].chars().position(|char| char == '.').unwrap();
        let x_goal = lines[lines.len() - 1]
            .chars()
            .position(|char| char == '.')
            .unwrap();

        let mut up = Grid::new(x_min, x_max, y_min, y_max);
        let mut down = Grid::new(x_min, x_max, y_min, y_max);
        let mut left = Grid::new(x_min, x_max, y_min, y_max);
        let mut right = Grid::new(x_min, x_max, y_min, y_max);
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
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
                    '#' => {}
                    _ => {
                        bail!("Expected ^v<> but got {char}");
                    }
                }
            }
        }

        Ok(Self {
            elf: (x_start, y_min - 1),
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

impl State {
    fn is_available(&self, x: usize, y: usize) -> bool {
        (self.start.0 == x && self.start.1 == y)
            || (self.goal.0 == x && self.goal.1 == y)
            || (self.x_min <= x
                && x <= self.x_max
                && self.y_min <= y
                && y <= self.y_max
                && !self.up.contains(&(x, y))
                && !self.down.contains(&(x, y))
                && !self.left.contains(&(x, y))
                && !self.right.contains(&(x, y)))
    }

    fn updated_blizzards(&self) -> Self {
        let mut up = self.up.empty_like();
        let mut down = self.down.empty_like();
        let mut left = self.left.empty_like();
        let mut right = self.right.empty_like();
        for (x, y) in self.up.iter() {
            if y == self.y_min {
                up.insert((x, self.y_max));
            } else {
                up.insert((x, y - 1));
            }
        }
        for (x, y) in self.down.iter() {
            if y == self.y_max {
                down.insert((x, self.y_min));
            } else {
                down.insert((x, y + 1));
            }
        }
        for (x, y) in self.left.iter() {
            if x == self.x_min {
                left.insert((self.x_max, y));
            } else {
                left.insert((x - 1, y));
            }
        }
        for (x, y) in self.right.iter() {
            if x == self.x_max {
                right.insert((self.x_min, y));
            } else {
                right.insert((x + 1, y));
            }
        }
        Self {
            elf: self.elf,
            start: self.start,
            goal: self.goal,
            up,
            down,
            left,
            right,
            x_min: self.x_min,
            x_max: self.x_max,
            y_min: self.y_min,
            y_max: self.y_max,
        }
    }

    fn moved_up(&self) -> Option<Self> {
        if self.elf.1 > 0 && self.is_available(self.elf.0, self.elf.1 - 1) {
            let mut result = self.clone();
            result.elf.1 -= 1;
            Some(result)
        } else {
            None
        }
    }
    fn moved_down(&self) -> Option<Self> {
        if self.is_available(self.elf.0, self.elf.1 + 1) {
            let mut result = self.clone();
            result.elf.1 += 1;
            Some(result)
        } else {
            None
        }
    }
    fn moved_left(&self) -> Option<Self> {
        if self.is_available(self.elf.0 - 1, self.elf.1) {
            let mut result = self.clone();
            result.elf.0 -= 1;
            Some(result)
        } else {
            None
        }
    }
    fn moved_right(&self) -> Option<Self> {
        if self.is_available(self.elf.0 + 1, self.elf.1) {
            let mut result = self.clone();
            result.elf.0 += 1;
            Some(result)
        } else {
            None
        }
    }
    fn wait(self) -> Option<Self> {
        if self.is_available(self.elf.0, self.elf.1) {
            Some(self)
        } else {
            None
        }
    }

    fn neighbors(&self) -> Vec<(Self, usize)> {
        let COST = 1;
        let mut partial = self.updated_blizzards();
        let mut result = Vec::new();
        if let Some(state) = partial.moved_up() {
            result.push((state, COST));
        }
        if let Some(state) = partial.moved_down() {
            result.push((state, COST));
        }
        if let Some(state) = partial.moved_left() {
            result.push((state, COST));
        }
        if let Some(state) = partial.moved_right() {
            result.push((state, COST));
        }
        if let Some(state) = partial.wait() {
            result.push((state, COST));
        }
        result
    }

    fn heuristic(&self) -> usize {
        self.elf.0.abs_diff(self.goal.0) + self.elf.1.abs_diff(self.goal.1)
    }
    fn success(&self) -> bool {
        self.elf == self.goal
    }

    fn print(&self, label: &str) {
        println!("{}:", label);
        for x in (self.x_min - 1)..=(self.x_max + 1) {
            if self.elf == (x, self.y_min - 1) {
                print!("E");
            } else if self.start.0 == x {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
        for y in self.y_min..=self.y_max {
            print!("#");
            for x in self.x_min..=self.x_max {
                if self.elf == (x, y) {
                    print!("E");
                    continue;
                }

                let mut tile = Vec::new();
                if self.up.contains(&(x, y)) {
                    tile.push('^');
                }
                if self.down.contains(&(x, y)) {
                    tile.push('v');
                }
                if self.left.contains(&(x, y)) {
                    tile.push('<');
                }
                if self.right.contains(&(x, y)) {
                    tile.push('>');
                }
                match tile.len() {
                    0 => {
                        print!(".");
                    }
                    1 => {
                        print!("{}", tile[0]);
                    }
                    _ => {
                        print!("{}", tile.len());
                    }
                }
            }
            print!("#");
            println!();
        }
        for x in (self.x_min - 1)..=(self.x_max + 1) {
            if self.elf == (x, self.y_max + 1) {
                print!("E");
            } else if self.goal.0 == x {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
        println!();
    }
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let start = State::from_str(input)?;

    // let mut state = start.clone();
    // state.print("Initial state");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 1, move down");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 2, move down");
    // state = state.updated_blizzards().wait().unwrap();
    // state.print("Minute 3, wait");
    // state = state.updated_blizzards().moved_up().unwrap();
    // state.print("Minute 4, move up");
    // state = state.updated_blizzards().moved_right().unwrap();
    // state.print("Minute 5, move right");
    // state = state.updated_blizzards().moved_right().unwrap();
    // state.print("Minute 6, move right");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 7, move down");
    // state = state.updated_blizzards().moved_left().unwrap();
    // state.print("Minute 8, move left");
    // state = state.updated_blizzards().moved_up().unwrap();
    // state.print("Minute 9, move up");
    // state = state.updated_blizzards().moved_right().unwrap();
    // state.print("Minute 10, move right");
    // state = state.updated_blizzards().wait().unwrap();
    // state.print("Minute 11, wait");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 12, move down");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 13, move down");
    // state = state.updated_blizzards().moved_right().unwrap();
    // state.print("Minute 14, move right");
    // state = state.updated_blizzards().moved_right().unwrap();
    // state.print("Minute 15, move right");
    // state = state.updated_blizzards().moved_right().unwrap();
    // state.print("Minute 16, move right");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 17, move down");
    // state = state.updated_blizzards().moved_down().unwrap();
    // state.print("Minute 18, move down");
    let (path, cost) = astar(
        &start,
        |s| s.neighbors(),
        |s| s.heuristic(),
        |s| s.success(),
    )
    .unwrap();
    // for (i, state) in path.iter().enumerate() {
    //     state.print(&format!("Minute {i}"))
    // }
    Ok(cost)
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
}
