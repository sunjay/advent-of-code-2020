use std::path::PathBuf;

use structopt::StructOpt;

use crate::util;

#[derive(Debug, StructOpt)]
pub struct Day1Args {
    /// The input file to read
    input: PathBuf,
}

pub fn day1(args: Day1Args) -> anyhow::Result<()> {
    let Day1Args {input} = args;

    let expenses: Vec<i64> = util::read_lines(&input)?;
    dbg!(expenses);

    Ok(())
}
