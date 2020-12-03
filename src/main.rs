mod util;
mod day1;
mod day2;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "advent of code 2020 solutions")]
enum AdventOfCode {
    #[structopt(name = "day1a")]
    Day1A(day1::Day1A),
    #[structopt(name = "day1b")]
    Day1B(day1::Day1B),
    #[structopt(name = "day2a")]
    Day2A(day2::Day2A),
}

fn main() -> anyhow::Result<()> {
    let opts = AdventOfCode::from_args();

    use AdventOfCode::*;
    match opts {
        Day1A(args) => day1::day1a(args),
        Day1B(args) => day1::day1b(args),
        Day2A(args) => day2::day2a(args),
    }
}
