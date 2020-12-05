mod util;
mod day1;
mod day2;
mod day3;
mod day4;

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
    #[structopt(name = "day2b")]
    Day2B(day2::Day2B),
    #[structopt(name = "day3a")]
    Day3A(day3::Day3A),
    #[structopt(name = "day3b")]
    Day3B(day3::Day3B),
    #[structopt(name = "day4a")]
    Day4A(day4::Day4A),
}

fn main() -> anyhow::Result<()> {
    let opts = AdventOfCode::from_args();

    use AdventOfCode::*;
    match opts {
        Day1A(args) => day1::day1a(args),
        Day1B(args) => day1::day1b(args),
        Day2A(args) => day2::day2a(args),
        Day2B(args) => day2::day2b(args),
        Day3A(args) => day3::day3a(args),
        Day3B(args) => day3::day3b(args),
        Day4A(args) => day4::day4a(args),
    }
}
