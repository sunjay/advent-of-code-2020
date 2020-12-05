use std::{collections::HashMap, iter, path::PathBuf};

use structopt::StructOpt;
use once_cell::sync::Lazy;
use regex::Regex;
use anyhow::{Context, bail, ensure};

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

#[derive(Debug, StructOpt)]
pub struct Day4B {
    /// The input file to read
    #[structopt(default_value = "input/day4")]
    input: PathBuf,
}

pub fn day4b(args: Day4B) -> anyhow::Result<()> {
    let Day4B {input} = args;

    let mut valid = 0;
    let mut total = 1;

    let mut fields = HashMap::new();

    // Need an extra newline so that last record is processed
    let extra_newline = iter::once(Ok(String::new()));
    for line in util::read_lines(input)?.chain(extra_newline) {
        let line: String = line?;

        if line.is_empty() {
            if validate_fields(&fields).is_ok() {
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

fn validate_fields(fields: &HashMap<String, String>) -> anyhow::Result<()> {
    // Birth Year
    let byr = fields.get("byr").context("missing field `byr`")?;
    let byr: u64 = byr.parse().context("invalid `byr`")?;
    if byr < 1920 || byr > 2002 {
        bail!("invalid `byr`");
    }

    // Issue Year
    let iyr = fields.get("iyr").context("missing field `iyr`")?;
    let iyr: u64 = iyr.parse().context("invalid `iyr`")?;
    if iyr < 2010 || iyr > 2020 {
        bail!("invalid `iyr`");
    }

    // Expiration Year
    let eyr = fields.get("eyr").context("missing field `eyr`")?;
    let eyr: u64 = eyr.parse().context("invalid `eyr`")?;
    if eyr < 2020 || eyr > 2030 {
        bail!("invalid `eyr`");
    }

    // Height
    let hgt = fields.get("hgt").context("missing field `hgt`")?;
    static R_HEIGHT: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(\d+)(cm|in)$").unwrap()
    });
    let caps = R_HEIGHT.captures(hgt).context("invalid `hgt`")?;
    let height: u64 = caps[1].parse().context("invalid `hgt` value")?;
    match &caps[2] {
        // regex guarantees that this is either "cm" or "in"
        "cm" if height < 150 || height > 193 => bail!("invalid `hgt` value (cm)"),
        "in" if height < 59 || height > 76 => bail!("invalid `hgt` value (in)"),
        _ => {},
    }

    // Hair Color
    let hcl = fields.get("hcl").context("missing field `hcl`")?;
    ensure!(hcl.as_bytes()[0] == b'#', "invalid `hcl` value");
    let color = u64::from_str_radix(&hcl[1..], 16).context("invalid `hcl` value")?;
    ensure!(color <= 0xfff_fff, "invalid `hcl` value (out of range)");

    // Eye Color
    let ecl = fields.get("ecl").context("missing field `ecl`")?;
    match &**ecl {
        "amb" |
        "blu" |
        "brn" |
        "gry" |
        "grn" |
        "hzl" |
        "oth" => {},

        _ => bail!("invalid `ecl` value `{}`", ecl),
    }

    // Passport ID
    let pid = fields.get("pid").context("missing field `pid`")?;
    if pid.len() != 9 || !pid.chars().all(|ch| ch.is_digit(10)) {
        bail!("invalid `pid` value `{}`", pid);
    }

    Ok(())
}
