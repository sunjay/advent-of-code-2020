use std::{collections::HashMap, iter, path::PathBuf};

use structopt::StructOpt;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::util;

#[derive(Debug, StructOpt)]
pub struct Day4A {
    /// The input file to read
    #[structopt(default_value = "input/day4")]
    input: PathBuf,
}

pub fn day4a(args: Day4A) -> anyhow::Result<()> {
    let Day4A {input} = args;

    let mut valid = 0;
    let mut total = 1;

    let mut fields = HashMap::new();

    // Need an extra newline so that last record is processed
    let extra_newline = iter::once(Ok(String::new()));
    for line in util::read_lines(input)?.chain(extra_newline) {
        let line: String = line?;

        if line.is_empty() {
            let expected_fields = &[
                "byr", // Birth Year
                "iyr", // Issue Year
                "eyr", // Expiration Year
                "hgt", // Height
                "hcl", // Hair Color
                "ecl", // Eye Color
                "pid", // Passport ID
                // Optional:
                //"cid", // Country ID
            ];

            if expected_fields.iter().all(|&field| fields.contains_key(field)) {
                valid += 1;
            }

            total += 1;
            fields.clear();

            continue;
        }

        static R_PAIR: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"([^: ]+):([^ \n]+)").unwrap()
        });

        fields.extend(R_PAIR.captures_iter(&line).map(|caps| {
            (caps[1].to_string(), caps[2].to_string())
        }));
    }

    println!("{} valid / {} total", valid, total);

    Ok(())
}
