use std::{collections::HashSet, path::PathBuf};

use structopt::StructOpt;

use crate::util;

#[derive(Debug, StructOpt)]
pub struct Day1A {
    /// The input file to read
    #[structopt(default_value = "input/day1")]
    input: PathBuf,
    /// The target value that two expenses must sum to
    #[structopt(default_value = "2020")]
    target: i64,
}

/// This problem is essentially Two Sum
pub fn day1a(args: Day1A) -> anyhow::Result<()> {
    let Day1A {input, target} = args;

    let mut seen = HashSet::new();

    for expense in util::read_lines(input)? {
        let expense: i64 = expense?;

        // The other value that would be needed to add up to target
        let other = target - expense;
        if seen.contains(&other) {
            println!("{} + {} = {}", expense, other, expense + other);
            println!("{} * {} = {}", expense, other, expense * other);
            break;
        }
        seen.insert(expense);
    }

    Ok(())
}
