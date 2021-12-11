use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::iter::zip;

use regex;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Arrow {
    tail: Point,
    head: Point,
}

fn interpolated(start: i32, stop: i32) -> Vec<i32> {
    if start < stop {
        (start..=stop).collect()
    } else {
        (stop..=start).rev().collect()
    }
}

impl Arrow {
    fn parse(line: &str) -> Arrow {
        let re = regex::Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let cap = re.captures(line).unwrap();
        Arrow {
            tail: Point {
                x: cap[1].parse::<i32>().unwrap(),
                y: cap[2].parse::<i32>().unwrap(),
            },
            head: Point {
                x: cap[3].parse::<i32>().unwrap(),
                y: cap[4].parse::<i32>().unwrap(),
            },
        }
    }

    fn is_horizontal(&self) -> bool {
        self.head.y == self.tail.y
    }
    fn is_vertical(&self) -> bool {
        self.head.x == self.tail.x
    }

    fn points(&self, include_diagonal: bool) -> Vec<Point> {
        let mut result = Vec::new();
        match (self.is_horizontal(), self.is_vertical()) {
            (false, false) => {
                if include_diagonal {
                    for (x, y) in zip(
                        interpolated(self.tail.x, self.head.x),
                        interpolated(self.tail.y, self.head.y),
                    ) {
                        result.push(Point { x, y })
                    }
                }
            }
            (false, true) => {
                let x = self.tail.x;
                assert_eq!(x, self.head.x);
                for y in interpolated(self.tail.y, self.head.y) {
                    result.push(Point { x, y })
                }
            }
            (true, false) => {
                let y = self.tail.y;
                assert_eq!(y, self.head.y);
                for x in interpolated(self.tail.x, self.head.x) {
                    result.push(Point { x, y })
                }
            }
            (true, true) => panic!("Impossible"),
        }
        result
    }
}

fn _counts(points: Vec<Point>) -> HashMap<Point, u32> {
    let mut result = HashMap::new();
    for point in points {
        *(result.entry(point).or_insert(0)) += 1;
    }
    result
}

fn _read_input(name: &str) -> String {
    fs::read_to_string(format!("day/5/{}", name)).unwrap()
}

fn _read_arrows(name: &str) -> Vec<Arrow> {
    _read_input(name).lines().map(Arrow::parse).collect()
}

fn _print_grid(counts: &HashMap<Point, u32>) {
    let max_row = counts.iter().map(|(p, _)| p.y).max().unwrap();
    let max_col = counts.iter().map(|(p, _)| p.x).max().unwrap();
    println!("r:{:?}, c:{:?}", max_row, max_col);
    for row_num in 0..=max_row {
        for col_num in 0..=max_col {
            print!(
                "{}",
                counts
                    .get(&Point {
                        x: col_num,
                        y: row_num
                    })
                    .unwrap_or(&0)
            );
        }
        println!();
    }
}

fn _risk(arrows: Vec<Arrow>, include_diagonal: bool) -> u32 {
    let counts = _counts(
        arrows
            .into_iter()
            .filter(|a| include_diagonal || a.is_horizontal() || a.is_vertical())
            .flat_map(|a| a.points(include_diagonal))
            .map(|p| {
                if p.y > 1000 {
                    println!("{:?}", p)
                };
                p
            })
            .collect(),
    );
    counts.into_iter().filter(|(_, c)| 2 <= *c).count() as u32
}

pub fn part_1(filename: &str) -> u32 {
    let arrows = _read_arrows(filename);
    _risk(arrows, false)
}

pub fn part_2(filename: &str) -> u32 {
    let arrows = _read_arrows(filename);
    _risk(arrows, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(part_1("example.txt"), 5);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(part_1("input.txt"), 6225);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(part_2("example.txt"), 12);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(part_2("input.txt"), 22116);
    }
}
