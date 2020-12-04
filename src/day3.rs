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

    let trees_found = trees_on_slope(&grid, dx, dy);
    println!("Trees: {}", trees_found);

    Ok(())
}

fn trees_on_slope(grid: &[Row], dx: usize, dy: usize) -> usize {
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

    trees_found
}

#[derive(Debug, StructOpt)]
pub struct Day3B {
    /// The input file to read
    #[structopt(default_value = "input/day3")]
    input: PathBuf,
}

pub fn day3b(args: Day3B) -> anyhow::Result<()> {
    let Day3B {input} = args;

    let grid: Vec<Row> = util::read_lines(input)?.collect::<Result<_, _>>()?;

    let slopes = &[
        // (dx, dy)
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut product = 1;
    for &(dx, dy) in slopes {
        let trees_found = trees_on_slope(&grid, dx, dy);
        println!("dx: {}, dy: {}, trees_found: {}", dx, dy, trees_found);
        product *= trees_found;
    }

    println!();
    println!("Multiplied together: {}", product);

    Ok(())
}
