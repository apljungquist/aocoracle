use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    fn faces(&self) -> Vec<Face> {
        vec![
            Face {
                point: Point {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                },
                axis: Axis::X,
            },
            Face {
                point: Point {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                },
                axis: Axis::Y,
            },
            Face {
                point: Point {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                },
                axis: Axis::Z,
            },
            Face {
                point: Point {
                    x: self.x - 1,
                    y: self.y,
                    z: self.z,
                },
                axis: Axis::X,
            },
            Face {
                point: Point {
                    x: self.x,
                    y: self.y - 1,
                    z: self.z,
                },
                axis: Axis::Y,
            },
            Face {
                point: Point {
                    x: self.x,
                    y: self.y,
                    z: self.z - 1,
                },
                axis: Axis::Z,
            },
        ]
    }
}

fn parsed(s: &str) -> anyhow::Result<HashSet<Point>> {
    let mut result = HashSet::new();
    for line in s.lines() {
        let p: Vec<_> = line.split(',').collect();
        let x = p[0].parse()?;
        let y = p[1].parse()?;
        let z = p[2].parse()?;
        result.insert(Point { x, y, z });
    }
    Ok(result)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Face {
    axis: Axis,
    point: Point,
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let droplets = parsed(input)?;
    let mut faces: HashSet<_> = HashSet::new();
    for face in droplets.iter().flat_map(|p| p.faces()) {
        if faces.contains(&face) {
            faces.remove(&face);
        } else {
            faces.insert(face);
        }
    }
    Ok(faces.len())
}

fn neighbors(f1: &Face) -> Vec<Face> {
    let mut result = Vec::new();
    let (dx, dy, dz) = match f1.axis {
        Axis::X => (1, 0, 0),
        Axis::Y => (0, 1, 0),
        Axis::Z => (0, 0, 1),
    };
    let points = [
        f1.point.clone(),
        Point::new(f1.point.x + dx, f1.point.y + dy, f1.point.z + dz),
    ];
    for point in points {
        for f2 in point.faces() {
            if f1.axis != f2.axis {
                result.push(f2);
            }
        }
    }
    assert_eq!(result.len(), 8);
    result
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let droplets = parsed(input)?;
    let mut faces: HashSet<_> = HashSet::new();
    for face in droplets.iter().flat_map(|p| p.faces()) {
        if faces.contains(&face) {
            faces.remove(&face);
        } else {
            faces.insert(face);
        }
    }

    let mut explored = HashSet::new();
    let mut remaining = Vec::new();
    remaining.push(faces.iter().min().unwrap().clone());

    while let Some(f1) = remaining.pop() {
        if explored.contains(&f1) {
            continue;
        }
        for f2 in neighbors(&f1) {
            if faces.contains(&f2) {
                remaining.push(f2);
            }
        }
        println!("{} {} {} {:?}", f1.point.x, f1.point.y, f1.point.z, f1.axis);
        explored.insert(f1);
    }
    println!("Not explored");
    for f1 in faces.iter().sorted() {
        if !explored.contains(f1) {
            println!("{} {} {} {:?}", f1.point.x, f1.point.y, f1.point.z, f1.axis);
        }
    }

    let answer = explored.len();
    assert!(answer == 58 || answer < 3402);
    Ok(answer)
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
