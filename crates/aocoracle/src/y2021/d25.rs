use crate::AnyError;
use hashbrown::HashSet;
use itertools::Itertools;

type Herd = HashSet<(usize, usize)>;

fn _herds(text: &str) -> Result<(Herd, Herd), AnyError> {
    let mut east = HashSet::new();
    let mut south = HashSet::new();
    for (row, line) in text.lines().enumerate() {
        for (col, cell) in line.trim().chars().enumerate() {
            match cell {
                '>' => {
                    east.insert((row, col));
                }
                'v' => {
                    south.insert((row, col));
                }
                '.' => (),
                _ => return Err(format!("Unexpected cell {}", cell).into()),
            }
        }
    }
    Ok((east, south))
}

fn _fmt_herds(east: &Herd, south: &Herd, height: usize, width: usize) -> String {
    (0..height)
        .map(|row| {
            (0..width)
                .map(|col| {
                    if east.contains(&(row, col)) {
                        '>'
                    } else if south.contains(&(row, col)) {
                        'v'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

fn _new_herds(old_east: &Herd, old_south: &Herd, height: usize, width: usize) -> (Herd, Herd) {
    let mut new_east = HashSet::with_capacity(old_east.len());
    for old_k in old_east {
        let new_k = (old_k.0, (old_k.1 + 1) % width);
        new_east.insert({
            if old_east.contains(&new_k) || old_south.contains(&new_k) {
                *old_k
            } else {
                new_k
            }
        });
    }
    let mut new_south = HashSet::with_capacity(old_south.len());
    for old_k in old_south {
        let new_k = ((old_k.0 + 1) % height, old_k.1);
        new_south.insert({
            if new_east.contains(&new_k) || old_south.contains(&new_k) {
                *old_k
            } else {
                new_k
            }
        });
    }
    (new_east, new_south)
}

fn _num_herds(east: Herd, south: Herd) -> usize {
    let height = *east
        .iter()
        .chain(south.iter())
        .map(|(r, _)| r)
        .max()
        .unwrap()
        + 1;
    let width = *east
        .iter()
        .chain(south.iter())
        .map(|(_, c)| c)
        .max()
        .unwrap()
        + 1;

    let mut herds = [(east, south), (HashSet::new(), HashSet::new())];
    for herd_num in 1.. {
        let old = &herds[(herd_num + 1) % 2];
        let new = _new_herds(&old.0, &old.1, height, width);

        if old.0 == new.0 && old.1 == new.1 {
            return herd_num;
        }
        herds[herd_num % 2] = new;
    }
    panic!("Oups")
}

pub fn part_1(input: &str) -> Result<String, AnyError> {
    let (east, south) = _herds(input)?;
    Ok(_num_herds(east, south).to_string())
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "1a1a16638a95e9aa", Part::One);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1);
    }
}
