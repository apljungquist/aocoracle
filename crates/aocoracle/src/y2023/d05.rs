use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::str::FromStr;

use anyhow::bail;
use itertools::Itertools;

trait SetOps {
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn difference(&self, other: &Self) -> Vec<Self>
    where
        Self: Sized;
}

impl<T: Copy + Ord> SetOps for Range<T> {
    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    fn difference(&self, other: &Self) -> Vec<Self> {
        if other.start <= self.start && self.end <= other.end {
            vec![]
        } else if other.end <= self.start || self.end <= other.start {
            vec![self.clone()]
        } else if other.start <= self.start && self.start <= other.end {
            vec![Self {
                start: other.end,
                end: self.end,
            }]
        } else if other.start <= self.end && self.end <= other.end {
            vec![Self {
                start: self.start,
                end: other.start,
            }]
        } else {
            assert!(self.start < other.start && other.end < self.end);
            vec![
                Self {
                    start: self.start,
                    end: other.start,
                },
                Self {
                    start: other.end,
                    end: self.end,
                },
            ]
        }
    }
}

struct MapLine {
    src_start: i64,
    dst_start: i64,
    range_len: i64,
}

impl Debug for MapLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:+}",
            Range {
                start: self.src_start,
                end: self.src_start + self.range_len,
            },
            self.dst_start - self.src_start,
        )
    }
}

impl MapLine {
    fn src_end(&self) -> i64 {
        self.src_start + self.range_len
    }
}

impl MapLine {
    fn apply(&self, subject: Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
        let mappable = self.src_start..self.src_end();
        let unmapped = subject.difference(&mappable);
        let mapped = subject.intersection(&mappable).map(|intersection| {
            let offset = self.dst_start - self.src_start;
            (intersection.start + offset)..(intersection.end + offset)
        });
        (mapped, unmapped)
    }
}

#[derive(Debug)]
struct Map {
    lines: Vec<MapLine>,
}

impl Map {
    fn apply(&self, src: Range<i64>) -> Vec<Range<i64>> {
        let mut all_unmapped = vec![src];
        let mut all_mapped = Vec::new();
        for line in &self.lines {
            all_unmapped = all_unmapped
                .into_iter()
                .flat_map(|range| {
                    let (mapped, unmapped) = line.apply(range);
                    all_mapped.extend(mapped);
                    unmapped
                })
                .collect();
        }
        all_mapped.extend(all_unmapped.into_iter());
        all_mapped
    }
}

struct Almanac {
    seed_numbers: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn seed_ranges_1(&self) -> Vec<Range<i64>> {
        self.seed_numbers
            .iter()
            .map(|&start| start..start + 1)
            .collect()
    }

    fn seed_ranges_2(&self) -> Vec<Range<i64>> {
        if self.seed_numbers.len() % 2 != 0 {
            panic!("Expected even number of seed numbers")
        }
        self.seed_numbers
            .iter()
            .chunks(2)
            .into_iter()
            .map(|endpoints| {
                let endpoints: Vec<_> = endpoints.cloned().collect();
                endpoints[0]..endpoints[0] + endpoints[1]
            })
            .collect()
    }

    fn closest_location(&self, seed_numbers_are_ranges: bool) -> i64 {
        let mut ranges = if seed_numbers_are_ranges {
            self.seed_ranges_2()
        } else {
            self.seed_ranges_1()
        };
        for map in &self.maps {
            ranges = ranges
                .into_iter()
                .flat_map(|range| map.apply(range))
                .collect();
            // ranges.sort_by_key(|r|r.start);
        }
        ranges.into_iter().map(|r| r.start).min().expect("Constructor ensures that there is at least one seed number and maps cannot reduce the number of numbers from non-zero to zero.")
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: VecDeque<_> = s.lines().collect();
        let seed_numbers = take_seed_numbers(&mut lines)?;
        let mut maps = Vec::with_capacity(7);
        for _ in 0..7 {
            maps.push(take_map(&mut lines)?);
        }
        Ok(Self { seed_numbers, maps })
    }
}

fn take_seed_numbers(lines: &mut VecDeque<&str>) -> anyhow::Result<Vec<i64>> {
    let line = lines
        .pop_front()
        .ok_or_else(|| anyhow::anyhow!("Expected another line"))?;
    let (title, numbers) = line
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("Expected line containing ':' but got {line:?}"))?;
    if title != "seeds" {
        bail!("Expected title 'seeds' but got {title:?}")
    }
    if !lines.pop_front().unwrap_or_default().is_empty() {
        bail!("Expected empty line after seeds")
    }
    let mut seed_numbers = Vec::new();
    for number in numbers.split_whitespace() {
        seed_numbers.push(number.parse()?);
    }
    Ok(seed_numbers)
}

fn take_map(lines: &mut VecDeque<&str>) -> anyhow::Result<Map> {
    lines
        .pop_front()
        .ok_or_else(|| anyhow::anyhow!("Expected map to begin with header"))?;
    let mut map_lines = Vec::new();
    loop {
        let Some(line) = lines.pop_front() else {
            break;
        };
        if line.is_empty() {
            break;
        }
        let line: Vec<_> = line.split_whitespace().collect();
        if line.len() != 3 {
            bail!("Expected line with 3 numbers but got {line:?}")
        }
        let src_start = line[1].parse()?;
        let dst_start = line[0].parse()?;
        let range_len = line[2].parse()?;
        map_lines.push(MapLine {
            src_start,
            dst_start,
            range_len,
        })
    }
    // map_lines.sort_by_key(|line| line.src_start);
    Ok(Map { lines: map_lines })
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let almanac = input.parse::<Almanac>()?;
    Ok(almanac.closest_location(false))
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let almanac = input.parse::<Almanac>()?;
    Ok(almanac.closest_location(true))
}

#[cfg(test)]
mod tests {
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};
    use crate::Part;

    use super::*;

    #[test]
    fn split_once() {
        let map_line = MapLine {
            src_start: 10,
            dst_start: 110,
            range_len: 10,
        };
        assert_eq!(dbg!(map_line.apply(0..10)), (None, vec![0..10]));
        assert_eq!(dbg!(map_line.apply(20..30)), (None, vec![20..30]));
        assert_eq!(
            dbg!(map_line.apply(0..30)),
            (Some(110..120), vec![0..10, 20..30])
        );
        assert_eq!(dbg!(map_line.apply(11..19)), (Some(111..119), vec![]));
    }

    #[test]
    fn split_once_again() {
        let map_line = MapLine {
            src_start: 56,
            dst_start: 60,
            range_len: 37,
        };
        assert_eq!(dbg!(map_line.apply(46..57)), (Some(60..61), vec![46..56]));
    }

    #[test]
    fn split_twice() {
        let map = Map {
            lines: vec![
                MapLine {
                    src_start: 98,
                    dst_start: 50,
                    range_len: 2,
                },
                MapLine {
                    src_start: 50,
                    dst_start: 52,
                    range_len: 48,
                },
            ],
        };
        dbg!(&map);
        assert_eq!(map.apply(79..93), vec![81..95]);
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "c36053593677d647", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "c36053593677d647", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
