use std::fs;

use hashbrown::HashSet;
use itertools::Itertools;

type Herd = HashSet<(usize, usize)>;

fn _herds(text: &str) -> (Herd, Herd) {
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
                _ => panic!("Unexpected cell {}", cell),
            }
        }
    }
    (east, south)
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

pub fn part_1(input: &str) -> u64 {
    let (east, south) = _herds(input);
    _num_herds(east, south) as u64
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/25/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 58);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 516);
    }
}
