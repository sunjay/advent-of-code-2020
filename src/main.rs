mod util;
mod day1;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "advent of code 2020 solutions")]
enum AdventOfCode {
    #[structopt(name = "day1a")]
    Day1A(day1::Day1A),
    #[structopt(name = "day1b")]
    Day1B(day1::Day1B),
}

fn main() -> anyhow::Result<()> {
    let opts = AdventOfCode::from_args();

    use AdventOfCode::*;
    match opts {
        Day1A(args) => day1::day1a(args),
        Day1B(args) => day1::day1b(args),
    }
}
