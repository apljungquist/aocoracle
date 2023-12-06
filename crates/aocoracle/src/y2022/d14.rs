use anyhow::{anyhow, bail};
use hashbrown::HashMap;
use std::fmt::Debug;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn rock_paths(s: &str) -> anyhow::Result<Vec<Vec<Point>>> {
    let mut result = Vec::new();
    for line in s.lines() {
        let mut path = Vec::new();
        for point in line.split(" -> ") {
            let (x, y) = point
                .split_once(',')
                .ok_or_else(|| anyhow!("Expected exactly one ','"))?;
            let x = x.parse()?;
            let y = y.parse()?;
            path.push(Point { x, y });
        }
        result.push(path);
    }
    Ok(result)
}

fn rock_coordinates(paths: &Vec<Vec<Point>>) -> anyhow::Result<HashMap<Point, Tile>> {
    let mut result = HashMap::new();
    for path in paths {
        for line in path.windows(2) {
            let start = line.get(0).expect("0 is in bounds for window of size 2");
            let end = line.get(1).expect("1 is in bounds for window of size 2");
            if start.x == end.x {
                let x = start.x;
                for y in start.y.min(end.y)..=start.y.max(end.y) {
                    result.insert(Point { x, y }, Tile::Rock);
                }
            } else if start.y == end.y {
                let y = start.y;
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    result.insert(Point { x, y }, Tile::Rock);
                }
            } else {
                bail!("Expected vertical or horizontal line")
            }
        }
    }
    if result.is_empty() {
        bail!("Expected at least one rock");
    }
    Ok(result)
}

fn grid(s: &str) -> anyhow::Result<Grid<Tile>> {
    let paths = rock_paths(s);
    let mut grid = Grid::try_new(rock_coordinates(&paths?)?)?;
    grid.0.insert(Point { x: 500, y: 0 }, Tile::Source);
    Ok(grid)
}

fn moved_sand<T>(grid: &Grid<T>, curr: &Point) -> Option<Point> {
    for dx in [0, -1, 1] {
        let next = Point {
            x: curr.x + dx,
            y: curr.y + 1,
        };
        if grid.0.get(&next).is_none() {
            return Some(next);
        }
    }
    None
}

struct Grid<T>(HashMap<Point, T>);

impl<T> Grid<T> {
    fn try_new(tiles: HashMap<Point, T>) -> anyhow::Result<Self> {
        if tiles.is_empty() {
            bail!("Expected at least one tile");
        }
        Ok(Self(tiles))
    }

    fn x_min(&self) -> i32 {
        self.0
            .keys()
            .map(|p| p.x)
            .min()
            .expect("Constructor ensures there is at least one tile")
    }
    fn x_max(&self) -> i32 {
        self.0
            .keys()
            .map(|p| p.x)
            .max()
            .expect("Constructor ensures there is at least one tile")
    }
    fn y_min(&self) -> i32 {
        self.0
            .keys()
            .map(|p| p.y)
            .min()
            .expect("Constructor ensures there is at least one tile")
    }
    fn y_max(&self) -> i32 {
        self.0
            .keys()
            .map(|p| p.y)
            .max()
            .expect("Constructor ensures there is at least one tile")
    }
}

// TODO: Can this be implemented without `T: Copy`?
impl<T: 'static> Grid<T>
where
    char: From<T>,
    T: Copy,
{
    #[cfg(debug_assertions)]
    fn print(&self, label: &str, default: T) {
        println!("{}", label);
        for y in self.y_min()..=self.y_max() {
            for x in self.x_min()..=self.x_max() {
                print!(
                    "{}",
                    char::from(*self.0.get(&Point { x, y }).unwrap_or(&default))
                );
            }
            println!();
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Rock,
    Sand,
    Source,
}

impl From<Tile> for char {
    fn from(t: Tile) -> char {
        match t {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
            Tile::Source => '+',
        }
    }
}

enum SimulationResult {
    SourceBlocked,
    RunsForever,
}

fn run_simulation(grid: &mut Grid<Tile>) -> (SimulationResult, usize) {
    let y_max = grid.y_max();
    for i in 0.. {
        // TODO: Keep track of previous insertion points to avoid quadratic runtime
        let mut curr = Point { x: 500, y: 0 };
        while let Some(next) = moved_sand(grid, &curr) {
            if next.y == y_max {
                return (SimulationResult::RunsForever, i);
            }
            curr = next;
        }
        if curr.y == 0 {
            return (SimulationResult::SourceBlocked, i + 1);
        }
        grid.0.insert(curr, Tile::Sand);
    }
    unreachable!()
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut grid = grid(input)?;
    #[cfg(debug_assertions)]
    grid.print("Before", Tile::Air);
    let result = match run_simulation(&mut grid) {
        (SimulationResult::SourceBlocked, _) => Err(anyhow!("Expected simulation to run forever")),
        (SimulationResult::RunsForever, num_sand_unit) => Ok(num_sand_unit),
    };
    #[cfg(debug_assertions)]
    grid.print("After", Tile::Air);
    result
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut grid = grid(input)?;
    let y_max = grid.y_max() + 2;
    for x in (500 - y_max)..=(500 + y_max) {
        grid.0.insert(Point { x, y: y_max }, Tile::Rock);
    }
    #[cfg(debug_assertions)]
    grid.print("Before", Tile::Air);
    let result = match run_simulation(&mut grid) {
        (SimulationResult::SourceBlocked, num_sand_unit) => Ok(num_sand_unit),
        (SimulationResult::RunsForever, _) => Err(anyhow!("Expected source to become blocked")),
    };
    #[cfg(debug_assertions)]
    grid.print("After", Tile::Air);
    result
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
        assert_correct_answer_on_correct_input!("e9f04d40c5066aaa", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("e9f04d40c5066aaa", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
