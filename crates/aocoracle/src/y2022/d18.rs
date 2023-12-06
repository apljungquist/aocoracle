use anyhow::anyhow;
use hashbrown::HashSet;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn faces(&self) -> Vec<Face> {
        let mut result = Vec::with_capacity(6);
        for (dx, dy, dz) in [(0, 0, 1), (0, 1, 0), (1, 0, 0)] {
            result.push(Face {
                lo: self.clone(),
                hi: Self {
                    x: self.x + dx,
                    y: self.y + dy,
                    z: self.z + dz,
                },
            });
            result.push(Face {
                hi: self.clone(),
                lo: Self {
                    x: self.x - dx,
                    y: self.y - dy,
                    z: self.z - dz,
                },
            });
        }
        result
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(8);
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                for dz in [-1, 0, 1] {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
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
        for face in self.faces() {
            if faces.contains(&face) {
                continue; // Crosses surface
            }
            let neighbor = if face.lo != *self {
                face.lo
            } else {
                assert_ne!(face.hi, *self);
                face.hi
            };
            if !points.contains(&neighbor) {
                continue; // Strays away from surface
            }
            result.push(neighbor);
        }
        result
    }
}

fn droplet(s: &str) -> anyhow::Result<HashSet<Cube>> {
    let re = regex::Regex::new(r"^(\d+),(\d+),(\d+)$").expect("Hard coded regex is valid");
    let mut result = HashSet::new();
    for line in s.lines() {
        let cap = re
            .captures(line)
            .ok_or_else(|| anyhow!("Could not capture a cube on line {}", line))?;
        result.insert(Cube::new(cap[1].parse()?, cap[2].parse()?, cap[3].parse()?));
    }
    Ok(result)
}

// A face can be identified by the two cubes that it touches.
// NB. That the order is important.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Face {
    lo: Cube,
    hi: Cube,
}

fn surface(cubes: &HashSet<Cube>) -> HashSet<Face> {
    let mut result = HashSet::new();
    for face in cubes.iter().flat_map(|f| f.faces()) {
        if result.contains(&face) {
            result.remove(&face);
        } else {
            result.insert(face);
        }
    }
    result
}

fn clique(faces: &HashSet<Face>, points: &HashSet<Cube>, start: Cube) -> HashSet<Cube> {
    let mut result = HashSet::new();
    let mut remaining = vec![start];

    while let Some(p) = remaining.pop() {
        if result.contains(&p) {
            continue;
        }
        for q in p.searchable_neighbors(faces, points) {
            remaining.push(q);
        }
        result.insert(p);
    }
    result
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let droplet = droplet(input)?;
    let surface = surface(&droplet);
    Ok(surface.len())
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let droplet = droplet(input)?;
    let surface = surface(&droplet);
    let searchable: HashSet<_> = droplet.iter().flat_map(|p| p.neighbors()).collect();
    let start = searchable.iter().min().unwrap().clone();
    let accessible = clique(&surface, &searchable, start)
        .iter()
        .flat_map(|cube| cube.faces())
        .filter(|face| surface.contains(face))
        .count();
    Ok(accessible)
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
        assert_correct_answer_on_correct_input!("89fc896b3a0296e4", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!("EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!("89fc896b3a0296e4", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(Part::One, Part::Two);
    }
}
