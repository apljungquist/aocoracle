use std::io;
use std::io::Read;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let args = aoclib::Cli::from_args();

    match aoclib::helper_text(&args, &text) {
        Ok(lines) => {
            lines.iter().for_each(|line| println!("{}", line));
            Ok(())
        }
        Err(error) => Err(error),
    }
}
