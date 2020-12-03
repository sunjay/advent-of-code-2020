use std::{collections::HashSet, path::PathBuf, str::FromStr};

use structopt::StructOpt;
use once_cell::sync::Lazy;
use regex::Regex;
use anyhow::Context;

use crate::util;

#[derive(Debug)]
struct InputLine {
    pub min: usize,
    pub max: usize,
    pub target_byte: u8,
    pub password: Vec<u8>,
}

impl FromStr for InputLine {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        static R_LINE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)").unwrap()
        });

        let caps = R_LINE.captures(text).context("line did not match")?;
        let min = caps[1].parse()?;
        let max = caps[2].parse()?;
        let target_byte: char = caps[3].parse()?;
        let target_byte = target_byte as u8;
        let password = caps[4].as_bytes().to_owned();

        Ok(Self {min, max, target_byte, password})
    }
}

#[derive(Debug, StructOpt)]
pub struct Day2A {
    /// The input file to read
    #[structopt(default_value = "input/day2")]
    input: PathBuf,
}

pub fn day2a(args: Day2A) -> anyhow::Result<()> {
    let Day2A {input} = args;

    let mut valid = 0;
    let mut total = 0;

    for line in util::read_lines(input)? {
        let line: InputLine = line?;

        let target_count = line.password.iter()
            .filter(|&&byte| byte == line.target_byte)
            .count();

        if target_count >= line.min && target_count <= line.max {
            valid += 1;
        }

        total += 1;
    }

    println!("{} valid / {} total", valid, total);

    Ok(())
}
