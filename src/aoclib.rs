use std::collections::{BTreeMap, HashMap};

use structopt::StructOpt;

#[cfg(test)]
mod testing;
mod y2018;
mod y2020;
mod y2021;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long)]
    year: Option<u32>,
    #[structopt(long)]
    day: Option<u32>,
    #[structopt(long)]
    part: Option<u32>,
}

pub type AnyError = Box<dyn std::error::Error>;
type Solver = dyn Fn(&str) -> Result<String, AnyError>;

fn _candidates(args: &Cli) -> Result<BTreeMap<(u32, u32, u32), &Solver>, AnyError> {
    let mut functions: HashMap<(u32, u32, u32), &Solver> = HashMap::new();
    functions.insert((2018, 1, 1), &y2018::d01::part_1);
    functions.insert((2018, 1, 2), &y2018::d01::part_2);
    functions.insert((2020, 1, 1), &y2020::d01::part_1);
    functions.insert((2020, 1, 2), &y2020::d01::part_2);
    functions.insert((2020, 2, 1), &y2020::d02::part_1);
    functions.insert((2020, 2, 2), &y2020::d02::part_2);
    functions.insert((2021, 1, 1), &y2021::d01::part_1);
    functions.insert((2021, 1, 2), &y2021::d01::part_2);
    functions.insert((2021, 2, 1), &y2021::d02::part_1);
    functions.insert((2021, 2, 2), &y2021::d02::part_2);
    functions.insert((2021, 3, 1), &y2021::d03::part_1);
    functions.insert((2021, 3, 2), &y2021::d03::part_2);
    functions.insert((2021, 4, 1), &y2021::d04::part_1);
    functions.insert((2021, 4, 2), &y2021::d04::part_2);
    functions.insert((2021, 5, 1), &y2021::d05::part_1);
    functions.insert((2021, 5, 2), &y2021::d05::part_2);
    functions.insert((2021, 6, 1), &y2021::d06::part_1);
    functions.insert((2021, 6, 2), &y2021::d06::part_2);
    functions.insert((2021, 7, 1), &y2021::d07::part_1);
    functions.insert((2021, 7, 2), &y2021::d07::part_2);
    functions.insert((2021, 8, 1), &y2021::d08::part_1);
    functions.insert((2021, 8, 2), &y2021::d08::part_2);
    functions.insert((2021, 20, 1), &y2021::d20::part_1);
    functions.insert((2021, 20, 2), &y2021::d20::part_2);
    functions.insert((2021, 21, 1), &y2021::d21::part_1);
    functions.insert((2021, 21, 2), &y2021::d21::part_2);
    functions.insert((2021, 22, 1), &y2021::d22::part_1);
    functions.insert((2021, 22, 2), &y2021::d22::part_2);
    functions.insert((2021, 23, 1), &y2021::d23::part_1);
    functions.insert((2021, 23, 2), &y2021::d23::part_2);
    functions.insert((2021, 24, 1), &y2021::d24::part_1);
    functions.insert((2021, 24, 2), &y2021::d24::part_2);
    functions.insert((2021, 25, 1), &y2021::d25::part_1);
    let functions = functions;

    let mut result = BTreeMap::new();
    let parts: Vec<u32> = match args.part {
        None => 1..=2,
        Some(1) => 1..=1,
        Some(2) => 2..=2,
        _ => return Err("Invalid part".into()),
    }
    .collect();

    let days: Vec<u32> = match args.day {
        None => 1..=25,
        Some(day) => day..=day,
    }
    .collect();

    let years: Vec<u32> = match args.year {
        None => 2020..=2021,
        Some(year) => year..=year,
    }
    .collect();

    for year in years {
        for &day in &days {
            for &part in &parts {
                let key = (year, day, part);
                if let Some(&func) = functions.get(&key) {
                    result.insert(key, func);
                }
            }
        }
    }

    if result.is_empty() {
        return Err("No matching candidates".into());
    }

    Ok(result)
}

pub fn helper(args: &Cli, text: &str) -> Result<Vec<String>, AnyError> {
    let candidates: BTreeMap<(u32, u32, u32), &Solver> = _candidates(args)?;
    if candidates.is_empty() {
        return Err("Invalid day and part".into());
    }

    let mut result = Vec::new();
    for ((year, day, part), func) in candidates.iter() {
        log::debug!("Trying year {} day {} part {}", year, day, part);
        match func(text) {
            Ok(output) => result.push(output),
            Err(error) => match args.day {
                None => log::debug!("{}", error),
                Some(_) => return Err(error),
            },
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const NUM_DAY: u32 = 7;
    const STEMS: [&str; 2] = ["example", "input"];

    #[test]
    fn specific_day_returns_2_solutions_on_right_input() {
        for day in 1..=NUM_DAY {
            let args = Cli {
                year: Some(2021),
                day: Some(day),
                part: None,
            };
            let input_day = day;
            for stem in STEMS {
                let stdin =
                    fs::read_to_string(format!("inputs/y2021/d{:02}/{}.txt", input_day, stem))
                        .unwrap();
                println!("day:{} input_day:{} stem:{}", day, input_day, stem);
                assert_eq!(helper(&args, &stdin).unwrap().len(), 2);
            }
        }
    }

    #[test]
    #[ignore]
    fn specific_day_returns_error_on_wrong_input() {
        for day in 1..=NUM_DAY {
            let args = Cli {
                year: Some(2021),
                day: Some(day),
                part: None,
            };
            for input_day in (1..=NUM_DAY).filter(|d| *d != day) {
                for stem in STEMS {
                    let stdin =
                        fs::read_to_string(format!("inputs/y2021/d{:02}/{}.txt", input_day, stem))
                            .unwrap();
                    println!("day:{} input_day:{} stem:{}", day, input_day, stem);
                    helper(&args, &stdin).unwrap().len();
                }
            }
        }
    }
}
