use std::io;
use std::io::Read;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    day: u32,
    part: u32,
}

fn helper() -> Result<String, Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    match (args.day, args.part) {
        (1, 1) => Ok(format!("{}", aoclib::day_1::part_1(&text))),
        (1, 2) => Ok(format!("{}", aoclib::day_1::part_2(&text))),
        (2, 1) => Ok(format!("{}", aoclib::day_2::part_1(&text))),
        (2, 2) => Ok(format!("{}", aoclib::day_2::part_2(&text))),
        (3, 1) => Ok(format!("{}", aoclib::day_3::part_1(&text))),
        (3, 2) => Ok(format!("{}", aoclib::day_3::part_2(&text))),
        (4, 1) => Ok(format!("{}", aoclib::day_4::part_1(&text))),
        (4, 2) => Ok(format!("{}", aoclib::day_4::part_2(&text))),
        (5, 1) => Ok(format!("{}", aoclib::day_5::part_1(&text))),
        (5, 2) => Ok(format!("{}", aoclib::day_5::part_2(&text))),
        (6, 1) => Ok(format!("{}", aoclib::day_6::part_1(&text))),
        (6, 2) => Ok(format!("{}", aoclib::day_6::part_2(&text))),
        (_, 1 | 2) => Err("Invalid day".into()),
        (1..=6, _) => Err("Invalid part".into()),
        _ => Err("Invalid day and part".into()),
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match helper() {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(error) => Err(error),
    }
}
