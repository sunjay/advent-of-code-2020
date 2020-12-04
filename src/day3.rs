use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

use crate::util;

#[derive(Debug)]
struct Row {
    /// true if the column contains a tree
    pub trees: Vec<bool>,
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            trees: line.bytes().map(|byte| match byte {
                b'.' => false,
                b'#' => true,
                _ => unreachable!(),
            }).collect(),
        })
    }
}

#[derive(Debug, StructOpt)]
pub struct Day3A {
    /// The input file to read
    #[structopt(default_value = "input/day3")]
    input: PathBuf,
    /// The amount to move right in the slope
    #[structopt(long, default_value = "3")]
    dx: usize,
    /// The amount to move down in the slope
    #[structopt(long, default_value = "1")]
    dy: usize,
}

pub fn day3a(args: Day3A) -> anyhow::Result<()> {
    let Day3A {input, dx, dy} = args;

    let grid: Vec<Row> = util::read_lines(input)?.collect::<Result<_, _>>()?;

    let mut current_row= 0;
    let mut current_col = 0;

    let mut trees_found = 0;
    while let Some(row) = grid.get(current_row) {
        if row.trees[current_col] {
            trees_found += 1;
        }

        current_row += dy;
        current_col = (current_col + dx) % row.trees.len();
    }

    println!("Trees: {}", trees_found);

    Ok(())
}
