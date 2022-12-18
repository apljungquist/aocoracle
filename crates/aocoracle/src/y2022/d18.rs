use hashbrown::{HashMap, HashSet};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Face {
    axis: Axis,
    point: Point,
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let droplets = parsed(input)?;
    let mut counts: HashMap<Face, usize> = HashMap::new();
    for face in droplets.iter().flat_map(|p| p.faces()) {
        *counts.entry(face).or_default() += 1;
    }
    let mut result = 0;
    for (k, v) in counts {
        match v {
            1 => result += 1,
            2 => {}
            _ => panic!("Oops"),
        }
    }
    Ok(result)
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
    fn works_on_tiny_example() {
        let droples = [Point::new(1, 1, 1), Point::new(2, 1, 1)];
    }
}
