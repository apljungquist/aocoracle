#![feature(iter_zip)]

use std::collections::{BTreeMap, HashMap};

use structopt::StructOpt;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long)]
    day: Option<u32>,
    #[structopt(long)]
    part: Option<u32>,
}

const NUM_DAY: u32 = 2;

type AnyError = Box<dyn std::error::Error>;
type Solver = dyn Fn(&str) -> Result<String, AnyError>;

fn _candidates(args: &Cli) -> Result<BTreeMap<(u32, u32), &Solver>, AnyError> {
    let mut functions: HashMap<(u32, u32), &Solver> = HashMap::new();
    functions.insert((1, 1), &day_1::part_1);
    functions.insert((1, 2), &day_1::part_2);
    functions.insert((2, 1), &day_2::part_1);
    functions.insert((2, 2), &day_2::part_2);
    let functions = functions;

    let mut result = BTreeMap::new();
    let parts: Vec<u32> = match args.part {
        None => 1..=NUM_DAY,
        Some(1) => 1..=1,
        Some(2) => 2..=2,
        _ => return Err("Invalid part".into()),
    }
    .collect();

    let days = match args.day {
        None => 1..=NUM_DAY,
        Some(day) if (1..=NUM_DAY).contains(&day) => day..=day,
        _ => return Err("Invalid day".into()),
    };

    for day in days {
        for part in &parts {
            result.insert((day, *part), functions[&(day, *part)]);
        }
    }

    Ok(result)
}

pub fn helper(args: &Cli, text: &str) -> Result<Vec<String>, AnyError> {
    let candidates: BTreeMap<(u32, u32), &Solver> = _candidates(args)?;
    if candidates.is_empty() {
        return Err("Invalid day and part".into());
    }

    let mut result = Vec::new();
    for ((day, part), func) in candidates.iter() {
        eprintln!("Trying day {} part {}", day, part);
        let prefix = match (args.day, args.part) {
            (None, None) => format!("Day {} part {}: ", day, part),
            (None, Some(_)) => format!("Day {}: ", day),
            (Some(_), None) => format!("Part {}: ", part),
            (Some(_), Some(_)) => format!(""),
        };
        match func(text) {
            Ok(output) => result.push(format!("{}{}", prefix, output)),
            Err(error) => match args.day {
                None => eprintln!("{}", error),
                Some(_) => return Err(error),
            },
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const STEMS: [&str; 2] = ["example", "input"];

    #[test]
    fn specific_day_returns_2_solutions_on_right_input() {
        for day in 1..=NUM_DAY {
            let args = Cli {
                day: Some(day),
                part: None,
            };
            let input_day = day;
            for stem in STEMS {
                let stdin = fs::read_to_string(format!("day/{}/{}.txt", input_day, stem)).unwrap();
                println!("day:{} input_day:{} stem:{}", day, input_day, stem);
                assert_eq!(helper(&args, &stdin).unwrap().len(), 2);
            }
        }
    }

    #[test]
    fn specific_part_returns_1_solution_on_right_input() {
        for part in 1..=2 {
            let args = Cli {
                day: None,
                part: Some(part),
            };
            for input_day in 1..=NUM_DAY {
                for stem in STEMS {
                    let stdin =
                        fs::read_to_string(format!("day/{}/{}.txt", input_day, stem)).unwrap();
                    println!("day:* input_day:{} stem:{}", input_day, stem);
                    assert_eq!(helper(&args, &stdin).unwrap().len(), 1);
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn specific_day_returns_error_on_wrong_input() {
        for day in 1..=NUM_DAY {
            let args = Cli {
                day: Some(day),
                part: None,
            };
            for input_day in (1..=NUM_DAY).filter(|d| *d != day) {
                for stem in STEMS {
                    let stdin =
                        fs::read_to_string(format!("day/{}/{}.txt", input_day, stem)).unwrap();
                    println!("day:{} input_day:{} stem:{}", day, input_day, stem);
                    helper(&args, &stdin).unwrap().len();
                }
            }
        }
    }
}
