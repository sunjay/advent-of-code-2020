mod util;
mod day1;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "advent of code 2020 solutions")]
enum AdventOfCode {
    #[structopt(name = "day1a")]
    Day1A(day1::Day1A),
}

fn main() -> anyhow::Result<()> {
    let opts = AdventOfCode::from_args();

    use AdventOfCode::*;
    match opts {
        Day1A(args) => day1::day1a(args),
    }
}
