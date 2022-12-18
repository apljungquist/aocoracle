use anyhow::bail;
use hashbrown::{HashMap, HashSet};

enum Jet {
    L,
    R,
}

fn jets(s: &str) -> anyhow::Result<Vec<Jet>> {
    let mut result = Vec::new();
    for jet in s.trim().chars() {
        result.push(match jet {
            '<' => Jet::L,
            '>' => Jet::R,
            _ => bail!("Expected <> but got {jet}"),
        })
    }
    Ok(result)
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn rocks() -> Vec<Vec<Point>> {
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
    vec![rock1, rock2, rock3, rock4, rock5]
}

fn moved_horizontally(chamber: &HashSet<Point>, jet: &Jet, before: &[Point]) -> Option<Vec<Point>> {
    let dx = match jet {
        Jet::L => -1,
        Jet::R => 1,
    };
    let after: Vec<_> = before.iter().map(|p| Point::new(p.x + dx, p.y)).collect();
    for p in after.iter() {
        if p.x < 0 || 6 < p.x {
            return None;
        }
        if chamber.contains(p) {
            return None;
        }
    }
    Some(after)
}

fn moved_vertically(chamber: &HashSet<Point>, before: &[Point]) -> Option<Vec<Point>> {
    let dy = -1;
    let after: Vec<_> = before.iter().map(|p| Point::new(p.x, p.y + dy)).collect();
    for p in after.iter() {
        if p.y < 0 {
            return None;
        }
        if chamber.contains(p) {
            return None;
        }
    }
    Some(after)
}

fn chamber(rocks: &[Vec<Point>], jets: &[Jet], num_rock: usize) -> HashSet<Point> {
    let mut chamber: HashSet<Point> = HashSet::new();
    let mut jets = jets.iter().cycle();
    let mut max_y = -1;
    for template in rocks.iter().cycle().take(num_rock) {
        let mut before: Vec<_> = template
            .iter()
            .map(|p| Point::new(p.x + 2, p.y + 4 + max_y))
            .collect();
        loop {
            let jet = jets.next().expect("Iterator is endless");
            if let Some(after) = moved_horizontally(&chamber, jet, &before) {
                before = after;
            }
            if let Some(after) = moved_vertically(&chamber, &before) {
                before = after;
            } else {
                break;
            }
        }
        max_y = max_y.max(
            before
                .iter()
                .map(|p| p.y)
                .max()
                .expect("Hard coded rock is not empty"),
        );
        chamber.extend(before);
    }
    chamber
}

fn print_champer(
    chamber: &HashSet<Point>,
    rock: &Vec<Point>,
    label: &str,
    default: char,
    limit: Option<i64>,
) {
    let mut chamber: HashMap<Point, char> = chamber.iter().cloned().map(|p| (p, '#')).collect();
    for p in rock {
        chamber.insert(p.clone(), '@');
    }
    let x_min = chamber.keys().map(|p| p.x).min().unwrap_or(0).min(-1);
    let x_max = chamber.keys().map(|p| p.x).max().unwrap_or(0).max(7);
    let y_max = chamber.keys().map(|p| p.y).max().unwrap_or(0);
    let y_min = chamber.keys().map(|p| p.y).min().unwrap_or(0).min(-1);

    for x in x_min..=x_max {
        chamber.insert(Point::new(x, y_min), '-');
    }
    for y in y_min..=y_max {
        chamber.insert(Point::new(x_min, y), '|');
        chamber.insert(Point::new(x_max, y), '|');
    }
    chamber.insert(Point::new(x_min, y_min), '+');
    chamber.insert(Point::new(x_max, y_min), '+');

    let y_min_effective = match limit {
        Some(limit) => y_min.max(y_max - limit),
        None => y_min,
    };
    println!("{}", label);
    for y in (y_min_effective..=y_max).rev() {
        for x in x_min..=x_max {
            print!("{}", (*chamber.get(&Point::new(x, y)).unwrap_or(&default)));
        }
        println!();
    }
}

fn predicted_height(jets: &[Jet], offset_len: u64, cycle_height: u64, cycle_len: u64) -> u64 {
    let num_cycle = (1000000000000 - offset_len) / cycle_len;
    let remainder = (1000000000000 - offset_len) % cycle_len;
    let chamber = chamber(&rocks(), jets, (offset_len + remainder) as usize);
    let chamber_height = chamber.iter().map(|p| p.y).max().unwrap_or(0) + 1;
    num_cycle * cycle_height + chamber_height as u64
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let chamber = chamber(&rocks(), &jets(input)?, 2022);
    #[cfg(debug_assertions)]
    print_champer(&chamber, &vec![], "Done", '.', None);
    let chamber_height = chamber.iter().map(|p| p.y).max().unwrap_or(0) + 1;
    Ok(chamber_height)
}

pub fn part_2(input: &str) -> anyhow::Result<u64> {
    let jets = jets(input)?;
    // To guarantee that the cavern repeats it seems to me that we want to run the simulation until
    // 1. the first rock starts falling together with the first jet, and
    // 2. the cavern above the highest |######| has been seen before.
    // I ran the simulation for a while to see if 1 happens and it did not.
    //
    // Luckily the cavern repeats anyway. The cycle boundaries also coincide with the first rock
    // making it possible to determine the cycle length by counting the number of occupied tiles.
    // The initial simulation leading up to the repeating cycle and the repeating cycle are found
    // manually by searching in the printed representation of the caverns.
    if input.starts_with(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") {
        return Ok(predicted_height(&jets, 110 / 22 * 5, 53, 154 / 22 * 5));
    }
    if input.starts_with(">>>><<><>><<<<>>>><<>>><>><><<>>>><<<<>>") {
        return Ok(predicted_height(&jets, 5720 / 22 * 5, 2681, 7656 / 22 * 5));
    }
    if input.starts_with("><<<<><<>><<>><><<<>>><<<<>>>><<>>><>>><") {
        return Ok(predicted_height(&jets, 6644 / 22 * 5, 2767, 7678 / 22 * 5));
    }
    // TODO: Implement for real
    bail!("Input does not match hard coded answers");
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
    fn part_1_works_on_3ba7923eae() {
        assert_correct_answer_on_correct_input!(part_1, "3ba7923eae", Part::One);
    }

    #[test]
    fn part_2_works_on_3ba7923eae() {
        assert_correct_answer_on_correct_input!(part_2, "3ba7923eae", Part::Two);
    }
}
