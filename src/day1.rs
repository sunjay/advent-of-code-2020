use std::path::PathBuf;

use structopt::StructOpt;

use crate::util;

#[derive(Debug, StructOpt)]
pub struct Day1Args {
    /// The input file to read
    input: PathBuf,
    /// The target value that two expenses must sum to
    #[structopt(default_value = "2020")]
    target: i64,
}

pub fn day1(args: Day1Args) -> anyhow::Result<()> {
    let Day1Args {input, target} = args;

    let expenses: Vec<i64> = util::read_lines(input)?.collect::<anyhow::Result<Vec<_>>>()?;

    for &expense1 in &expenses {
        for &expense2 in &expenses {
            if expense1 + expense2 == target {
                println!("{} + {} = {}", expense1, expense2, expense1 + expense2);
                println!("{} * {} = {}", expense1, expense2, expense1 * expense2);
            }
        }
    }

    Ok(())
}
