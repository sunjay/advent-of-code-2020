use std::{collections::HashSet, path::PathBuf};

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
