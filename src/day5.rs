use std::{fmt, path::PathBuf, str::FromStr};

use structopt::StructOpt;
use anyhow::{Context, ensure};

use crate::util;

const ROWS: usize = 128;
const SEATS_PER_ROW: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Partition {
    /// Selects the lower half/partition of an interval
    Lower,
    /// Selects the upper half/partition of an interval
    Upper,
}

#[derive(Debug, Clone)]
struct SeatSpec {
    pub row_spec: Vec<Partition>,
    pub col_spec: Vec<Partition>,
}

impl fmt::Display for SeatSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {row_spec, col_spec} = self;

        for &part in row_spec {
            use Partition::*;
            match part {
                Lower => write!(f, "F")?,
                Upper => write!(f, "B")?,
            }
        }

        for &part in col_spec {
            use Partition::*;
            match part {
                Lower => write!(f, "L")?,
                Upper => write!(f, "R")?,
            }
        }

        Ok(())
    }
}

impl FromStr for SeatSpec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_spec = Vec::new();
        let mut col_spec = Vec::new();

        for ch in s.chars() {
            match ch {
                'F' => row_spec.push(Partition::Lower),
                'B' => row_spec.push(Partition::Upper),
                'L' => col_spec.push(Partition::Lower),
                'R' => col_spec.push(Partition::Upper),
                _ => unreachable!(),
            }
        }

        Ok(Self {row_spec, col_spec})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Interval {
    /// The start value of the interval (inclusive)
    pub start: usize,
    /// The end value of the interval (inclusive)
    pub end: usize,
}

impl Interval {
    pub fn len(self) -> usize {
        self.end - self.start + 1
    }

    pub fn half(self) -> usize {
        self.start + self.len() / 2
    }

    pub fn lower_half(self) -> Self {
        Self {
            start: self.start,
            end: self.half() - 1,
        }
    }

    pub fn upper_half(self) -> Self {
        Self {
            start: self.half(),
            end: self.end,
        }
    }

    /// Returns the value of this interval by checking that it only represents a single value
    pub fn value(self) -> anyhow::Result<usize> {
        ensure!(self.start == self.end, "interval ({}, {}) did not contain exactly one value", self.start, self.end);

        Ok(self.start)
    }
}

/// Reduces an interval to a single value using the given partitions
fn reduce_interval(mut range: Interval, partitions: &[Partition]) -> anyhow::Result<usize> {
    for &part in partitions {
        use Partition::*;
        match part {
            Lower => range = range.lower_half(),
            Upper => range = range.upper_half(),
        }
    }

    range.value()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Seat {
    pub row: usize,
    pub col: usize,
}

impl Seat {
    pub fn id(self) -> usize {
        self.row * SEATS_PER_ROW + self.col
    }

    pub fn from_spec(spec: &SeatSpec) -> anyhow::Result<Self> {
        let SeatSpec {row_spec, col_spec} = spec;
        let row = reduce_interval(Interval {start: 0, end: ROWS-1}, row_spec)?;
        let col = reduce_interval(Interval {start: 0, end: SEATS_PER_ROW-1}, col_spec)?;
        Ok(Self {row, col})
    }
}

#[derive(Debug, StructOpt)]
pub struct Day5A {
    /// The input file to read
    #[structopt(default_value = "input/day5")]
    input: PathBuf,
}

pub fn day5a(args: Day5A) -> anyhow::Result<()> {
    let Day5A {input} = args;

    let mut max_id = 0;
    for line in util::read_lines(input)? {
        let spec: SeatSpec = line?;
        let seat = Seat::from_spec(&spec)
            .with_context(|| format!("reducing spec `{}`", spec))?;
        max_id = seat.id().max(max_id);
    }

    println!("Maximum ID: {}", max_id);

    Ok(())
}
