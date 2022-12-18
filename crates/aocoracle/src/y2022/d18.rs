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
                p: Self { ..*self },
                q: Self {
                    x: self.x + 1,
                    ..*self
                },
            },
            Face {
                p: Self { ..*self },
                q: Self {
                    y: self.y + 1,
                    ..*self
                },
            },
            Face {
                p: Self { ..*self },
                q: Self {
                    z: self.z + 1,
                    ..*self
                },
            },
            Face {
                p: Self {
                    x: self.x - 1,
                    ..*self
                },
                q: Self { ..*self },
            },
            Face {
                p: Self {
                    y: self.y - 1,
                    ..*self
                },
                q: Self { ..*self },
            },
            Face {
                p: Self {
                    z: self.z - 1,
                    ..*self
                },
                q: Self { ..*self },
            },
        ]
    }

    fn with_neighbors(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(9);
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                for dz in [-1, 0, 1] {
                    result.push(Self {
                        x: self.x + dx,
                        y: self.y + dy,
                        z: self.z + dz,
                    })
                }
            }
        }
        result
    }

    fn searchable_neighbors(&self, faces: &HashSet<Face>, points: &HashSet<Self>) -> Vec<Self> {
        let mut result = Vec::with_capacity(6);
        for f in self.faces() {
            let other = if f.p != *self {
                f.p.clone()
            } else {
                assert_ne!(f.q, *self);
                f.q.clone()
            };
            if faces.contains(&f) {
                continue;
            }
            if !points.contains(&other) {
                continue;
            }
            result.push(other);
        }
        result
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
struct Face {
    p: Point,
    q: Point,
}

fn clique(faces: &HashSet<Face>, points: &HashSet<Point>, start: Point) -> HashSet<Point> {
    let mut explored = HashSet::new();
    let mut remaining = vec![start];

    while let Some(p) = remaining.pop() {
        if explored.contains(&p) {
            continue;
        }
        for q in p.searchable_neighbors(faces, points) {
            remaining.push(q);
        }
        explored.insert(p);
    }
    explored
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

    let search_space: HashSet<_> = droplets.iter().flat_map(|p| p.with_neighbors()).collect();
    let start = search_space.iter().min().unwrap().clone();
    let accessible_points = clique(&faces, &search_space, start);
    let accessible_faces: HashSet<_> = accessible_points
        .iter()
        .flat_map(|p| p.faces())
        .filter(|f| faces.contains(f))
        .collect();

    let answer = accessible_faces.len();
    println!("{}", answer);
    assert!(answer == 58 || answer < 3402);
    assert!(answer == 58 || answer > 1038);
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
