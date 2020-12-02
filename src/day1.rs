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

#[derive(Debug, StructOpt)]
pub struct Day1B {
    /// The input file to read
    #[structopt(default_value = "input/day1")]
    input: PathBuf,
    /// The target value that three expenses must sum to
    #[structopt(default_value = "2020")]
    target: i64,
}

/// This problem is essentially Three Sum
pub fn day1b(args: Day1B) -> anyhow::Result<()> {
    let Day1B {input, target} = args;

    let expenses: Vec<i64> = util::read_lines(input)?.collect::<anyhow::Result<_>>()?;
    for (i, expense) in expenses.iter().enumerate() {
        let rest = &expenses[i+1..];
        let remaining = target - expense;

        if let Some((other1, other2)) = two_sum(rest, remaining) {
            println!("{} + {} + {} = {}", expense, other1, other2, expense + other1 + other2);
            println!("{} * {} * {} = {}", expense, other1, other2, expense * other1 * other2);
        }
    }

    Ok(())
}

fn two_sum(numbers: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut seen = HashSet::new();

    for &value in numbers {
        // The other value that would be needed to add up to target
        let other = target - value;
        if seen.contains(&other) {
            return Some((value, other));
        }
        seen.insert(value);
    }

    None
}
