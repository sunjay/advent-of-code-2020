mod util;
mod day1;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "advent of code 2020 solutions")]
enum AdventOfCode {
    Day1(day1::Day1Args),
}

fn main() -> anyhow::Result<()> {
    let opts = AdventOfCode::from_args();

    use AdventOfCode::*;
    match opts {
        Day1(args) => day1::day1(args),
    }
}
