use std::{collections::{HashMap, HashSet}, iter, path::PathBuf};

use structopt::StructOpt;

use crate::util;

#[derive(Debug, StructOpt)]
pub struct Day6A {
    /// The input file to read
    #[structopt(default_value = "input/day6")]
    input: PathBuf,
}

pub fn day6a(args: Day6A) -> anyhow::Result<()> {
    let Day6A {input} = args;

    let mut yes_count = 0;
    let mut group = HashSet::new();
    for line in util::read_lines(input)? {
        let line: String = line?;

        if line.is_empty() {
            group.clear();
            continue;
        }

        for ch in line.chars() {
            if !group.contains(&ch) {
                yes_count += 1;
                group.insert(ch);
            }
        }
    }

    println!("Yes: {}", yes_count);

    Ok(())
}

#[derive(Debug, StructOpt)]
pub struct Day6B {
    /// The input file to read
    #[structopt(default_value = "input/day6")]
    input: PathBuf,
}

pub fn day6b(args: Day6B) -> anyhow::Result<()> {
    let Day6B {input} = args;

    let mut total_everyone_yes = 0usize;
    let mut group_size = 0;
    let mut group_yes: HashMap<_, usize> = HashMap::new();
    let extra_newline = iter::once(Ok(String::new()));
    for line in util::read_lines(input)?.chain(extra_newline) {
        let line: String = line?;

        if line.is_empty() {
            // Find the questions to which everyone in the group said yes
            for (_, &yes_count) in &group_yes {
                if yes_count == group_size {
                    total_everyone_yes += 1;
                }
            }

            group_size = 0;
            group_yes.clear();
            continue;
        }

        group_size += 1;

        for ch in line.chars() {
            *group_yes.entry(ch).or_default() += 1;
        }
    }

    println!("Total: {}", total_everyone_yes);

    Ok(())
}
