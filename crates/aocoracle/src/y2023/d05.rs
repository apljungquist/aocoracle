use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::str::FromStr;

use anyhow::bail;
use itertools::Itertools;

use crate::y2023::d05::Resource::Seed;

trait RangeOps {
    fn intersection(&self, other: &Self) -> Option<Self> where Self: Sized;
    fn difference(&self, other: &Self) -> Vec<Self> where Self: Sized;
}

impl<T: Copy + Ord> RangeOps for Range<T> {
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
            vec![Self { start: other.end, end: self.end }]
        } else if other.start <= self.end && self.end <= other.end {
            vec![Self { start: self.start, end: other.start }]
        } else {
            assert!(self.start < other.start && other.end < self.end);
            vec![
                Self { start: self.start, end: other.start },
                Self { start: other.end, end: self.end },
            ]
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Resource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => bail!("Expected a resource but got {s}"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct MapHeader {
    src: Resource,
    dst: Resource,
}

struct MapLine {
    src_start: i64,
    dst_start: i64,
    range_len: i64,
}

impl Debug for MapLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}",
               Range { start: self.src_start, end: self.src_start + self.range_len },
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
    fn split(&self, range: Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
        let src = self.src_start..self.src_end();
        let unmapped = range.difference(&src);
        let mapped = range.intersection(&src).map(|r| {
            let offset = self.dst_start - self.src_start;
            (r.start + offset)..(r.end + offset)
        });
        (mapped, unmapped)
    }
}

#[derive(Debug)]
struct MapBody {
    map_lines: Vec<MapLine>,
}

impl MapBody {
    fn get(&self, src: &i64) -> Option<i64> {
        for line in &self.map_lines {
            let src_start = line.src_start;
            let srd_end = src_start + line.range_len;
            if (src_start..srd_end).contains(src) {
                return Some(src - src_start + line.dst_start);
            }
        }
        None
    }

    fn split(&self, src: Range<i64>) -> Vec<Range<i64>> {
        let mut remaining = vec![src];
        let mut done = Vec::new();
        for line in &self.map_lines {
            remaining = remaining.into_iter().flat_map(|range| {
                let (mapped, unmapped) = line.split(range);
                done.extend(mapped);
                unmapped
            }).collect();
        }
        done.extend(remaining.into_iter());
        done
    }
}

fn take_seed_numbers1(lines: &mut VecDeque<&str>) -> anyhow::Result<Vec<i64>> {
    let line = lines.pop_front().unwrap();
    let (title, numbers) = line.split_once(":").unwrap();
    assert!(lines.pop_front().unwrap().is_empty());
    Ok(numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect())
}

fn take_seed_numbers2(lines: &mut VecDeque<&str>) -> anyhow::Result<Vec<Range<i64>>> {
    let line = lines.pop_front().unwrap();
    let (title, numbers) = line.split_once(":").unwrap();
    assert!(lines.pop_front().unwrap().is_empty());
    Ok(numbers
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|(mut chunk)| {
            let start: i64 = chunk.next().unwrap().parse().unwrap();
            let len: i64 = chunk.next().unwrap().parse().unwrap();
            let end = start + len;
            start..end
        })
        .collect())
}

fn take_map_header(lines: &mut VecDeque<&str>) -> anyhow::Result<MapHeader> {
    let line = lines.pop_front().unwrap();
    let (mapping, _) = line.split_once(' ').unwrap();
    let parts: Vec<_> = mapping.split('-').collect();
    let src = parts.get(0).unwrap().parse()?;
    let dst = parts.get(2).unwrap().parse()?;
    Ok(MapHeader { src, dst })
}

fn take_map_body(lines: &mut VecDeque<&str>) -> anyhow::Result<MapBody> {
    let mut map_lines = Vec::new();
    loop {
        let Some(line) = lines.pop_front() else {
            break;
        };
        if line.is_empty() {
            break;
        }
        let line: Vec<_> = line.split_whitespace().collect();
        let src_start = line.get(1).unwrap().parse()?;
        let dst_start = line.get(0).unwrap().parse()?;
        let range_len = line.get(2).unwrap().parse()?;
        map_lines.push(MapLine {
            src_start,
            dst_start,
            range_len,
        })
    }
    map_lines.sort_by_key(|l| l.src_start);
    Ok(MapBody { map_lines })
}

fn take_resource_maps(lines: &mut VecDeque<&str>) -> anyhow::Result<HashMap<Resource, MapBody>> {
    let mut resource_maps = HashMap::new();
    while !lines.is_empty() {
        let header = take_map_header(lines)?;
        let body = take_map_body(lines)?;
        resource_maps.insert(header.src, body);
    }
    Ok(resource_maps)
}

fn naive(
    seed_numbers: Vec<i64>,
    resource_maps: HashMap<Resource, MapBody>,
) -> anyhow::Result<i64> {
    use Resource::*;
    let mut location2seed = HashMap::with_capacity(seed_numbers.len());
    for seed in seed_numbers {
        let mut number = seed;
        for src in [Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity] {
            if let Some(n) = resource_maps.get(&src).unwrap().get(&number) {
                number = n;
            }
        }
        location2seed.insert(number, seed);
    }
    let location = location2seed.keys().min().unwrap();
    Ok(*location)
}

fn less_naive(
    seed_numbers: Vec<Range<i64>>,
    resource_maps: HashMap<Resource, MapBody>,
) -> anyhow::Result<i64> {
    use Resource::*;

    let mut ranges = seed_numbers;
    for src in [Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity] {
        ranges.sort_by_key(|r| r.start);
        let resource_map = resource_maps.get(&src).unwrap();
        ranges = ranges.into_iter().flat_map(|r| resource_map.split(r)).collect();
    }
    Ok(ranges.into_iter().map(|r| r.start).min().unwrap())
}

pub fn part_1(input: &str) -> anyhow::Result<i64> {
    let mut lines: VecDeque<_> = input.lines().collect();
    let seed_numbers = take_seed_numbers1(&mut lines)?;
    let resource_maps = take_resource_maps(&mut lines)?;
    naive(seed_numbers, resource_maps)
}

pub fn part_2(input: &str) -> anyhow::Result<i64> {
    let mut lines: VecDeque<_> = input.lines().collect();
    let seed_numbers = take_seed_numbers2(&mut lines)?;
    let resource_maps = take_resource_maps(&mut lines)?;
    less_naive(seed_numbers, resource_maps)
}

#[cfg(test)]
mod tests {
    use crate::Part;
    use crate::testing::{assert_correct_answer_on_correct_input, assert_error_on_wrong_input};

    use super::*;

    #[test]
    fn split_once() {
        let map_line = MapLine { src_start: 10, dst_start: 110, range_len: 10 };
        assert_eq!(
            dbg!(map_line.split(0..10)),
            (None, vec![0..10])
        );
        assert_eq!(
            dbg!(map_line.split(20..30)),
            (None, vec![20..30])
        );
        assert_eq!(
            dbg!(map_line.split(0..30)),
            (Some(110..120), vec![0..10, 20..30])
        );
        assert_eq!(
            dbg!(map_line.split(11..19)),
            (Some(111..119), vec![])
        );
    }

    #[test]
    fn split_once_again() {
        let map_line = MapLine { src_start: 56, dst_start: 60, range_len: 37 };
        assert_eq!(
            dbg!(map_line.split(46..57)),
            (Some(60..61), vec![46..56])
        );
    }

    #[test]
    fn split_twice() {
        let map = MapBody {
            map_lines: vec![
                MapLine { src_start: 98, dst_start: 50, range_len: 2 },
                MapLine { src_start: 50, dst_start: 52, range_len: 48 },
            ]
        };
        dbg!(&map);
        assert_eq!(
            map.split(79..93),
            vec![81..95]
        );
    }

    #[test]
    fn part_1_works_on_example() {
        assert_correct_answer_on_correct_input!(part_1, "EXAMPLE", Part::One);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_correct_answer_on_correct_input!(part_1, "INPUT", Part::One);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_correct_answer_on_correct_input!(part_2, "EXAMPLE", Part::Two);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_correct_answer_on_correct_input!(part_2, "INPUT", Part::Two);
    }

    #[test]
    fn returns_error_on_wrong_input() {
        assert_error_on_wrong_input!(part_1, part_2);
    }
}
