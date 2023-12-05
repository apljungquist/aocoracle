use crate::y2023::d05::Resource::Seed;
use anyhow::bail;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

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

#[derive(Debug)]
struct MapLine {
    src_start: usize,
    dst_start: usize,
    range_len: usize,
}

#[derive(Debug)]
struct MapBody {
    map_lines: Vec<MapLine>,
}

impl MapBody {
    fn get(&self, src: &usize) -> Option<usize> {
        for line in &self.map_lines {
            let src_start = line.src_start;
            let srd_end = src_start + line.range_len;
            if (src_start..srd_end).contains(src) {
                return Some(src - src_start + line.dst_start);
            }
        }
        None
    }
}

fn take_seed_numbers(lines: &mut VecDeque<&str>) -> anyhow::Result<Vec<usize>> {
    let line = lines.pop_front().unwrap();
    let (title, numbers) = line.split_once(":").unwrap();
    assert!(lines.pop_front().unwrap().is_empty());
    Ok(numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
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
    Ok(MapBody { map_lines })
}

pub fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut lines: VecDeque<_> = input.lines().collect();

    let seed_numbers = take_seed_numbers(&mut lines)?;

    let mut resource_maps = HashMap::new();
    while !lines.is_empty() {
        let header = take_map_header(&mut lines)?;
        let body = take_map_body(&mut lines)?;
        resource_maps.insert(header.src, body);
    }


    use Resource::*;
    let mut location2seed = HashMap::with_capacity(seed_numbers.len());
    for seed in seed_numbers {
        let mut number = seed;
        println!("#######");
        for src in [Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity] {
            println!("Maping {src:?} {number}");
            if let Some(n) = resource_maps.get(&src).unwrap().get(&number) {
                number = n;
            }
        }
        location2seed.insert(number, seed);
    }
    let location = location2seed.keys().min().unwrap();
    Ok(*location)
}

pub fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;
    Ok(sum)
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
